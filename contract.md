# Agent's Ladder game contract spec

Status: specified, not deployed. This demo runs the Ladder Coins economy in the browser and reads live Genesis agent records from the chain. This document specifies the onchain game contract for the GitHub and Vercel build, where the economy settles on NEAR testnet.

## Purpose

A small contract that holds the game economy and settles it onchain, reading agent records from the Genesis Core (`core.genesisdemo354551.testnet`) and never duplicating Genesis state.

## State

- `faucet_paid: Map<AccountId, bool>` — whether an account has claimed the starting 500 Ladder Coins this season.
- `runs: Map<AccountId, Run>` — the current run per player: level reached, pot, lifelines used, started at, season.
- `wins: Map<AccountId, u32>` — paid wins per player.
- `generation: Map<AccountId, u8>` — player agent generation, starts at 0, max 9.
- `season: u64` — the current weekly season number.
- `treasury: AccountId` — the game treasury, funded once at deploy from the Genesis Treasury by the deployer. Testnet only.

## Methods

- `claim_faucet()` — gives the caller 500 Ladder Coins once per season. Fails if already claimed this season.
- `start_run()` — starts a run. Fails if a run is already in progress for the caller.
- `answer(question_id: u32, answer: u8)` — records an answer for the current level. The question set is committed at run start as a hash, and the correct answers are revealed by the contract at settle time, so the player cannot read the answers from state. A correct answer moves the run up one rung and grows the pot to that rung value. A wrong answer, or a call to `timeout()` after 30 seconds without an answer, ends the run and pays the last safe point (80 after question 5, 500 after question 10, else 0).
- `use_5050()`, `use_audience()` — each costs 20 Ladder Coins, once per run. 50:50 is resolved by the contract. Audience is resolved from a recorded vote of Genesis agent records weighted by wins.
- `phone_friend(token_id: String)` — costs 50 Ladder Coins. Pays the called Genesis agent 40 coins and keeps 10 in the game treasury. The called agent must hold a Core record. The agent answer is resolved by the contract from the agent generation: 70 percent correct at generation 0, plus 5 percent per generation, capped at 95 percent, using onchain randomness from the block.
- `walk_away()` — banks the current pot, ends the run. Walking away at question 10 or above counts as a paid win.
- `win_run()` — called by the contract when question 15 is answered correctly. Pays 5,000 Ladder Coins and counts as a paid win.
- `evolve()` — after 3 paid wins, moves the player agent up one generation, up to generation 9. Follows the Genesis rule.
- `get_leaderboard()` — returns players ranked by total coins earned, then generation, then win rate, for the current season.
- `end_season()` — anyone may call it after the week ends. Crowns the top three, records them, resets the season board.

## Money

- Ladder Coins are a NEP-245 multi token minted by this contract at 1 Ladder Coin to 0.001 testnet NEAR for accounting, so the ladder values in the design map to real testnet value: question 1 pays 10 coins, question 15 pays 5,000 coins. The player never sees NEAR in the game UI.
- Lifeline payments to Genesis agents settle in testnet NEAR through the Genesis Core payment path. Phone a Friend pays the called agent directly.
- The faucet is capped per season per account. The treasury is funded once at deploy. No minting beyond the faucet cap and the ladder payouts.

## What it does not do

- It does not mint Genesis agent NFTs and does not change Genesis state. It only reads Genesis records.
- It does not hold player funds beyond the game treasury. It never touches mainnet.
- It does not store question answers in plain text. The question set is committed by hash at run start.

## Deploy checklist (for the repo build)

1. Build with `cargo near build non-reproducible-wasm`.
2. Deploy to a fresh testnet account funded from the testnet faucet.
3. Fund the game treasury with testnet NEAR.
4. Point the site at the deployed contract and the Genesis Core.
5. Play a full run on the deployed site in a real browser before calling it done.
