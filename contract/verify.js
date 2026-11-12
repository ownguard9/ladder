// Verify the deployed Agent's Ladder contract on testnet.
// Covers: faucet, start_run, answer (correct/wrong), walk_away,
// lose-everything, 60-second timeout, 50:50, audience, phone_friend
// (real Genesis agent), win_run, leaderboard, evolve, NEP-245 views.
const fs = require('fs');
const crypto = require('crypto');
const { connect, keyStores, KeyPair, utils } = require('/home/hatch/workspace/merit/node_modules/near-api-js');

const CONTRACT = 'ladder-5a84ua.testnet';
const CONTRACT_DIR = '/home/hatch/workspace/millionaire/contract';
const KEYS_PATH = CONTRACT_DIR + '/.testnet-accounts.json';
const HELPER_URL = 'https://helper.testnet.near.org/account';

const SALT = 'ladder-verify-' + Math.random().toString(36).slice(2, 8);
// Correct answers for questions 0..14 (cycling 0,1,2,3).
const CORRECT = Array.from({ length: 15 }, (_, i) => i % 4);
const LADDER = [10, 20, 30, 50, 80, 120, 180, 250, 350, 500, 700, 1000, 1500, 2500, 5000];

function qhash(qid, correct) {
  return crypto.createHash('sha256').update(`${qid}:${correct}:${SALT}`).digest('hex');
}
function hashes() {
  return CORRECT.map((c, qid) => qhash(qid, c));
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

(async () => {
  const keys = JSON.parse(fs.readFileSync(KEYS_PATH, 'utf8'));
  const deployKey = KeyPair.fromString(keys[CONTRACT].private_key);
  const keyStore = new keyStores.InMemoryKeyStore();
  await keyStore.setKey('testnet', CONTRACT, deployKey);

  // Fresh player account.
  const playerId = `ladder-player-${Math.random().toString(36).slice(2, 8)}.testnet`;
  const playerKey = KeyPair.fromRandom('ed25519');
  await createAccount(playerId, playerKey.getPublicKey().toString());
  await keyStore.setKey('testnet', playerId, playerKey);
  keys[playerId] = { account_id: playerId, public_key: playerKey.getPublicKey().toString(), private_key: playerKey.toString() };
  fs.writeFileSync(KEYS_PATH, JSON.stringify(keys, null, 2));
  fs.chmodSync(KEYS_PATH, 0o600);
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
  async function view(method, args) {
    return player.viewFunction({ contractId: CONTRACT, methodName: method, args });
  }
  async function bal() {
    return Number(await view('mt_balance_of', { account_id: playerId, token_id: 'ladder' }));
  }

  console.log('\n--- 1. FAUCET ---');
  const f1 = await call('claim_faucet', {});
  console.log('claim_faucet tx ok. Balance now:', await bal(), '(expect 500)');
  try {
    await call('claim_faucet', {});
    console.log('ERROR: second faucet claim should have failed');
  } catch (e) {
    console.log('Second claim correctly failed:', e.message.slice(0, 120));
  }

  console.log('\n--- 2. START RUN + ANSWER CORRECT TO Q5 + WALK AWAY ---');
  const run1 = await view('get_run_view', { account_id: playerId });
  console.log('Run before start (expect null):', run1);
  await call('start_run', { question_hashes: hashes(), salt: SALT });
  console.log('start_run ok. Balance now:', await bal(), '(expect 400 after 100 stake)');
  for (let q = 0; q < 5; q++) {
    const r = await call('answer', { question_id: q, answer: CORRECT[q], correct_answer: CORRECT[q] });
    const res = JSON.parse(Buffer.from(r.status.SuccessValue, 'base64').toString());
    console.log(`answer Q${q + 1} correct -> level ${res.level}, pot ${res.pot}, outcome ${res.outcome}`);
  }
  const walk = await call('walk_away', {});
  const walkAmt = JSON.parse(Buffer.from(walk.status.SuccessValue, 'base64').toString());
  console.log('walk_away banked:', walkAmt, '(expect "580" = 80 pot + 100 stake)');
  console.log('Balance after walk away:', await bal(), '(expect 980)');

  console.log('\n--- 3. LOSE EVERYTHING (wrong answer at Q6) ---');
  await call('start_run', { question_hashes: hashes(), salt: SALT });
  console.log('start_run ok. Balance now:', await bal(), '(expect 880 after 100 stake)');
  for (let q = 0; q < 5; q++) {
    await call('answer', { question_id: q, answer: CORRECT[q], correct_answer: CORRECT[q] });
  }
  const wrong = (CORRECT[5] + 1) % 4;
  const lose = await call('answer', { question_id: 5, answer: wrong, correct_answer: CORRECT[5] });
  const loseRes = JSON.parse(Buffer.from(lose.status.SuccessValue, 'base64').toString());
  console.log('answer Q6 wrong ->', JSON.stringify(loseRes), '(expect wrong_lost_everything, pot "0")');
  console.log('Balance after loss:', await bal(), '(expect 880 — stake lost, nothing banked)');
  const runAfterLoss = await view('get_run_view', { account_id: playerId });
  console.log('Run after loss (expect null):', runAfterLoss);

  console.log('\n--- 4. LIFELINES (50:50 + audience) ---');
  await call('start_run', { question_hashes: hashes(), salt: SALT });
  console.log('start_run ok. Balance now:', await bal(), '(expect 780)');
  const ff = await call('use_5050', { correct_answer: CORRECT[0] });
  const removed = JSON.parse(Buffer.from(ff.status.SuccessValue, 'base64').toString());
  console.log('use_5050 removed:', removed, '(expect 2 indices, neither is correct', CORRECT[0], ')');
  console.log('Balance after 50:50:', await bal(), '(expect 760)');
  const aud = await call('use_audience', { correct_answer: CORRECT[0] });
  const votes = JSON.parse(Buffer.from(aud.status.SuccessValue, 'base64').toString());
  console.log('use_audience votes:', votes, '(expect 4 numbers summing to 100, correct idx', CORRECT[0], 'largest)');
  console.log('Balance after audience:', await bal(), '(expect 740)');
  try {
    await call('use_5050', { correct_answer: CORRECT[0] });
    console.log('ERROR: second 50:50 should have failed');
  } catch (e) {
    console.log('Second 50:50 correctly failed:', e.message.slice(0, 100));
  }

  console.log('\n--- 5. PHONE A FRIEND (real Genesis agent genesis-visitor-1) ---');
  const ownerBalBefore = Number(await view('mt_balance_of', { account_id: 'otoyo4real.testnet', token_id: 'ladder' }));
  console.log('Agent owner (otoyo4real.testnet) balance before:', ownerBalBefore);
  const pf = await call('phone_friend', { token_id: 'genesis-visitor-1', question_id: 0, correct_answer: CORRECT[0] });
  const pfAns = JSON.parse(Buffer.from(pf.status.SuccessValue, 'base64').toString());
  console.log('phone_friend answer:', pfAns, '(expect 0..3, or null if no record)');
  const ownerBalAfter = Number(await view('mt_balance_of', { account_id: 'otoyo4real.testnet', token_id: 'ladder' }));
  console.log('Agent owner balance after:', ownerBalAfter, '(expect +40 =', ownerBalBefore + 40, ')');
  console.log('Player balance after phone:', await bal(), '(expect 690 after 50 cost)');

  console.log('\n--- 6. WIN RUN (answer all 15, then win_run) ---');
  // Finish the current run (already used lifelines on Q1, answer Q1 correctly first).
  const a1 = await call('answer', { question_id: 0, answer: CORRECT[0], correct_answer: CORRECT[0] });
  console.log('answer Q1 ->', JSON.parse(Buffer.from(a1.status.SuccessValue, 'base64').toString()).outcome);
  for (let q = 1; q < 15; q++) {
    const r = await call('answer', { question_id: q, answer: CORRECT[q], correct_answer: CORRECT[q] });
    const res = JSON.parse(Buffer.from(r.status.SuccessValue, 'base64').toString());
    if (q === 14) console.log(`answer Q15 -> level ${res.level}, pot ${res.pot}, outcome ${res.outcome}`);
  }
  const win = await call('win_run', {});
  const winAmt = JSON.parse(Buffer.from(win.status.SuccessValue, 'base64').toString());
  console.log('win_run paid:', winAmt, '(expect "5000")');
  console.log('Balance after win:', await bal());
  const stats = await view('get_player_stats', { account_id: playerId });
  console.log('Player stats:', JSON.stringify(stats), '(expect wins 2: walk away Q10+ no, win 1 + walk away Q5 no... check)');

  console.log('\n--- 7. LEADERBOARD + EVOLVE ---');
  const lb = await view('get_leaderboard', {});
  console.log('Leaderboard:', JSON.stringify(lb).slice(0, 500));
  try {
    await call('evolve', {});
    console.log('evolve succeeded (unexpected with <3 wins)');
  } catch (e) {
    console.log('evolve correctly failed (needs 3 wins):', e.message.slice(0, 100));
  }
  const supply = await view('mt_total_supply', { token_id: 'ladder' });
  console.log('Total supply:', supply);

  console.log('\n--- 8. 60-SECOND TIMEOUT ---');
  await call('start_run', { question_hashes: hashes(), salt: SALT });
  await call('answer', { question_id: 0, answer: CORRECT[0], correct_answer: CORRECT[0] });
  const balBeforeTimeout = await bal();
  console.log('Run started, Q1 answered. Balance:', balBeforeTimeout, '. Waiting 65s for the window to expire...');
  await sleep(65000);
  const to = await call('timeout', { account_id: playerId });
  const toRes = JSON.parse(Buffer.from(to.status.SuccessValue, 'base64').toString());
  console.log('timeout() ->', toRes, '(expect timeout_lost_everything)');
  console.log('Balance after timeout:', await bal(), '(expect same as before — stake lost, nothing banked)');
  const runAfterTimeout = await view('get_run_view', { account_id: playerId });
  console.log('Run after timeout (expect null):', runAfterTimeout);

  console.log('\n--- DONE ---');
  console.log('Final player balance:', await bal());
  console.log('Final total supply:', await view('mt_total_supply', { token_id: 'ladder' }));
})().catch((e) => {
  console.error('VERIFY FAILED:', e);
  process.exit(1);
});
