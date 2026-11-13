# Agent's Ladder

**Answer. Climb. Risk it all.**

A quiz game where you play as an AI agent. Answer 15 questions to climb the ladder from 10 to 5,000 Ladder Coins, and at every step decide whether to bank your winnings or risk it all on the next question.

The site is multi page. Every page has a header and navigation: Home, Play, Daily Challenge, Invite to Duel, Leaderboard, How to play. Your wallet and profile carry across the pages.

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

This table says plainly what genuinely settles onchain and what is still simulated in the browser. The game is honest about the split.

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

1. **Sign in with Gmail, or play as a guest.** A small Continue with Gmail button sits in the wallet panel. Signing in creates your game account behind the scenes; the faucet, stake, answers, lifelines, walk away and win all settle through the contract, and the balance shown comes from the contract. Playing as a guest uses the on device economy, so guests can still play. Signing out restores the on device balance. The game never blocks a player who does not sign in, and no chain words appear anywhere the player can see.
2. **Claim the faucet** to get 500 Ladder Coins in your wallet.
3. **Stake 100 coins** to start a run. Without staking you cannot play. There is no free run.
4. **Answer questions.** Each question has 4 answers in a 2 by 2 grid, and 1 minute on the clock. Tap an answer, then confirm "Is this your final answer?" — or say Yes or No by voice. Every question is read aloud with its four answers, and a gold badge shows what the question is worth.
5. **Climb the ladder.** 15 questions, 10 to 5,000 Ladder Coins. Question 5 pays 80, question 6 pays 120, question 7 pays 180, question 10 pays 500, question 15 pays 5,000.
6. **5 correct answers is the minimum to earn anything.** Lose before question 5 and you lose your 100 stake — your balance stays at 400.
7. **Safe points are decision points, not safety nets.** Question 5 and question 10 are the safe points. They do not protect you if you fail later. They are the moments the game asks you to choose.
8. **Walk away from question 5 upwards.** After every correct answer from question 5 onwards (question 5, 6, 7, 8, 9, 10, 11, 12, 13 and 14), you can walk away with your earnings so far, plus your 100 stake back. Walk away at question 5 and you bank 180 (80 pot plus your 100 stake back), so your balance becomes 580. Walk away at question 6 and you bank 220, so your balance becomes 620. Walk away at question 7 and you bank 280, so your balance becomes 680. Walk away at question 10 and you bank 600, so your balance becomes 1,000.
9. **The game always asks.** After every correct answer from question 5 onwards, the game stops and asks you plainly: "Do you want to walk away with [amount] Ladder Coins, or continue playing?" — with a clear Walk away button and a clear Continue button. The game waits for your answer. It never skips the question and never moves on without you choosing.
10. **Refuse to walk away and fail, and you lose everything.** If you choose to keep playing and then get a question wrong, you lose everything from the run — no stake back, no pot, no safe point payout. Your balance stays at 400. The screen says "You lost everything. Your balance is 400 Ladder Coins."

Everything is audio and text: every question is read aloud as well as shown, every prompt and result is spoken as well as shown. The sign up form is text only. When you open the site fresh, a short original rising chime plays and a voice says "Welcome to Agent's Ladder. Answer, climb, and risk it all." In How to play, under Listening, you can choose a Male voice, a Female voice, or Any voice; every spoken line uses the voice you pick.

## Daily Challenge

The Daily Challenge has its own page and its own header. It is hard on purpose: 5 questions drawn only from the hard and very hard pools. The same 5 questions go to everyone that day. You get one attempt per day, no retry. Answer one wrong and the day is over. Answer all 5 and you earn 500 Ladder Coins plus a place on the daily leaderboard, shown on the same page.

The Daily Challenge is client side for now: the one attempt per day rule is enforced in the browser, not by the contract. Global enforcement onchain is a planned milestone.

## Share your score

After a run, a duel or a Daily Challenge, the game shows a share score card: your score big and bold, your agent name from sign up on the card (Guest if you played as a guest), the question you reached, and the Agent's Ladder name. Share on X or Copy it.

## Try it — do this, see that

1. Claim the faucet and see 500 Ladder Coins in your wallet.
2. Stake 100 coins to start a run and see your balance drop to 400.
3. Answer 5 questions correctly and see the game ask "Do you want to walk away with 180 Ladder Coins, or continue playing?"
4. Tap Walk away at question 5 and see "You banked 80 coins, plus your 100 stake back. Your balance is now 580 Ladder Coins."
5. Start a new run, answer 6 questions correctly, tap Walk away and see your balance become 620.
6. Start a new run, answer 5 questions correctly, tap Continue, then get question 6 wrong and see "You lost everything. Your balance is 400 Ladder Coins."
7. Start a new run, get a question wrong before question 5 and see "You lost your stake" with your balance at 400.
8. Tap 50:50 on a question and see two wrong answers removed, leaving the correct answer and one other.

## Lifelines

Once per run each: 50:50 (20 coins), Ask the Audience (20 coins), Phone a Friend (50 coins — call a real agent and see their record before you choose who to call).

Phone a Friend looks up a real agent record onchain and pays the called agent's owner 40 Ladder Coins. The answer itself is generated from the agent's generation (70 percent correct at generation 0, plus 5 percent per generation, capped at 95 percent). It is not a live reply from the agent. Ask the Audience shows a vote distribution drawn from onchain randomness and biased toward the correct answer (55 to 75 percent for the correct answer). It is not votes from real agents.

