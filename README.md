# Agent's Ladder

**Answer. Climb. Risk it all.**

A Who Wants to Be a Millionaire style quiz game. You play as an AI agent, answer 15 questions to climb a money ladder, and risk it all for the million.

The site is multi page. Every page has a header and navigation: Home, Play, Daily Challenge, Invite to Duel, Leaderboard, How to play. Your wallet and profile carry across the pages.

## The question bank

1,200 questions in the bank: 300 easy, 300 medium, 300 hard, 300 very hard. Cryptocurrency is 30 percent of the bank (90 crypto questions per band, 360 in total); the other 70 percent covers history, science, geography, sport, entertainment, music, food, technology, AI and current affairs. A question you just answered does not repeat in your next run.

Difficulty rises from question 3: questions 1 and 2 are easy, questions 3 to 5 are medium, questions 6 to 10 are hard, questions 11 to 15 are very hard.

## How to play

Open `index.html` in a browser. No install, no wallet, no gas.

1. **Sign up** with your email and an agent name, sign in with your email, or play as a guest.
2. **Claim the faucet** to get 500 Ladder Coins in your wallet.
3. **Stake 100 coins** to start a run. Without staking you cannot play. There is no free run.
4. **Answer questions.** Each question has 4 answers in a 2 by 2 grid, and 1 minute on the clock. Tap an answer, then confirm "Is this your final answer?" — or say Yes or No by voice. Every question is read aloud with its four answers, and a gold badge shows what the question is worth.
5. **Climb the ladder.** 15 questions, 10 to 5,000 Ladder Coins. Question 5 pays 80, question 6 pays 120, question 7 pays 180, question 10 pays 500, question 15 pays 5,000.
6. **5 correct answers is the minimum to earn anything.** Lose before question 5 and you lose your 100 stake — your balance stays at 400.
7. **Safe points are decision points, not safety nets.** Question 5 and question 10 are the safe points. They do not protect you if you fail later. They are the moments the game asks you to choose.
8. **Walk away from question 5 upwards.** After every correct answer from question 5 onwards (question 5, 6, 7, 8, 9, 10, 11, 12, 13 and 14), you can walk away with your earnings so far, plus your 100 stake back. Walk away at question 5 and you bank 580. Walk away at question 6 and you bank 620. Walk away at question 7 and you bank 680. Walk away at question 10 and you bank 1,000.
9. **The game always asks.** After every correct answer from question 5 onwards, the game stops and asks you plainly: "Do you want to walk away with [amount] Ladder Coins, or continue playing?" — with a clear Walk away button and a clear Continue button. The game waits for your answer. It never skips the question and never moves on without you choosing.
10. **Refuse to walk away and fail, and you lose everything.** If you choose to keep playing and then get a question wrong, you lose everything from the run — no stake back, no pot, no safe point payout. Your balance stays at 400. The screen says "You lost everything. Your balance is 400 Ladder Coins."

Everything is audio and text: every question is read aloud as well as shown, every prompt and result is spoken as well as shown. The sign up form is text only.

## Daily Challenge

The Daily Challenge has its own page and its own header. It is hard on purpose: 5 questions drawn only from the hard and very hard pools. The same 5 questions go to everyone that day. You get one attempt per day, no retry. Answer one wrong and the day is over. Answer all 5 and you earn 500 Ladder Coins plus a place on the daily leaderboard, shown on the same page.

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

## Competitive play

- **Duels:** challenge another agent. Both stake 100, answer the same 15 questions, most correct answers wins both stakes (200). Tie returns both stakes.
- **Real duels:** two real human players duel each other. Both stake 100 and answer the same 15 questions; most correct answers wins 200, tie returns stakes. **Pass and play (same device):** "Duel a friend on this device". Enter both names. Player 1 answers the 15 questions, then Player 2 answers the same 15 in the same order. Both running scores show in the HUD, the winner is announced out loud, and the share card carries both names and both scores. **Duel by code (different devices):** "Create a duel code" gives a short code that fixes the 15 questions, so the same code always deals the same questions on any device. The other player joins with the code and plays the same set, then shares a short result code back. Entering it settles the duel: winner takes 200, tie returns stakes.
- **Practice duels:** duels against leaderboard agents are clearly labelled as practice ("Duel an agent"), separate from real duels ("Duel a real player").
- **Invite to Duel:** the invite has its own separate page, with its own header, away from the leaderboard. Open it from the navigation, pick a player from the leaderboard or enter their agent name, and send the invite. It states the stakes: both stake 100 over the same 15 questions, winner takes 200, tie returns stakes. The other player accepts or declines; if they accept, the duel starts at once. The invite status (sent, accepted, declined) shows on the page, and you can Copy the invite or Share it on X.
- **Weekly seasons:** the leaderboard runs in weekly seasons. Top three at the end of the week get 500, 300 and 100 bonus coins, then the board resets.
- **Win streaks:** consecutive wins show on the leaderboard. Three in a row earns a streak badge.

## Tracks

Games (primary) + Agent Economy (second). Lead: Isaac.

The game follows the Games track rules: NPCs with wallets, a player-owned economy that evolves through play without a dev patch, judged on the game loop not the token, and players never notice the chain — no wallet popups, no gas, no chain words in the UI.

## Files

- `index.html` — the full playable game (single file).
- `DESIGN.md` — the full design document.
- `contract.md` — the onchain contract spec for the repo build.

## The contract and the wallet

The Ladder Coins economy is live onchain on NEAR testnet. The contract is `ladder-5a84ua.testnet` (deploy tx `Hj4G6nXR3o5CwaxkVvu5NYACS6eoaETa7Rf9EvZazXMg`, treasury 7.71 NEAR). It implements the game rules exactly: faucet 500 once per season, stake 100, walk away banks pot plus stake, a wrong answer or a 60 second timeout loses everything, win pays 5,000, Phone a Friend pays the called agent 40 coins. The full spec is in `contract.md` and the source is in `contract/`.

The wallet connection is optional. A small Connect wallet button sits in the wallet panel. When a wallet is connected, the faucet, stake, answers, lifelines, walk away and win all settle through the contract, and the balance shown comes from the contract. When no wallet is connected, the game plays exactly as before with the on device economy, so guests and players without a wallet can still play. Disconnecting restores the on device balance. The game never blocks a player who does not connect, and no chain words appear anywhere the player can see.

## Status

Live on NEAR testnet. The contract is deployed and verified; the game uses it when a wallet is connected and plays the on device economy when one is not.
