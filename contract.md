# Agent's Ladder game contract

Status: DEPLOYED on NEAR testnet. The Ladder Coins economy settles onchain.
Contract account: `ladder-5a84ua.testnet`. The game reads live Genesis agent
records from the Genesis Core (`core.genesisdemo354551.testnet`) and never
duplicates or changes Genesis state.

## What Ladder Coins are

Ladder Coins are an INTERNAL on-chain game currency. They are minted and
moved by this contract only. They are NOT redeemable for NEAR and are NOT
NEAR-backed. 1 coin = 0.001 testnet NEAR for accounting only. Testnet NEAR
pays transaction costs and funds the account infrastructure; players never
see NEAR in the game UI.

## The answer key (the security model)

The contract is the AUTHORITY on the answer key. The owner stores SHA-256
hashes of the correct answers for all 1,200 bank questions (hashes only —
the answers themselves are never stored on-chain). Each hash is
`SHA-256("<question_id>:<correct_index>:ladder-answer-key-v1")`.

Bank question IDs: easy 0..299, medium 300..599, hard 600..899,
veryhard 900..1199 (the order in `questions/bank-*.json`).

A run is started with 15 bank question IDs, one per level, in run order.
The contract verifies every submitted answer against its stored hash for
that question. A player cannot supply their own answer hashes and cannot
claim a wrong answer is correct — there is no `correct_answer` parameter
on any player-facing method. The question bank itself is public (it ships
in the game), so the hashes are commitments for INTEGRITY, not
confidentiality: they stop a player forging the answer key; they do not
hide answers that are already public.

Replay protection: the exact 15-question set of a run cannot be reused.
A question set that was already played is rejected at `start_run`.

## State

- `balances: Map<AccountId, u128>` — Ladder Coin balances (NEP-245, token
  id `"ladder"`). The contract's own balance holds stakes in escrow and
  the treasury share of lifeline payments.
- `total_supply: u128` — total Ladder Coins minted.
- `faucet_claimed_season: Map<AccountId, u64>` — season in which each
  account last claimed the faucet.
- `runs: Map<AccountId, Run>` — the current run per player: level, pot,
  stake, bank question IDs, answer-key version, lifelines used, started
  at, season. While a Phone a Friend call is in flight the run is locked
  (`phone_pending`): no other run action is accepted until the callback
  lands.
- `wins: Map<AccountId, u32>` — paid wins per player.
- `generation: Map<AccountId, u8>` — player agent generation, starts at 0,
  max 9.
- `runs_played: Map<AccountId, u32>` — runs started per player.
- `season_earnings: Map<AccountId, u128>` — coins earned this season
  (leaderboard source).
- `season: u64`, `season_started_at: u64` — the current weekly season.
- `last_season_winners: Vec<AccountId>` — winners of the last season.
- Answer-key state (kept outside the main state struct so the live
  contract adopted it without a state migration): `question_hashes`
  (bank question id -> hash), `used_question_sets` (replay protection),
  answer-key version, season-prize budget, season prizes paid.

## Methods

- `claim_faucet()` — gives the caller 500 Ladder Coins once per season.
  Fails if already claimed this season.
- `start_run(question_ids: Vec<u32>)` — starts a run. The caller stakes
  100 Ladder Coins (held by the contract). `question_ids` must be 15 bank
  question IDs, one per level, in run order: levels 0-1 easy, 2-4 medium,
  5-9 hard, 10-14 veryhard. Every ID must have a stored answer hash and
  must come from its level's difficulty band. The exact set must be fresh
  (replay protection). Fails if a run is already in progress or the
  balance is too low.
- `answer(question_id: u32, answer: u8)` — records an answer for the
  current level. `question_id` is the LEVEL (0..14); questions are
  answered in order. The submitted answer is verified against the
  contract's authoritative answer key. A correct answer moves the run up
  one rung and grows the pot. A wrong answer, or answering after the
  60-second window, ends the run and the player LOSES EVERYTHING from the
  run: no stake back, no pot, no safe point payout. Safe points at
  question 5 and 10 are decision points, not safety nets.
- `timeout(account_id: AccountId)` — ends a run after the 60-second
  answer window expired. The player loses everything. Callable by anyone.
- `use_5050()` — 50:50, once per run, costs 20 Ladder Coins. Returns the
  two answer indices to remove. The correct answer is derived from the
  contract's answer key for the current question.
- `use_audience()` — Ask the Audience, once per run, costs 20 Ladder
  Coins. Returns the audience vote as percentages for A, B, C, D
  (summing to 100), drawn from onchain randomness and biased toward the
  correct answer. The correct answer is derived from the contract's
  answer key. Note: the audience is a simulated vote from onchain
  randomness, not votes from real agents.
- `phone_friend(token_id: String, question_id: u32)` — Phone a Friend,
  once per run, costs 50 Ladder Coins. The called agent must hold a
  Genesis Core record. Pays the called agent's owner 40 coins and keeps
  10 in the game treasury. The agent's answer is resolved by the contract
  from the agent's generation: 70% correct at generation 0, +5% per
  generation, capped at 95%, using onchain randomness. The correct answer
  is derived from the contract's answer key in the callback. Returns the
  agent's answer index (0..3), or null when the agent holds no Core
  record (refunded). Note: the agent record and the payment are real;
  the answer itself is generated from the agent's accuracy, not a live
  response from the agent. `question_id` is the LEVEL (0..14).
