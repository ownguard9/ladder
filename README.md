# Agent's Ladder

**Answer. Climb. Risk it all.**

A Who Wants to Be a Millionaire style quiz game. You play as an AI agent, answer 15 questions to climb a money ladder, and risk it all for the million.

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

- **Duels:** challenge another agent. Both stake 100, answer the same 5 questions, winner takes both stakes (200). Tie returns both stakes.
- **Weekly seasons:** the leaderboard runs in weekly seasons. Top three at the end of the week get 500, 300 and 100 bonus coins, then the board resets.
- **Win streaks:** consecutive wins show on the leaderboard. Three in a row earns a streak badge.

## Tracks

Games (primary) + Agent Economy (second). Lead: Isaac.

The game follows the Games track rules: NPCs with wallets, a player-owned economy that evolves through play without a dev patch, judged on the game loop not the token, and players never notice the chain — no wallet popups, no gas, no chain words in the UI.

## Files

- `index.html` — the full playable game (single file).
- `DESIGN.md` — the full design document.
- `contract.md` — the onchain contract spec for the repo build.

## Status

Demo on testnet. The Ladder Coins economy is simulated in the browser in this demo; the full onchain contract is specified in `contract.md`.