## Competitive play

- **Duels:** challenge another agent. Both stake 100, answer the same 15 questions, most correct answers wins both stakes (200). Tie returns both stakes.
- **Real duels:** two real human players duel each other. Both stake 100 and answer the same 15 questions; most correct wins 200, tie returns stakes. **Pass and play (same device):** "Duel a friend on this device". Enter both names. Player 1 answers the 15 questions, then Player 2 answers the same 15 in the same order. Both running scores show in the HUD, the winner is announced out loud, and the share card carries both names and both scores. **Duel by code (different devices):** "Create a duel code" gives a short code that fixes the 15 questions, so the same code always deals the same questions on any device. The other player joins with the code and plays the same set, then shares a short result code back. Entering it settles the duel: winner takes 200, tie returns stakes. Real duels are client side for now; the result code is not proof the other player actually played, and onchain duel settlement (create duel, lock both stakes, commit the shared question set, record both players' answers, settle onchain, refund on a tie) is a planned milestone.
- **Practice duels:** duels against leaderboard agents are clearly labelled as practice ("Duel an agent"), separate from real duels ("Duel a real player").
- **Invite to Duel:** the invite has its own separate page, with its own header, away from the leaderboard. Open it from the navigation, pick a player from the leaderboard or enter their agent name, and send the invite. It states the stakes: both stake 100 over the same 15 questions, winner takes 200, tie returns stakes. The other player accepts or declines; if they accept, the duel starts at once. The invite status (sent, accepted, declined) shows on the page, and you can Copy the invite or Share it on X.
- **Weekly seasons:** the leaderboard runs in weekly seasons. Top three at the end of the week get 500, 300 and 100 bonus coins, then the board resets. Season prizes are settled onchain by `end_season`, which anyone may call after the week ends.
- **Win streaks:** consecutive wins show on the leaderboard. Three in a row earns a streak badge.

## Tokenomics

Ladder Coins are an internal onchain game currency. They are recorded in the game contract's ledger and they are not redeemable for NEAR. 1 Ladder Coin equals 0.001 testnet NEAR for accounting only: the ladder values map to testnet value, but there is no exchange and no redemption, and the contract never pays out NEAR to a player.

How coins move:

- **Faucet:** 500 Ladder Coins once per season per account.
- **Stake:** 100 Ladder Coins to start a run, held by the contract during the run.
- **Walk away:** banks the pot plus the 100 stake back.
- **Win:** 5,000 Ladder Coins for answering all 15 questions, minted as a ladder payout. The 100 stake stays with the treasury on a win.
- **Lifelines:** 50:50 costs 20, Ask the Audience costs 20, Phone a Friend costs 50 (40 to the called agent's owner, 10 to the treasury).
- **Season prizes:** 500, 300 and 100 to the top three, settled onchain at the end of the week.
- **Evolution:** 3 paid wins moves the player agent up one generation, up to generation 9.

Testnet NEAR pays transaction costs and funds the account infrastructure. Players do not need to understand any of that to play: they see coins, not NEAR, not gas, not chain words.

The contract implements NEP-245 style multi token methods for the single token id `"ladder"` (`mt_balance_of`, `mt_batch_balance_of`, `mt_total_supply`, `mt_tokens`, `mt_token`, `mt_transfer`, `mt_batch_transfer`), but it does not implement the complete NEP-245 interface: there is no `mt_transfer_call` and no `mt_on_transfer` resolution. Ladder Coins are described as an internal onchain game currency, not as a fully NEP-245 compliant token.

## Tracks

Games (primary) + Agent Economy (second). Lead: Isaac.

The game follows the Games track rules: NPCs with wallets, a player-owned economy that evolves through play without a dev patch, judged on the game loop not the token, and players never notice the chain — no wallet popups, no gas, no chain words in the UI.

## Files

- `index.html` — the full playable game (single file).
- `DESIGN.md` — the full design document.
- `contract.md` — the deployed contract, as it actually is.
- `SECURITY.md` — trust assumptions, known limitations and security status.
- `contract/` — the contract source (`src/lib.rs`), deploy script and verification script.

## The contract and signing in

The Ladder Coins economy is live onchain on NEAR testnet. The contract is `ladder-5a84ua.testnet` (see Deployment history; treasury 7.71 NEAR). It implements the game rules exactly: faucet 500 once per season, stake 100, walk away banks pot plus stake, a wrong answer or a 60 second timeout loses everything, win pays 5,000, Phone a Friend pays the called agent's owner 40 coins. The full spec is in `contract.md` and the source is in `contract/`.

Signing in is optional. A small Continue with Gmail button sits in the wallet panel. When you sign in with Gmail, the game creates and registers your account behind the scenes and the faucet, stake, answers, lifelines, walk away and win all settle through the contract, and the balance shown comes from the contract. When you do not sign in, the game plays exactly as before with the on device economy, so guests and players without an account can still play. Signing out restores the on device balance. The game never blocks a player who does not sign in, and no chain words appear anywhere the player can see.

## Status

Live on NEAR testnet. The contract is deployed and verified; the game uses it when you sign in with Gmail and plays the on device economy when you do not. The answer key verification fix is deployed and verified on chain (13 Nov 2026, answer key version 12, all 1,200 hashes populated and spot checked), and the audience-votes fix followed the same day (see Deployment history). The contract now holds the authoritative answer key, so the player cannot choose their own correct answers. See `SECURITY.md` for the remaining known limitations.
