// Deploy Agent's Ladder contract to a fresh NEAR testnet account.
// Creates the account via the testnet helper (10 NEAR faucet), deploys
// the WASM, and initialises the contract. Saves keys to
// ~/workspace/millionaire/contract/.testnet-accounts.json (chmod 600).
const fs = require('fs');
const path = require('path');
const { connect, keyStores, KeyPair, utils } = require('/home/hatch/workspace/merit/node_modules/near-api-js');

const CONTRACT_DIR = '/home/hatch/workspace/millionaire/contract';
const WASM_PATH = path.join(CONTRACT_DIR, 'target/near/agents_ladder.wasm');
const KEYS_PATH = path.join(CONTRACT_DIR, '.testnet-accounts.json');
const HELPER_URL = 'https://helper.testnet.near.org/account';

function randomSuffix() {
  return Math.random().toString(36).slice(2, 8);
}

async function createAccount(accountId, publicKey) {
  const res = await fetch(HELPER_URL, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ newAccountId: accountId, newAccountPublicKey: publicKey }),
  });
  const text = await res.text();
  if (!res.ok) throw new Error(`helper ${res.status}: ${text.slice(0, 300)}`);
  return JSON.parse(text);
}

(async () => {
  const accountId = `ladder-${randomSuffix()}.testnet`;
  const keyPair = KeyPair.fromRandom('ed25519');
  const publicKey = keyPair.getPublicKey().toString();
  console.log('Creating account:', accountId);
  const created = await createAccount(accountId, publicKey);
  console.log('Helper response:', JSON.stringify(created).slice(0, 200));

  // Save keys (never print the secret key).
  const keyStore = new keyStores.InMemoryKeyStore();
  await keyStore.setKey('testnet', accountId, keyPair);
  const keys = {};
  keys[accountId] = {
    account_id: accountId,
    public_key: publicKey,
    private_key: keyPair.toString(),
  };
  fs.writeFileSync(KEYS_PATH, JSON.stringify(keys, null, 2));
  fs.chmodSync(KEYS_PATH, 0o600);
  console.log('Keys saved to', KEYS_PATH, '(chmod 600)');

  const config = {
    networkId: 'testnet',
    keyStore,
    nodeUrl: 'https://rpc.testnet.fastnear.com',
    walletUrl: 'https://testnet.mynearwallet.com',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://testnet.nearblocks.io',
  };
  const near = await connect(config);
  const account = await near.account(accountId);
  const balance = await account.getAccountBalance();
  console.log('Account balance:', utils.format.formatNearAmount(balance.available), 'NEAR available');

  // Deploy the contract.
  const wasm = fs.readFileSync(WASM_PATH);
  console.log('Deploying WASM (' + wasm.length + ' bytes)...');
  const deployTx = await account.deployContract(wasm);
  console.log('Deploy tx:', deployTx.transaction.hash);

  // Initialise: owner = this account, treasury = this account.
  console.log('Initialising contract...');
  const initTx = await account.functionCall({
    contractId: accountId,
    methodName: 'new',
    args: { owner_id: accountId, treasury: accountId },
    gas: BigInt('30000000000000'),
  });
  console.log('Init tx:', initTx.transaction.hash);

  // Confirm it is live.
  const season = await account.viewFunction({ contractId: accountId, methodName: 'get_season', args: {} });
  console.log('get_season:', season);
  const treasury = await account.viewFunction({ contractId: accountId, methodName: 'get_treasury', args: {} });
  console.log('get_treasury:', treasury);
  const bal2 = await account.getAccountBalance();
  console.log('Balance after deploy:', utils.format.formatNearAmount(bal2.available), 'NEAR available');
  console.log('CONTRACT_ACCOUNT=' + accountId);
})().catch((e) => {
  console.error('DEPLOY FAILED:', e.message);
  process.exit(1);
});