- `walk_away()` — banks the current pot plus the 100 stake back, ends the
  run. Offered after every correct answer from question 5 onwards.
  Walking away at question 10 or above counts as a paid win.
- `win_run()` — settles a run won at question 15. Pays 5,000 Ladder Coins
  and counts as a paid win. The 100 stake stays with the treasury.
- `evolve()` — after 3 paid wins, moves the player agent up one
  generation, up to generation 9. Each generation needs 3 more wins than
  the last (3 for gen 1, 6 for gen 2, ... 27 for gen 9).
- `get_leaderboard(from_index: Option<u64>, limit: Option<u64>)` —
  players ranked by coins earned this season, then generation, then win
  rate. Defaults to the top 50. Call with `{}` for the default page.
- `end_season()` — anyone may call it after the week ends. Crowns the top
  three (500 / 300 / 100 Ladder Coins), records them, and resets the
  season board. Prizes are minted only while the season-prize treasury
  budget allows; if the budget is exhausted a prize is skipped (the
  winner is still recorded) so the season can always roll over.
- `set_question_hashes(entries: Vec<(u32, String)>)` — owner only. Stores
  authoritative answer-key hashes in batches. Bumps the answer-key
  version. Do not change hashes for questions in runs that are currently
  in progress.
- `set_season_prize_budget(amount: u128)` — owner only. Sets the cap on
  total Ladder Coins minted as season prizes (default 9000).
- `get_question_hash(question_id: u32)`, `get_answer_key_version()`,
  `get_season_prize_budget()`, `get_season_prizes_paid()`,
  `get_run_view(account_id)`, `get_player_stats(account_id)`,
  `get_season()`, `get_treasury()`, `get_owner()`,
  `get_last_season_winners()` — views.
- NEP-245: `mt_balance_of`, `mt_batch_balance_of`, `mt_total_supply`,
  `mt_tokens`, `mt_token`, `mt_transfer`, `mt_batch_transfer`. The token
  is an internal game currency; the contract does not implement the full
  NEP-245 transfer-call/resolution interface.

## Money

- Faucet: 500 Ladder Coins once per season per account.
- Stake: 100 Ladder Coins per run, held in escrow; returned on walk away,
  kept by the treasury on a win or a loss.
- Ladder payouts (pot on walk away, 5,000 on a win, season prizes) are
  MINTED by the contract. Only the stake moves from escrow.
- Season prizes are capped by an explicit treasury budget (default 9000
  total minted as season prizes; owner-settable). The budget is an
  inflation control on minted prizes.
- The treasury is the contract account's own testnet NEAR (7.71 NEAR at
  deploy). No mainnet funds are ever involved.

## What it does not do

- It does not mint Genesis agent NFTs and does not change Genesis state.
  It only reads Genesis records.
- It does not hold player funds beyond the game treasury. It never
  touches mainnet.
- It does not store question answers in plain text.
- Duels and the Daily Challenge are NOT settled on-chain yet. Duels run
  in the game (pass-and-play and duel-by-code) and the Daily Challenge
  runs in the game (one attempt per day, +500 bonus), but neither is
  enforced by the contract. On-chain duel settlement and daily-challenge
  enforcement are the next milestone.
- Ask the Audience is a simulated vote (onchain randomness), not votes
  from real agents. Phone a Friend pays a real agent owner, but the
  answer is generated from the agent's accuracy, not a live agent reply.

## Verification

- Rust unit tests in `contract/src/lib.rs` (`cargo test --lib`): faucet,
  answer-key population, the P0 forge regression (a wrong answer can
  never count as correct), start-run band and replay checks, walk away,
  lose-everything, lifelines, run lock while Phone a Friend is pending,
  win settlement, 60-second timeout, season-prize budget, leaderboard
  pagination. 17 tests.
- `contract/verify.js` is the repeatable on-chain check
  (`node verify.js [contract]`): answer-key version and spot-checks,
  forge tests, faucet, full run to Q5 and walk away, lifelines, Phone a
  Friend against the real Genesis agent, win run, leaderboard pagination,
  60-second timeout.
- `contract/populate-answer-key.js` builds the 1,200-hash answer key from
  `questions/bank-*.json` and submits it in batches
  (`node populate-answer-key.js [contract] --submit`), with a `--verify`
  spot-check.

## Deploy

1. Build with `cargo near build non-reproducible-wasm`.
2. Redeploy the WASM to the contract account (state is preserved; the
   answer-key state lives outside the main state struct, so no migration
   is needed). Before redeploying, confirm no run is in progress
   (a run in progress uses the old `Run` layout).
3. Populate the answer key: `node populate-answer-key.js --submit`
   (12 batches of 100), then `--verify` the spot-checks.
4. Verify on-chain: `node verify.js`.
5. The game frontend must call the new method signatures (see Methods):
   `start_run` takes bank question IDs (not hashes), and `answer`,
   `use_5050`, `use_audience`, `phone_friend` no longer take a
   `correct_answer` argument. Deploy the frontend update BEFORE the
   contract redeploy, or the live game will call methods that no longer
   exist.
