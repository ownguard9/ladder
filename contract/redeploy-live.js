// Redeploy the Agent's Ladder contract WASM to the LIVE contract account
// (ladder-5a84ua.testnet). State is preserved: the answer-key state lives
// outside the main state struct, so no migration is needed. This script
// does NOT call `new` (re-initialising would wipe the state).
//
// PRECONDITIONS (do these first):
//   1. The game frontend has been updated to the new method signatures
//      (start_run takes bank question IDs; answer/use_5050/use_audience/
//      phone_friend no longer take correct_answer) and pushed live.
//      Deploying the contract first WILL break the live game.
//   2. No run is in progress on the live contract (a run in progress
//      uses the old Run layout). Check the known players with
//      get_run_view, or deploy at a quiet moment.
//   3. The WASM has been built: cargo near build non-reproducible-wasm.
//
// AFTER this script:
//   4. Populate the answer key: node populate-answer-key.js --submit
//      (12 batches), then node populate-answer-key.js --verify.
//   5. Verify on-chain: node verify.js
//   6. Play the live game in a real browser.
const fs = require('fs');
const path = require('path');
const { connect, keyStores, KeyPair } = require('near-api-js');

const CONTRACT = 'ladder-5a84ua.testnet';
const CONTRACT_DIR = __dirname;
const WASM_PATH = path.join(CONTRACT_DIR, 'target/near/agents_ladder.wasm');
const KEYS_PATH = path.join(CONTRACT_DIR, '.testnet-accounts.json');

(async () => {
  const keys = JSON.parse(fs.readFileSync(KEYS_PATH, 'utf8'));
  if (!keys[CONTRACT]) throw new Error('No key for ' + CONTRACT + ' in ' + KEYS_PATH);
  const deployKey = KeyPair.fromString(keys[CONTRACT].private_key);
  const keyStore = new keyStores.InMemoryKeyStore();
  await keyStore.setKey('testnet', CONTRACT, deployKey);
  const near = await connect({
    networkId: 'testnet', keyStore,
    nodeUrl: 'https://rpc.testnet.fastnear.com',
    walletUrl: 'https://testnet.mynearwallet.com',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://testnet.nearblocks.io',
  });
  const account = await near.account(CONTRACT);

  // Pre-check: the contract must still answer (state intact).
  const seasonBefore = await account.viewFunction({ contractId: CONTRACT, methodName: 'get_season', args: {} });
  console.log('Season before redeploy:', seasonBefore);

  const wasm = fs.readFileSync(WASM_PATH);
  console.log('Redeploying WASM (' + wasm.length + ' bytes) to', CONTRACT, '...');
  const deployTx = await account.deployContract(wasm);
  console.log('Redeploy tx:', deployTx.transaction.hash);

  // Post-check: state must be intact and the new methods present.
  const seasonAfter = await account.viewFunction({ contractId: CONTRACT, methodName: 'get_season', args: {} });
  console.log('Season after redeploy:', seasonAfter, '(must equal before)');
  if (String(seasonAfter) !== String(seasonBefore)) throw new Error('Season changed — state may have been reset!');
  const version = await account.viewFunction({ contractId: CONTRACT, methodName: 'get_answer_key_version', args: {} });
  console.log('Answer-key version:', version, '(0 until the key is populated)');
  const budget = await account.viewFunction({ contractId: CONTRACT, methodName: 'get_season_prize_budget', args: {} });
  console.log('Season prize budget:', budget, '(expect 9000 default)');
  console.log('REDEPLOY OK. Next: node populate-answer-key.js --submit && node populate-answer-key.js --verify && node verify.js');
})().catch((e) => {
  console.error('REDEPLOY FAILED:', e.message);
  process.exit(1);
});
