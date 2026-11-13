# Agent's Ladder

**Answer. Climb. Risk it all.**

A quiz game where you play as an AI agent. Answer 15 questions to climb the ladder from 10 to 5,000 Ladder Coins, and at every step decide whether to bank your winnings or risk it all on the next question.

## Live deployment

- **Game:** https://ladder-game-wheat.vercel.app/
- **Contract:** `ladder-5a84ua.testnet` on NEAR testnet
- **Treasury:** 7.71 NEAR (the contract account's own balance)
- **Explorer:** https://testnet.nearblocks.io/address/ladder-5a84ua.testnet
- **Status:** deployed and verified live on testnet. The full contract spec is in `contract.md` and the source is in `contract/`. Known limitations are listed in `SECURITY.md`.

### Deployment history

The contract account is the same throughout; only the code was replaced. State (balances, answer key, season) was preserved across both upgrades.

- **Original deployment — 12 Nov 2026, 15:02 UTC:** `Hj4G6nXR3o5CwaxkVvu5NYACS6eoaETa7Rf9EvZazXMg` — first deploy of the contract. ([explorer](https://testnet.nearblocks.io/txns/Hj4G6nXR3o5CwaxkVvu5NYACS6eoaETa7Rf9EvZazXMg))
- **First upgrade — 13 Nov 2026, 00:05 UTC:** `HV1b1qsDPKHG8qgQz8wKhRDdgSPU99N9GdksL1WsXhMY` — answer-key security fix (the contract becomes the authority on correct answers). ([explorer](https://testnet.nearblocks.io/txns/HV1b1qsDPKHG8qgQz8wKhRDdgSPU99N9GdksL1WsXhMY))
- **Latest upgrade — 13 Nov 2026, 08:15 UTC (current code):** `2JwQtYccobbpcu2tv7xinuhR4MFviVLgzHbb29zmMtCx` — Ask the Audience vote-distribution fix (votes now always sum to 100). ([explorer](https://testnet.nearblocks.io/txns/2JwQtYccobbpcu2tv7xinuhR4MFviVLgzHbb29zmMtCx))
- **Current deployed code hash:** `jJbSQaF3mm5Lx4g15Ek34PW3qcKRYdBsfcuFysdRFcR` — matches the WASM built from `contract/src/lib.rs` on main.

## What is onchain and what is simulated

| Feature | Status |
| --- | --- |
| Google sign-in | Live |
| NEAR testnet account | Live |
| Faucet | On-chain |
| Ladder run settlement | On-chain (answer-key verification fix deployed 13 Nov 2026; see Deployment history) |
| Agent lookup (Phone a Friend) | On-chain (agent record + payment) |
| Agent response (Phone a Friend answer) | Simulated (accuracy-based, not a live agent reply) |
| Ask the Audience | Simulated (random distribution biased to the correct answer, not real agent votes) |
| Real-time duels | Not yet on-chain (client-side duel codes; on-chain settlement is a planned milestone) |
| Daily leaderboard / Daily Challenge | Not yet on-chain (client-side; global enforcement is a planned milestone) |
| Ladder Coins | Internal on-chain game currency, not redeemable for NEAR, not full NEP-245 |

## How to play

Open the site in a browser. No install, no wallet, no gas.

1. **Sign in with Gmail, or play as a guest.** Signing in creates your game account behind the scenes; the faucet, stake, answers, lifelines, walk away and win all settle through the contract. Guests play with the on device economy. No chain words appear anywhere the player can see.
2. **Claim the faucet** for 500 Ladder Coins, then **stake 100** to start a run.
3. **Answer 15 questions**, 4 answers each, 1 minute on the clock. Tap an answer, then confirm it is your final answer — or say Yes or No by voice. Every question is read aloud as well as shown.
4. **Climb the ladder:** question 5 pays 80, question 6 pays 120, question 7 pays 180, question 10 pays 500, question 15 pays 5,000.
5. **Walk away from question 5 upwards.** After every correct answer from question 5 on, the game asks whether to walk away with the pot plus the 100 stake back (180 at question 5, so a 580 balance) or keep playing.
6. **Refuse and fail, and you lose everything.** Keep playing after refusing to walk away, then get a question wrong, and the run pays nothing — no stake back, no pot. Lose before question 5 and you lose only the 100 stake.

## Daily Challenge

5 hard or very hard questions, the same 5 for everyone that day, one attempt per day, no retry. Answer all 5 and earn 500 Ladder Coins plus a place on the daily leaderboard. Client side for now; onchain enforcement is a planned milestone.

## Lifelines

Once per run each: 50:50 (20 coins), Ask the Audience (20 coins), Phone a Friend (50 coins). Phone a Friend looks up a real agent record onchain and pays the called agent's owner 40 coins; the answer itself is generated from the agent's generation (70 percent correct at generation 0, plus 5 percent per generation, capped at 95 percent), not a live reply. Ask the Audience shows a vote distribution biased toward the correct answer (55 to 75 percent), not votes from real agents.

## Competitive play

- **Duels:** both stake 100 over the same 15 questions; most correct answers wins 200, tie returns stakes. Pass and play on one device, or duel by code across devices. Practice duels against leaderboard agents are clearly labelled; real duels are client side for now, and onchain settlement is a planned milestone.
- **Weekly seasons:** top three at the end of the week get 500, 300 and 100 bonus coins, settled onchain by `end_season`, then the board resets.
- **Win streaks:** three consecutive wins earns a streak badge on the leaderboard.

## Tokenomics

Ladder Coins are an internal onchain game currency, not redeemable for NEAR. 1 Ladder Coin equals 0.001 testnet NEAR for accounting only; there is no exchange and no redemption, and the contract never pays out NEAR to a player. Faucet 500 once per season, stake 100 per run, walk away banks pot plus stake, win pays 5,000, lifelines cost 20/20/50, season prizes 500/300/100. 3 paid wins moves the player agent up one generation, up to generation 9.

## Tracks

Games (primary) + Agent Economy (second). Lead: Isaac.

## Files

- `index.html` — the full playable game (single file).
- `DESIGN.md` — the full design document.
- `contract.md` — the deployed contract, as it actually is.
- `SECURITY.md` — trust assumptions, known limitations and security status.
- `contract/` — the contract source (`src/lib.rs`), deploy script and verification script.

## Status

Live on NEAR testnet. The answer key verification fix is deployed and verified on chain (13 Nov 2026, answer key version 12, all 1,200 hashes populated and spot checked), and the audience-votes fix followed the same day (see Deployment history). See `SECURITY.md` for the remaining known limitations.
