// Verify the deployed Agent's Ladder contract on testnet (repeatable).
// Covers: answer-key authority (the P0 fix), faucet, start_run with
// bank question IDs, answer (correct/wrong), walk_away, lose-everything,
// 60-second timeout, 50:50, audience, phone_friend (real Genesis agent),
// win_run, leaderboard pagination, season prize budget views, NEP-245.
//
// The contract under test must have its answer key populated
// (see populate-answer-key.js --submit). Question sets are picked from
// the real bank files with a per-run offset so replay protection never
// trips inside one run of this script.
//
// Usage:
//   node verify.js [contract-account]
//   (default contract: ladder-5a84ua.testnet)
const fs = require('fs');
const path = require('path');
const { connect, keyStores, KeyPair, utils } = require('near-api-js');

const CONTRACT = process.argv[2] || 'ladder-5a84ua.testnet';
const CONTRACT_DIR = __dirname;
const KEYS_PATH = CONTRACT_DIR + '/.testnet-accounts.json';
const QUESTIONS_DIR = path.join(__dirname, '..', 'questions');
const HELPER_URL = 'https://helper.testnet.near.org/account';
const LADDER = [10, 20, 30, 50, 80, 120, 180, 250, 350, 500, 700, 1000, 1500, 2500, 5000];

const BANDS = [
  { file: 'bank-easy.json', offset: 0, count: 2 },
  { file: 'bank-medium.json', offset: 300, count: 3 },
  { file: 'bank-hard.json', offset: 600, count: 5 },
  { file: 'bank-veryhard.json', offset: 900, count: 5 },
];
const BANKS = BANDS.map((b) => JSON.parse(fs.readFileSync(path.join(QUESTIONS_DIR, b.file), 'utf8')));

// A 15-question set with known correct answers, offset `n` inside each
// band so every run in this script uses a fresh set (replay protection).
// BASE is random per script run so the script is repeatable against the
// same contract: replay protection is global, and a set used by a
// previous run of this script must not be reused.
const BASE = Math.floor(Math.random() * 200);
function questionSet(n) {
  const m = BASE + n;
  const ids = [];
  const correct = [];
  BANDS.forEach((b, bi) => {
    for (let i = 0; i < b.count; i++) {
      const idx = m + i;
      ids.push(b.offset + idx);
      correct.push(BANKS[bi][idx].c);
    }
  });
  return { ids, correct };
}
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function createAccount(accountId, publicKey) {
  const res = await fetch(HELPER_URL, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ newAccountId: accountId, newAccountPublicKey: publicKey }),
  });
  if (!res.ok) throw new Error(`helper ${res.status}: ${(await res.text()).slice(0, 200)}`);
  return res.json();
}

let failures = 0;
function check(name, cond, detail) {
  console.log(`${cond ? 'PASS' : 'FAIL'}: ${name}${detail ? ' — ' + detail : ''}`);
  if (!cond) failures++;
}
async function expectFail(name, fn, needle) {
  try {
    await fn();
    check(name, false, 'call succeeded, expected failure');
  } catch (e) {
    const msg = e.message || String(e);
    check(name, !needle || msg.includes(needle), 'failed as expected: ' + msg.slice(0, 120));
  }
}

