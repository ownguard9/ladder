// Build the authoritative answer key for Agent's Ladder and (optionally)
// populate it on the deployed contract.
//
// The answer key is 1,200 SHA-256 hashes, one per bank question:
//   hash = SHA-256 hex of "<question_id>:<correct_index>:<SALT>"
// Bank question IDs: easy 0..299, medium 300..599, hard 600..899,
// veryhard 900..1199 (bank-*.json order within each band).
// The contract stores HASHES ONLY — the answers are never sent on-chain.
//
// Usage:
//   node populate-answer-key.js            # generate batch files only
//   node populate-answer-key.js --submit   # generate + submit to the contract
//   node populate-answer-key.js --verify   # read back a sample and compare
//
// The contract method is owner-only; the owner key is read from
// ~/workspace/millionaire/contract/.testnet-accounts.json (never printed).
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { connect, keyStores, KeyPair, utils } = require('near-api-js');

const CONTRACT = process.argv[2] && !process.argv[2].startsWith('--') ? process.argv[2] : 'ladder-5a84ua.testnet';
const CONTRACT_DIR = __dirname;
const KEYS_PATH = path.join(CONTRACT_DIR, '.testnet-accounts.json');
const QUESTIONS_DIR = path.join(__dirname, '..', 'questions');
const BATCHES_DIR = path.join(CONTRACT_DIR, 'answer-key-batches');
const SALT = 'ladder-answer-key-v1';
const BATCH_SIZE = 100;

const BANDS = [
  { file: 'bank-easy.json', offset: 0 },
  { file: 'bank-medium.json', offset: 300 },
  { file: 'bank-hard.json', offset: 600 },
  { file: 'bank-veryhard.json', offset: 900 },
];

function buildKey() {
  const entries = [];
  for (const band of BANDS) {
    const bank = JSON.parse(fs.readFileSync(path.join(QUESTIONS_DIR, band.file), 'utf8'));
    bank.forEach((q, i) => {
      const qid = band.offset + i;
      const hash = crypto.createHash('sha256').update(`${qid}:${q.c}:${SALT}`).digest('hex');
      entries.push([qid, hash]);
    });
  }
  entries.sort((a, b) => a[0] - b[0]);
  return entries;
}

function writeBatches(entries) {
  fs.mkdirSync(BATCHES_DIR, { recursive: true });
  const files = [];
  for (let i = 0; i < entries.length; i += BATCH_SIZE) {
    const batch = entries.slice(i, i + BATCH_SIZE);
    const name = `batch-${String(i / BATCH_SIZE).padStart(2, '0')}.json`;
    const file = path.join(BATCHES_DIR, name);
    fs.writeFileSync(file, JSON.stringify({ entries: batch }, null, 2));
    files.push(file);
  }
  return files;
}

async function submit(entries) {
  const keys = JSON.parse(fs.readFileSync(KEYS_PATH, 'utf8'));
  if (!keys[CONTRACT]) throw new Error('No key for ' + CONTRACT + ' in ' + KEYS_PATH);
  const ownerKey = KeyPair.fromString(keys[CONTRACT].private_key);
  const keyStore = new keyStores.InMemoryKeyStore();
  await keyStore.setKey('testnet', CONTRACT, ownerKey);
  const near = await connect({
    networkId: 'testnet', keyStore,
    nodeUrl: 'https://rpc.testnet.fastnear.com',
    walletUrl: 'https://testnet.mynearwallet.com',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://testnet.nearblocks.io',
  });
  const owner = await near.account(CONTRACT);
  const GAS = BigInt('100000000000000'); // 100 TGas
  for (let i = 0; i < entries.length; i += BATCH_SIZE) {
    const batch = entries.slice(i, i + BATCH_SIZE);
    const tx = await owner.functionCall({
      contractId: CONTRACT, methodName: 'set_question_hashes',
      args: { entries: batch }, gas: GAS,
      attachedDeposit: BigInt(0),
    });
    console.log(`batch ${i / BATCH_SIZE + 1}/${Math.ceil(entries.length / BATCH_SIZE)} (${batch[0][0]}..${batch[batch.length - 1][0]}) tx:`, tx.transaction.hash);
  }
  const version = await owner.viewFunction({ contractId: CONTRACT, methodName: 'get_answer_key_version', args: {} });
  console.log('Answer-key version after submit:', version);
}

async function verify(entries) {
  const keys = JSON.parse(fs.readFileSync(KEYS_PATH, 'utf8'));
  const ownerKey = KeyPair.fromString(keys[CONTRACT].private_key);
  const keyStore = new keyStores.InMemoryKeyStore();
  await keyStore.setKey('testnet', CONTRACT, ownerKey);
  const near = await connect({
    networkId: 'testnet', keyStore,
    nodeUrl: 'https://rpc.testnet.fastnear.com',
    walletUrl: 'https://testnet.mynearwallet.com',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://testnet.nearblocks.io',
  });
  const owner = await near.account(CONTRACT);
  const version = await owner.viewFunction({ contractId: CONTRACT, methodName: 'get_answer_key_version', args: {} });
  console.log('Answer-key version on chain:', version);
  // Spot-check 12 questions spread across the bank.
  const sample = [0, 1, 150, 299, 300, 450, 599, 600, 750, 899, 900, 1199];
  let ok = 0;
  for (const qid of sample) {
    const expected = entries[qid][1];
    const onchain = await owner.viewFunction({ contractId: CONTRACT, methodName: 'get_question_hash', args: { question_id: qid } });
    const match = onchain === expected;
    if (match) ok++;
    console.log(`qid ${qid}: ${match ? 'MATCH' : 'MISMATCH (onchain=' + onchain + ' expected=' + expected + ')'}`);
  }
  console.log(`Spot-check: ${ok}/${sample.length} match`);
  if (ok !== sample.length) process.exit(1);
}

(async () => {
  const entries = buildKey();
  console.log('Built answer key:', entries.length, 'hashes (salt:', SALT + ')');
  // Sanity: every hash is 64 hex chars, ids are 0..1199 with no gaps.
  const bad = entries.filter(([qid, h]) => !/^[0-9a-f]{64}$/.test(h));
  if (bad.length) throw new Error('Bad hashes: ' + bad.length);
  if (entries.length !== 1200 || entries[0][0] !== 0 || entries[1199][0] !== 1199) {
    throw new Error('Unexpected id range');
  }
  const files = writeBatches(entries);
  console.log('Wrote', files.length, 'batch files to', BATCHES_DIR);
  if (process.argv.includes('--submit')) await submit(entries);
  if (process.argv.includes('--verify')) await verify(entries);
})().catch((e) => {
  console.error('POPULATE FAILED:', e.message);
  process.exit(1);
});
