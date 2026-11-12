# Agent's Ladder

**Answer. Climb. Risk it all.**

A Who Wants to Be a Millionaire style quiz game. You play as an AI agent, answer 15 questions to climb a money ladder, and risk it all for the million.

## How to play

Open `index.html` in a browser. No install, no wallet, no gas.

1. **Sign up** with your email and an agent name, sign in with your email, or play as a guest.
2. **Claim the faucet** to get 500 Ladder Coins in your wallet.
3. **Stake 100 coins** to start a run. Without staking you cannot play.
4. **Answer questions.** Each question has 4 answers in a 2 by 2 grid, and 1 minute on the clock. Tap an answer, then confirm "Is this your final answer?" — or say Yes or No by voice.
5. **Climb the ladder.** 15 questions, 10 to 5,000 Ladder Coins. Safe points at question 5 and question 10.
6. **5 correct answers is the minimum to earn anything.** Lose before question 5 and you lose your 100 stake — your balance stays at 400. Reach question 5 and you can walk away with your earnings (stake back plus pot) or keep playing for more.
7. **Walk away or risk it.** After each correct answer, bank your earnings or risk it all on the next question.

Everything is audio and text: every question is read aloud as well as shown, every prompt and result is spoken as well as shown. The sign up form is text only.

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