(async () => {
  const keys = JSON.parse(fs.readFileSync(KEYS_PATH, 'utf8'));
  const deployKey = KeyPair.fromString(keys[CONTRACT].private_key);
  const keyStore = new keyStores.InMemoryKeyStore();
  await keyStore.setKey('testnet', CONTRACT, deployKey);

  const playerId = `ladder-player-${Math.random().toString(36).slice(2, 8)}.testnet`;
  const playerKey = KeyPair.fromRandom('ed25519');
  await createAccount(playerId, playerKey.getPublicKey().toString());
  await keyStore.setKey('testnet', playerId, playerKey);
  keys[playerId] = { account_id: playerId, public_key: playerKey.getPublicKey().toString(), private_key: playerKey.toString() };
  fs.writeFileSync(KEYS_PATH, JSON.stringify(keys, null, 2));
  fs.chmodSync(KEYS_PATH, 0o600);
  console.log('Contract:', CONTRACT);
  console.log('Player account:', playerId);

  const config = {
    networkId: 'testnet', keyStore,
    nodeUrl: 'https://rpc.testnet.fastnear.com',
    walletUrl: 'https://testnet.mynearwallet.com',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://testnet.nearblocks.io',
  };
  const near = await connect(config);
  const player = await near.account(playerId);
  const GAS = BigInt('100000000000000'); // 100 TGas

  async function call(method, args, deposit = '0') {
    return player.functionCall({
      contractId: CONTRACT, methodName: method, args,
      gas: GAS, attachedDeposit: BigInt(utils.format.parseNearAmount(deposit)),
    });
  }
  async function callOk(method, args) {
    const r = await call(method, args);
    const v = r.status.SuccessValue;
    return v ? JSON.parse(Buffer.from(v, 'base64').toString()) : null;
  }
  async function view(method, args) {
    return player.viewFunction({ contractId: CONTRACT, methodName: method, args });
  }
  async function bal() {
    return Number(await view('mt_balance_of', { account_id: playerId, token_id: 'ladder' }));
  }
  // Read-after-write can lag: a view call may hit a node that has not yet
  // processed the last transaction. Poll the balance until it equals the
  // expected value (or the timeout expires) instead of trusting one read.
  async function waitForBal(expected, tries = 12) {
    let b = await bal();
    for (let i = 0; i < tries && b !== expected; i++) {
      await sleep(1000);
      b = await bal();
    }
    return b;
  }

  console.log('\n--- 0. ANSWER KEY (P0 fix) ---');
  const version = await view('get_answer_key_version', {});
  console.log('Answer-key version:', version);
  check('answer key is populated (version > 0)', Number(version) > 0);
  const q0 = questionSet(0);
  const hash0 = await view('get_question_hash', { question_id: q0.ids[0] });
  check('stored hash exists for a bank question', typeof hash0 === 'string' && hash0.length === 64);
  const noHash = await view('get_question_hash', { question_id: 1199 });
  console.log('Hash for qid 1199:', typeof noHash === 'string' ? 'present' : 'absent (key may be partial)');
  const budget = await view('get_season_prize_budget', {});
  console.log('Season prize budget:', budget, '| prizes paid so far:', await view('get_season_prizes_paid', {}));

  console.log('\n--- 1. FAUCET ---');
  await call('claim_faucet', {});
  check('faucet grants 500', (await bal()) === 500);
  await expectFail('second faucet claim fails', () => call('claim_faucet', {}), 'already claimed');

  console.log('\n--- 2. FORGE TESTS (P0 regression) ---');
  // (a) start_run with a question that has no stored hash must fail.
  // 9999 is outside the bank (0..1199), so it has no stored hash.
  await expectFail('start_run with an unkeyed question fails', () => call('start_run', { question_ids: [9999, 1, 300, 301, 302, 600, 601, 602, 603, 604, 900, 901, 902, 903, 904] }), 'no stored answer hash');
  // (b) a wrong answer can never count as correct.
  const s1 = questionSet(0);
  await call('start_run', { question_ids: s1.ids });
  check('start_run with bank question ids ok, balance 400 after stake', (await waitForBal(400)) === 400);
  const wrong0 = (s1.correct[0] + 1) % 4;
  const loseRes = await callOk('answer', { question_id: 0, answer: wrong0 });
  check('wrong answer loses everything', loseRes && loseRes.outcome === 'wrong_lost_everything' && loseRes.pot === '0', JSON.stringify(loseRes));
  check('balance still 400 after loss (stake lost)', (await bal()) === 400);
  // (c) replaying the exact same question set must fail.
  await expectFail('replaying the same question set fails', () => call('start_run', { question_ids: s1.ids }), 'already used');

  console.log('\n--- 3. START RUN + ANSWER CORRECT TO Q5 + WALK AWAY ---');
  const s2 = questionSet(10);
  await call('start_run', { question_ids: s2.ids });
  check('second run starts, balance 300 after stake', (await waitForBal(300)) === 300);
  for (let q = 0; q < 5; q++) {
    const r = await callOk('answer', { question_id: q, answer: s2.correct[q] });
    check(`answer Q${q + 1} correct (level ${r.level}, pot ${r.pot})`, r.correct && r.outcome === 'correct');
  }
  const walkAmt = await callOk('walk_away', {});
  check('walk_away banks 180 (80 pot + 100 stake)', walkAmt === '180', 'banked ' + walkAmt);
  check('balance 480 after walk away', (await bal()) === 480);

  console.log('\n--- 4. LIFELINES (50:50 + audience, no correct_answer arg) ---');
  const s3 = questionSet(20);
  await call('start_run', { question_ids: s3.ids });
  const removed = await callOk('use_5050', {});
  check('use_5050 removes 2 wrong answers', Array.isArray(removed) && removed.length === 2 && !removed.includes(s3.correct[0]), JSON.stringify(removed));
  const votes = await callOk('use_audience', {});
  const voteSum = Array.isArray(votes) ? votes.reduce((a, b) => a + b, 0) : 0;
  check('use_audience votes sum to 100, correct is largest', voteSum === 100 && votes[s3.correct[0]] === Math.max(...votes), JSON.stringify(votes));
  await expectFail('second 50:50 fails', () => call('use_5050', {}), 'already used');

  console.log('\n--- 5. PHONE A FRIEND (real Genesis agent genesis-visitor-1) ---');
  const ownerBalBefore = Number(await view('mt_balance_of', { account_id: 'otoyo4real.testnet', token_id: 'ladder' }));
  const pfAns = await callOk('phone_friend', { token_id: 'genesis-visitor-1', question_id: 0 });
  check('phone_friend returns an answer 0..3 or null', pfAns === null || (pfAns >= 0 && pfAns <= 3), 'answer ' + pfAns);
  // The 40-coin owner payment lands in the cross-contract callback, which
  // executes in a later block than the phone_friend call itself, so poll
  // for the balance change instead of reading it immediately (an immediate
  // read can race the callback and see the old balance).
  let ownerBalAfter = ownerBalBefore;
  for (let i = 0; i < 15; i++) {
    await sleep(1000);
    ownerBalAfter = Number(await view('mt_balance_of', { account_id: 'otoyo4real.testnet', token_id: 'ladder' }));
    if (ownerBalAfter === ownerBalBefore + 40) break;
  }
  check('agent owner paid +40', ownerBalAfter === ownerBalBefore + 40, `${ownerBalBefore} -> ${ownerBalAfter}`);
  // Finish this run by answering all 15 correctly, then win.
  const a1 = await callOk('answer', { question_id: 0, answer: s3.correct[0] });
  check('answer Q1 after phone ok', a1.correct);
  for (let q = 1; q < 15; q++) {
    const r = await callOk('answer', { question_id: q, answer: s3.correct[q] });
    if (q === 14) check('answer Q15 -> won_call_win_run', r.outcome === 'won_call_win_run');
  }
  const winAmt = await callOk('win_run', {});
  check('win_run pays 5000', winAmt === '5000', 'paid ' + winAmt);

  console.log('\n--- 6. LEADERBOARD PAGINATION ---');
  const lbAll = await view('get_leaderboard', {});
  check('get_leaderboard({}) returns rows', Array.isArray(lbAll) && lbAll.length >= 1, lbAll.length + ' rows');
  const lbTop1 = await view('get_leaderboard', { from_index: 0, limit: 1 });
  check('get_leaderboard limit 1 returns exactly 1 row', Array.isArray(lbTop1) && lbTop1.length === 1);
  // from_index 1: if there are 2+ rows it returns the second row,
  // otherwise it correctly returns an empty page.
  const lbSkip = await view('get_leaderboard', { from_index: 1, limit: 1 });
  if (lbAll.length >= 2) {
    check('get_leaderboard from_index 1 returns the second row', Array.isArray(lbSkip) && lbSkip.length === 1 && lbSkip[0].account_id !== lbTop1[0].account_id);
  } else {
    check('get_leaderboard from_index 1 is empty when only 1 row exists', Array.isArray(lbSkip) && lbSkip.length === 0);
  }
  const lbEmpty = await view('get_leaderboard', { from_index: 10000, limit: 5 });
  check('get_leaderboard far beyond the end is empty', Array.isArray(lbEmpty) && lbEmpty.length === 0);
  const stats = await view('get_player_stats', { account_id: playerId });
  console.log('Player stats:', JSON.stringify(stats));
  const supply = await view('mt_total_supply', { token_id: 'ladder' });
  console.log('Total supply:', supply);

  console.log('\n--- 7. 60-SECOND TIMEOUT ---');
  const s4 = questionSet(30);
  await call('start_run', { question_ids: s4.ids });
  await callOk('answer', { question_id: 0, answer: s4.correct[0] });
  const balBeforeTimeout = await bal();
  console.log('Run started, Q1 answered. Balance:', balBeforeTimeout, '. Waiting 65s...');
  await sleep(65000);
  const toRes = await callOk('timeout', { account_id: playerId });
  check('timeout() -> timeout_lost_everything', toRes === 'timeout_lost_everything', String(toRes));
  check('balance unchanged after timeout (stake lost)', (await bal()) === balBeforeTimeout);

  console.log('\n--- DONE ---');
  console.log('Final player balance:', await bal());
  console.log('Final total supply:', await view('mt_total_supply', { token_id: 'ladder' }));
  if (failures > 0) {
    console.error(`VERIFY FAILED: ${failures} check(s) failed`);
    process.exit(1);
  }
  console.log('VERIFY PASSED: all checks green');
})().catch((e) => {
  console.error('VERIFY FAILED:', e);
  process.exit(1);
});
