# Agent's Ladder — security

This document states the trust assumptions, known limitations and security status of Agent's Ladder plainly. It is written for a judge or developer reviewing the project. The game is a testnet hackathon project. Nothing here holds real funds and nothing runs on mainnet.

## Scope

- Contract: `ladder-5a84ua.testnet` on NEAR testnet. Full deployment history is in `README.md`.
- Game site: https://ladder-game-wheat.vercel.app/
- Ladder Coins are an internal onchain game currency. They are not redeemable for NEAR and they hold no monetary value. The worst case for any flaw below is a cheated leaderboard, not a loss of funds.

## Known limitations

### 1. Answer key authority: the most important limitation

**Status: FIXED and DEPLOYED 13 Nov 2026 (see `README.md` for the deployment history).**

The deployed contract trusts player supplied answer commitments. At `start_run` the player supplies the 15 question hashes and the salt. At `answer`, `use_5050`, `use_audience` and `phone_friend` the player supplies the `correct_answer`, and the contract checks it against the player supplied commitment. The player is both the committer and the verifier, so a technically capable player can choose their own 15 answers, generate matching hashes, start a run with them, submit the same answers as the correct answers, and win all 15 questions for 5,000 Ladder Coins.

The fix is deployed and verified on chain. The contract stores authoritative answer hashes for all 1,200 bank questions (SHA-256 of "<question_id>:<correct_index>:ladder-answer-key-v1", hashes only, never the answers), start_run takes bank question IDs, and answer and the lifelines no longer accept a player supplied correct answer. The contract is now the authority on the answer key and the player cannot choose it. Verified on the live contract 13 Nov 2026: forge attempts rejected, replay rejected, walk away banks 180, win pays 5,000, all checks green.

Do not remove `correct_answer` from `answer` without replacing the verification mechanism: without it there is no way to determine whether a submitted answer is correct.

### 2. Google sign-in is not verified on a trusted backend

Signing in uses Google Identity Services in the browser. The game decodes the Google ID token in the browser to read the email and name. It does not verify the token signature, audience, issuer, expiry or verified email claim on a trusted backend. A forged token presented to the browser could therefore impersonate a Google identity inside the game.

The impact is limited: the game account holds testnet NEAR and Ladder Coins with no monetary value, and the identity is used for a display name and saved progress, not for access to real assets. A proper identity to wallet mechanism (for example NEAR Auth with an own Auth0 application, or a backend that verifies Google ID tokens and manages wallet recovery) is the longer term answer. NEAR Auth with an own Auth0 application was tried during development and failed on the target phone with a DNS resolution error for the Auth0 tenant, which is why the current Google Identity Services approach was chosen: it works on the player's device.

### 3. The game account key lives in the browser

When a player signs in, the game generates a per player ed25519 keypair in the browser, stores it in localStorage keyed to the player's identity, creates and funds the NEAR testnet account through the public testnet helper (10 NEAR), and signs game transactions directly with that key. There is no password and no recovery flow.

That means:

- Signing in on another phone or browser creates a different NEAR account for the same Google account.
- Clearing browser storage destroys access to the locally stored signing key.
- A compromised browser or malicious script could steal the key.
- A locally saved session is restored without asking Google to authenticate again.

Do not store long lived private keys in ordinary localStorage if the game ever holds meaningful player assets. It does not today: the account holds testnet NEAR and Ladder Coins only.

### 4. The interface shows results before the chain confirms

The game shows the next question and the answer reveal at once, and the contract calls run in the background in the order played. That is a deliberate choice for speed: the player asked for a fast game, and waiting for chain confirmation on every tap made it feel slow. The local answer is reliable because the game holds the question bank and the answers, and the balance shown is refreshed from the contract after each run, so a failed transaction corrects the displayed balance. But a player can see a successful answer in the interface even if the transaction behind it fails. The contract, not the interface, is the source of truth for the balance and the run state, and the interface must never be treated as more authoritative than the contract.

### 5. Phone a Friend does not lock the run while it is pending

FIXED 13 Nov 2026: `answer`, `use_5050`, `use_audience`, `walk_away` and `win_run` all reject while a Phone a Friend call is in flight (`phone_pending`). The run is now fully locked during a pending Phone a Friend call.

### 6. Ask the Audience and Phone a Friend answers are simulated

Ask the Audience returns a vote distribution drawn from onchain randomness and biased toward the correct answer. It does not collect votes from real agents. Phone a Friend looks up a real Genesis Core agent record and pays the agent's owner 40 Ladder Coins onchain, but the answer is generated from the agent generation (70 percent correct at generation 0, plus 5 percent per generation, capped at 95 percent). It is not a live reply from the agent. Both are described as simulated in the README and in `contract.md`.

### 7. Duels and the Daily Challenge are not settled onchain

Real duels (pass and play, duel by code) and the Daily Challenge are client side. The duel result code is not proof the other player actually played, and the one attempt per day rule for the Daily Challenge is enforced in the browser, not by the contract. Onchain duel settlement and global Daily Challenge enforcement are planned milestones. The README says so plainly.

### 8. Leaderboard and season prizes

- `get_leaderboard` returns the top 50 players only. There is no pagination.
- Season prizes (500, 300, 100 Ladder Coins to the top three) are minted by `end_season` under an explicit treasury budget of 9,000 total Ladder Coins (10 seasons), enforced in the contract since 13 Nov 2026. `end_season` pays prizes only while the budget allows.
- `end_season` may be called by anyone after the week ends.

### 9. Partial NEP-245

The contract implements NEP-245 style methods for the single token id `"ladder"` (`mt_balance_of`, `mt_batch_balance_of`, `mt_total_supply`, `mt_tokens`, `mt_token`, `mt_transfer`, `mt_batch_transfer`). It does not implement `mt_transfer_call` or `mt_on_transfer` resolution. Ladder Coins are described as an internal onchain game currency, not as a fully NEP-245 compliant token.

## What is already sound

- Player authorisation uses `predecessor_account_id()` on every state changing call, so only the caller can move their own coins or act on their own run.
- The Phone a Friend cross contract callback is private, so only the contract itself can settle it.
- Release overflow checks are enabled.
- The contract never touches mainnet, never holds player funds beyond the game treasury and the stake held during a run, and never mints Genesis agent NFTs or changes Genesis state.
- The question set is committed by hash at run start, so answers are never stored in plain text in contract state (see limitation 1 for why the commitment alone is not enough).

## Reporting

This is a hackathon project. There is no bug bounty and no responsible disclosure process. If you find a flaw, the honest course is the one taken here: state it, fix it if it can be fixed safely, and document it if it cannot be fixed before the deadline.
