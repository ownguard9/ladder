//! Agent's Ladder — onchain game economy for the NEAR hackathon.
//!
//! The Ladder Coins economy settles onchain. Ladder Coins are a NEP-245
//! multi token minted by this contract (1 coin = 0.001 testnet NEAR for
//! accounting). The contract reads Genesis agent records from the Genesis
//! Core (`core.genesisdemo354551.testnet`) and never duplicates or changes
//! Genesis state.
//!
//! Core rules (corrected spec):
//! - A wrong answer, or a 60-second timeout without an answer, ends the run
//!   and the player LOSES EVERYTHING from the run: no stake back, no pot,
//!   no safe point payout. Safe points at question 5 and 10 are decision
//!   points, not safety nets.
//! - Walk away banks the current pot PLUS the 100 stake back. Offered after
//!   every correct answer from question 5 onwards. Walking away at question
//!   10 or above counts as a paid win.
//! - Timer is 60 seconds per question, NOT 30.
//! - Win at question 15 pays 5,000 Ladder Coins and counts as a paid win.
//! - Question answers are committed by hash at run start (SHA-256 of
//!   "question_id:correct_answer:salt"), so the contract never stores
//!   answers in plain text and a player cannot read them from state.

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::store::{LookupMap, UnorderedMap};
use near_sdk::{
    env, near, AccountId, Gas, NearToken, PanicOnDefault, Promise, PromiseError,
};

/// The single Ladder Coin token id (NEP-245).
pub const LADDER_TOKEN_ID: &str = "ladder";
/// The Genesis Core contract we read agent records from (never write).
pub const GENESIS_CORE: &str = "core.genesisdemo354551.testnet";

/// Ladder rung values, question 1..15.
pub const LADDER: [u128; 15] = [
    10, 20, 30, 50, 80, 120, 180, 250, 350, 500, 700, 1000, 1500, 2500, 5000,
];
/// Stake to start a run.
pub const STAKE: u128 = 100;
/// Faucet grant, once per season per account.
pub const FAUCET_AMOUNT: u128 = 500;
/// Cost of 50:50 and Ask the Audience, each once per run.
pub const LIFELINE_COST: u128 = 20;
/// Cost of Phone a Friend. 40 goes to the called agent's owner, 10 stays
/// in the game treasury.
pub const PHONE_FRIEND_COST: u128 = 50;
pub const PHONE_FRIEND_PAY: u128 = 40;
/// Seconds per question. 60, NOT 30.
pub const QUESTION_TIME_LIMIT_NS: u64 = 60_000_000_000;
/// One weekly season, in nanoseconds.
pub const SEASON_DURATION_NS: u64 = 7 * 24 * 60 * 60 * 1_000_000_000;
/// Season prizes for the top three: 1st, 2nd, 3rd.
pub const SEASON_PRIZES: [u128; 3] = [500, 300, 100];
/// Paid wins needed to evolve one generation.
pub const WINS_TO_EVOLVE: u32 = 3;
/// Final generation.
pub const MAX_GENERATION: u8 = 9;

/// One player's run in progress.
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Run {
    /// Questions answered correctly so far (0..15).
    pub level: u8,
    /// Current pot in Ladder Coins (ladder value at `level`).
    #[schemars(with = "String")]
    pub pot: U128,
    /// Stake held by the contract for this run.
    #[schemars(with = "String")]
    pub stake: U128,
    /// Block timestamp (ns) when the run started.
    pub started_at: u64,
    /// Block timestamp (ns) of the last correct answer (or run start).
    pub last_answer_at: u64,
    /// SHA-256 hex commitments, one per question, of
    /// "question_id:correct_answer:salt".
    pub question_hashes: Vec<String>,
    /// Per-run salt used in the commitments (public; reveals nothing
    /// without the correct answers).
    pub salt: String,
    pub used_5050: bool,
    pub used_audience: bool,
    pub used_phone: bool,
    /// True while a Phone a Friend cross-contract call is in flight.
    pub phone_pending: bool,
    pub season: u64,
}

/// Outcome of an `answer` call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct AnswerResult {
    pub correct: bool,
    pub level: u8,
    #[schemars(with = "String")]
    pub pot: U128,
    /// "correct" | "wrong_lost_everything" | "timeout_lost_everything" |
    /// "won_call_win_run"
    pub outcome: String,
}

/// One leaderboard row for the current season.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct LeaderboardEntry {
    pub account_id: AccountId,
    #[schemars(with = "String")]
    pub earned: U128,
    pub generation: u8,
    pub wins: u32,
    pub runs_played: u32,
}

/// Public player stats.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct PlayerStats {
    pub wins: u32,
    pub generation: u8,
    pub runs_played: u32,
    #[schemars(with = "String")]
    pub earned_season: U128,
    #[schemars(with = "String")]
    pub balance: U128,
}

/// Minimal NEP-245 token metadata.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<u8>,
}

/// Minimal NEP-245 token record.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct MtToken {
    pub token_id: String,
    pub owner_id: AccountId,
    pub metadata: Option<TokenMetadata>,
}

/// A Genesis agent record, as returned by the Core's `get_agent`.
/// Only the fields this contract needs; serde ignores the rest.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct CoreAgent {
    pub token_id: String,
    pub owner_id: AccountId,
    pub generation: u32,
    pub status: CoreAgentStatus,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub enum CoreAgentStatus {
    Alive,
    Dead,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub treasury: AccountId,
    /// Ladder Coin balances (NEP-245, single token id "ladder").
    /// The contract's own balance (key = current account) holds stakes in
    /// escrow and the treasury share of lifeline payments.
    pub balances: LookupMap<AccountId, u128>,
    pub total_supply: u128,
    /// Season number in which each account last claimed the faucet.
    pub faucet_claimed_season: LookupMap<AccountId, u64>,
    pub runs: LookupMap<AccountId, Run>,
    pub wins: LookupMap<AccountId, u32>,
    pub generation: LookupMap<AccountId, u8>,
    pub runs_played: LookupMap<AccountId, u32>,
    /// Coins earned this season, per account (leaderboard source).
    pub season_earnings: UnorderedMap<AccountId, u128>,
    pub season: u64,
    pub season_started_at: u64,
    pub last_season_winners: Vec<AccountId>,
}

fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0f) as usize] as char);
    }
    s
}

/// SHA-256 hex of "question_id:correct_answer:salt".
fn question_hash(question_id: u32, correct_answer: u8, salt: &str) -> String {
    let input = format!("{}:{}:{}", question_id, correct_answer, salt);
    to_hex(&env::sha256(input.as_bytes()))
}

/// Onchain randomness from the block seed: a number in [0, n).
fn rand_below(n: u64) -> u64 {
    let seed = env::random_seed();
    let mut bytes = [0u8; 8];
    let take = seed.len().min(8);
    bytes[..take].copy_from_slice(&seed[..take]);
    u64::from_le_bytes(bytes) % n
}

#[near]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, treasury: AccountId) -> Self {
        Self {
            owner_id,
            treasury,
            balances: LookupMap::new(b"b"),
            total_supply: 0,
            faucet_claimed_season: LookupMap::new(b"f"),
            runs: LookupMap::new(b"r"),
            wins: LookupMap::new(b"w"),
            generation: LookupMap::new(b"g"),
            runs_played: LookupMap::new(b"p"),
            season_earnings: UnorderedMap::new(b"e"),
            season: 1,
            season_started_at: env::block_timestamp(),
            last_season_winners: vec![],
        }
    }

    // ─── Internal token helpers ──────────────────────────────────

    fn balance_of(&self, account_id: &AccountId) -> u128 {
        self.balances.get(account_id).copied().unwrap_or(0)
    }

    fn mint(&mut self, account_id: &AccountId, amount: u128) {
        let b = self.balance_of(account_id) + amount;
        self.balances.insert(account_id.clone(), b);
        self.total_supply += amount;
    }

    /// Move `amount` from `from` to `to`. Panics on insufficient balance.
    fn move_coins(&mut self, from: &AccountId, to: &AccountId, amount: u128) {
        assert!(
            self.balance_of(from) >= amount,
            "Insufficient Ladder Coin balance"
        );
        let fb = self.balance_of(from) - amount;
        let tb = self.balance_of(to) + amount;
        self.balances.insert(from.clone(), fb);
        self.balances.insert(to.clone(), tb);
    }

    fn record_earnings(&mut self, account_id: &AccountId, amount: u128) {
        let e = self.season_earnings.get(account_id).copied().unwrap_or(0) + amount;
        self.season_earnings.insert(account_id.clone(), e);
    }

    fn get_run(&self, account_id: &AccountId) -> Run {
        self.runs
            .get(account_id)
            .cloned()
            .expect("No run in progress for this account")
    }

    fn get_wins(&self, account_id: &AccountId) -> u32 {
        self.wins.get(account_id).copied().unwrap_or(0)
    }

    fn get_generation(&self, account_id: &AccountId) -> u8 {
        self.generation.get(account_id).copied().unwrap_or(0)
    }

    fn get_runs_played(&self, account_id: &AccountId) -> u32 {
        self.runs_played.get(account_id).copied().unwrap_or(0)
    }

    /// True if the run's 60-second answer window has expired.
    fn run_timed_out(&self, run: &Run) -> bool {
        run.level < 15
            && env::block_timestamp().saturating_sub(run.last_answer_at)
                > QUESTION_TIME_LIMIT_NS
    }

    /// End a run as a loss: the player loses everything from the run.
    /// The stake stays with the contract (treasury); the pot was never
    /// paid out, so there is nothing to move. No safe point payout.
    fn end_run_lost(&mut self, account_id: &AccountId) {
        self.runs.remove(account_id);
    }

    /// Verify the committed correct answer for a question in this run.
    fn verify_answer(&self, run: &Run, question_id: u32, correct_answer: u8) {
        let expected = run
            .question_hashes
            .get(question_id as usize)
            .expect("Invalid question id");
        let actual = question_hash(question_id, correct_answer, &run.salt);
        assert_eq!(&actual, expected, "Answer commitment mismatch");
    }

    // ─── NEP-245 multi token ─────────────────────────────────────

    /// Balance of `token_id` for `account_id`.
    pub fn mt_balance_of(&self, account_id: AccountId, token_id: String) -> U128 {
        assert_eq!(token_id, LADDER_TOKEN_ID, "Unknown token");
        U128(self.balance_of(&account_id))
    }

    pub fn mt_batch_balance_of(
        &self,
        account_ids: Vec<AccountId>,
        token_ids: Vec<String>,
    ) -> Vec<U128> {
        assert_eq!(
            account_ids.len(),
            token_ids.len(),
            "Accounts and tokens length mismatch"
        );
        account_ids
            .iter()
            .zip(token_ids.iter())
            .map(|(a, t)| {
                assert_eq!(t, LADDER_TOKEN_ID, "Unknown token");
                U128(self.balance_of(a))
            })
            .collect()
    }

    pub fn mt_total_supply(&self, token_id: String) -> U128 {
        assert_eq!(token_id, LADDER_TOKEN_ID, "Unknown token");
        U128(self.total_supply)
    }

    pub fn mt_tokens(&self, _from_index: Option<U128>, _limit: Option<u64>) -> Vec<String> {
        vec![LADDER_TOKEN_ID.to_string()]
    }

    pub fn mt_token(&self, token_id: String) -> Option<MtToken> {
        if token_id != LADDER_TOKEN_ID {
            return None;
        }
        Some(MtToken {
            token_id: LADDER_TOKEN_ID.to_string(),
            owner_id: env::current_account_id(),
            metadata: Some(TokenMetadata {
                title: Some("Ladder Coin".to_string()),
                description: Some(
                    "The Agent's Ladder game coin. 1 coin = 0.001 testnet NEAR for accounting."
                        .to_string(),
                ),
                symbol: Some("LADDER".to_string()),
                decimals: Some(0),
            }),
        })
    }

    /// Transfer Ladder Coins to another account. Requires 1 yoctoNEAR.
    pub fn mt_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: String,
        amount: U128,
        _memo: Option<String>,
    ) {
        assert_eq!(token_id, LADDER_TOKEN_ID, "Unknown token");
        assert_eq!(
            env::attached_deposit(),
            NearToken::from_yoctonear(1),
            "Requires 1 yoctoNEAR"
        );
        let sender = env::predecessor_account_id();
        self.move_coins(&sender, &receiver_id, amount.0);
    }

    pub fn mt_batch_transfer(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<String>,
        amounts: Vec<U128>,
        _memo: Option<String>,
    ) {
        assert_eq!(
            token_ids.len(),
            amounts.len(),
            "Tokens and amounts length mismatch"
        );
        assert_eq!(
            env::attached_deposit(),
            NearToken::from_yoctonear(1),
            "Requires 1 yoctoNEAR"
        );
        let sender = env::predecessor_account_id();
        for (t, a) in token_ids.iter().zip(amounts.iter()) {
            assert_eq!(t, LADDER_TOKEN_ID, "Unknown token");
            self.move_coins(&sender, &receiver_id, a.0);
        }
    }

    // ─── Faucet ──────────────────────────────────────────────────

    /// Gives the caller 500 Ladder Coins once per season.
    /// Fails if already claimed this season.
    pub fn claim_faucet(&mut self) -> U128 {
        let caller = env::predecessor_account_id();
        let claimed = self.faucet_claimed_season.get(&caller).copied().unwrap_or(0);
        assert_ne!(claimed, self.season, "Faucet already claimed this season");
        self.faucet_claimed_season.insert(caller.clone(), self.season);
        self.mint(&caller, FAUCET_AMOUNT);
        U128(self.balance_of(&caller))
    }

    // ─── Runs ────────────────────────────────────────────────────

    /// Starts a run. The caller stakes 100 Ladder Coins (held by the
    /// contract). `question_hashes` must be 15 SHA-256 hex commitments,
    /// one per question, of "question_id:correct_answer:salt".
    /// Fails if a run is already in progress or the balance is too low.
    pub fn start_run(&mut self, question_hashes: Vec<String>, salt: String) -> Run {
        let caller = env::predecessor_account_id();
        assert!(
            self.runs.get(&caller).is_none(),
            "A run is already in progress"
        );
        assert_eq!(question_hashes.len(), 15, "Need exactly 15 question hashes");
        assert!(
            self.balance_of(&caller) >= STAKE,
            "Need at least 100 Ladder Coins to stake"
        );
        let contract = env::current_account_id();
        self.move_coins(&caller, &contract, STAKE);
        let now = env::block_timestamp();
        let run = Run {
            level: 0,
            pot: U128(0),
            stake: U128(STAKE),
            started_at: now,
            last_answer_at: now,
            question_hashes,
            salt,
            used_5050: false,
            used_audience: false,
            used_phone: false,
            phone_pending: false,
            season: self.season,
        };
        self.runs.insert(caller.clone(), run.clone());
        let played = self.get_runs_played(&caller) + 1;
        self.runs_played.insert(caller.clone(), played);
        run
    }

    /// Records an answer for the current level. `question_id` must equal
    /// the current level (questions are answered in order). `correct_answer`
    /// is verified against the commitment made at run start.
    /// A correct answer moves the run up one rung and grows the pot.
    /// A wrong answer, or answering after the 60-second window, ends the
    /// run and the player LOSES EVERYTHING from the run: no stake back,
    /// no pot, no safe point payout.
    pub fn answer(
        &mut self,
        question_id: u32,
        answer: u8,
        correct_answer: u8,
    ) -> AnswerResult {
        let caller = env::predecessor_account_id();
        let mut run = self.get_run(&caller);
        assert!(run.level < 15, "Run already won, call win_run");
        assert_eq!(
            question_id as usize, run.level as usize,
            "Answer the questions in order"
        );
        assert!(answer < 4, "Answer must be 0..3");
        assert!(correct_answer < 4, "Correct answer must be 0..3");
        if self.run_timed_out(&run) {
            self.end_run_lost(&caller);
            return AnswerResult {
                correct: false,
                level: run.level,
                pot: U128(0),
                outcome: "timeout_lost_everything".to_string(),
            };
        }
        self.verify_answer(&run, question_id, correct_answer);
        if answer == correct_answer {
            run.level += 1;
            run.pot = U128(LADDER[(run.level - 1) as usize]);
            run.last_answer_at = env::block_timestamp();
            let outcome = if run.level == 15 {
                "won_call_win_run".to_string()
            } else {
                "correct".to_string()
            };
            let result = AnswerResult {
                correct: true,
                level: run.level,
                pot: run.pot,
                outcome,
            };
            self.runs.insert(caller.clone(), run);
            result
        } else {
            self.end_run_lost(&caller);
            AnswerResult {
                correct: false,
                level: run.level,
                pot: U128(0),
                outcome: "wrong_lost_everything".to_string(),
            }
        }
    }

    /// Ends a run after the 60-second answer window expired. The player
    /// loses everything from the run. Callable by anyone.
    pub fn timeout(&mut self, account_id: AccountId) -> String {
        let run = self.get_run(&account_id);
        assert!(run.level < 15, "Run already won, call win_run");
        assert!(
            self.run_timed_out(&run),
            "The 60-second answer window has not expired yet"
        );
        self.end_run_lost(&account_id);
        "timeout_lost_everything".to_string()
    }

    // ─── Lifelines ───────────────────────────────────────────────

    /// 50:50, once per run, costs 20 Ladder Coins. Returns the two answer
    /// indices to remove (two of the three wrong answers, picked by
    /// onchain randomness). `correct_answer` is verified against the
    /// run commitment.
    pub fn use_5050(&mut self, correct_answer: u8) -> Vec<u8> {
        let caller = env::predecessor_account_id();
        let mut run = self.get_run(&caller);
        assert!(run.level < 15, "Run already won");
        assert!(!run.used_5050, "50:50 already used this run");
        assert!(!self.run_timed_out(&run), "Answer window expired");
        assert!(correct_answer < 4, "Correct answer must be 0..3");
        self.verify_answer(&run, run.level as u32, correct_answer);
        assert!(
            self.balance_of(&caller) >= LIFELINE_COST,
            "Need at least 20 Ladder Coins"
        );
        let contract = env::current_account_id();
        self.move_coins(&caller, &contract, LIFELINE_COST);
        run.used_5050 = true;
        self.runs.insert(caller.clone(), run);
        let wrong: Vec<u8> = (0u8..4).filter(|&a| a != correct_answer).collect();
        let drop = rand_below(3) as usize;
        wrong
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i != drop)
            .map(|(_, a)| a)
            .collect()
    }

    /// Ask the Audience, once per run, costs 20 Ladder Coins. Returns the
    /// audience vote as percentages for answers A, B, C, D (summing to
    /// 100), drawn from onchain randomness and biased toward the correct
    /// answer. `correct_answer` is verified against the run commitment.
    pub fn use_audience(&mut self, correct_answer: u8) -> Vec<u8> {
        let caller = env::predecessor_account_id();
        let mut run = self.get_run(&caller);
        assert!(run.level < 15, "Run already won");
        assert!(!run.used_audience, "Audience already used this run");
        assert!(!self.run_timed_out(&run), "Answer window expired");
        assert!(correct_answer < 4, "Correct answer must be 0..3");
        self.verify_answer(&run, run.level as u32, correct_answer);
        assert!(
            self.balance_of(&caller) >= LIFELINE_COST,
            "Need at least 20 Ladder Coins"
        );
        let contract = env::current_account_id();
        self.move_coins(&caller, &contract, LIFELINE_COST);
        run.used_audience = true;
        self.runs.insert(caller.clone(), run);
        let correct_share = 55 + rand_below(21); // 55..75 for the right answer
        let mut rest = 100 - correct_share;
        let mut votes = [0u64; 4];
        votes[correct_answer as usize] = correct_share;
        for a in 0..4 {
            if a as u8 == correct_answer {
                continue;
            }
            if a == 3 {
                votes[a] = rest;
            } else {
                let share = rand_below(rest + 1);
                votes[a] = share;
                rest -= share;
            }
        }
        votes.iter().map(|&v| v as u8).collect()
    }

    /// Phone a Friend, once per run, costs 50 Ladder Coins. The called
    /// agent must hold a Genesis Core record. Pays the called agent's
    /// owner 40 coins and keeps 10 in the game treasury. The agent's
    /// accuracy is 70% at generation 0, +5% per generation, capped at
    /// 95%, using onchain randomness. Returns the agent's answer index
    /// (0..3), or null when the agent holds no Core record (refunded).
    /// `correct_answer` is verified against the run commitment.
    pub fn phone_friend(
        &mut self,
        token_id: String,
        question_id: u32,
        correct_answer: u8,
    ) -> Promise {
        let caller = env::predecessor_account_id();
        let mut run = self.get_run(&caller);
        assert!(run.level < 15, "Run already won");
        assert!(!run.used_phone, "Phone a Friend already used this run");
        assert!(!run.phone_pending, "Phone a Friend already in progress");
        assert!(!self.run_timed_out(&run), "Answer window expired");
        assert_eq!(
            question_id as usize, run.level as usize,
            "Phone a Friend is for the current question"
        );
        assert!(correct_answer < 4, "Correct answer must be 0..3");
        self.verify_answer(&run, question_id, correct_answer);
        assert!(
            self.balance_of(&caller) >= PHONE_FRIEND_COST,
            "Need at least 50 Ladder Coins"
        );
        let contract = env::current_account_id();
        self.move_coins(&caller, &contract, PHONE_FRIEND_COST);
        run.phone_pending = true;
        self.runs.insert(caller.clone(), run);
        Promise::new(GENESIS_CORE.parse().unwrap())
            .function_call(
                "get_agent".to_string(),
                near_sdk::serde_json::json!({ "token_id": token_id })
                    .to_string()
                    .into_bytes(),
                NearToken::from_yoctonear(0),
                Gas::from_tgas(10),
            )
            .then(
                Promise::new(env::current_account_id()).function_call(
                    "phone_friend_callback".to_string(),
                    near_sdk::serde_json::json!({
                        "player_id": caller,
                        "token_id": token_id,
                        "question_id": question_id,
                        "correct_answer": correct_answer,
                    })
                    .to_string()
                    .into_bytes(),
                    NearToken::from_yoctonear(0),
                    Gas::from_tgas(20),
                ),
            )
    }

    /// Callback after the Genesis Core `get_agent` call in `phone_friend`.
    /// Private: only this contract can call it.
    #[private]
    pub fn phone_friend_callback(
        &mut self,
        player_id: AccountId,
        token_id: String,
        question_id: u32,
        correct_answer: u8,
        #[callback_result] agent: Result<Option<CoreAgent>, PromiseError>,
    ) -> Option<u8> {
        let contract = env::current_account_id();
        let mut run = match self.runs.get(&player_id) {
            Some(r) => r.clone(),
            None => return None,
        };
        run.phone_pending = false;
        let agent = match agent {
            Ok(Some(a)) if a.status == CoreAgentStatus::Alive => a,
            _ => {
                // No Core record (or dead agent, or failed call): refund.
                self.move_coins(&contract, &player_id, PHONE_FRIEND_COST);
                self.runs.insert(player_id.clone(), run);
                return None;
            }
        };
        run.used_phone = true;
        self.runs.insert(player_id.clone(), run);
        // 40 to the called agent's owner, 10 stays in the treasury.
        self.move_coins(&contract, &agent.owner_id, PHONE_FRIEND_PAY);
        let accuracy = (70u64 + 5u64 * agent.generation as u64).min(95);
        let roll = rand_below(100);
        if roll < accuracy {
            Some(correct_answer)
        } else {
            let wrong: Vec<u8> = (0u8..4).filter(|&a| a != correct_answer).collect();
            Some(wrong[rand_below(3) as usize])
        }
    }

    // ─── Settle ──────────────────────────────────────────────────

    /// Banks the current pot plus the 100 stake back, ends the run.
    /// Offered after every correct answer from question 5 onwards.
    /// Walking away at question 10 or above counts as a paid win.
    /// The pot is minted as winnings (ladder payouts are minted, per the
    /// spec); the stake is returned from the escrow held by the contract.
    pub fn walk_away(&mut self) -> U128 {
        let caller = env::predecessor_account_id();
        let run = self.get_run(&caller);
        assert!(run.level >= 5, "Walk away is offered from question 5 onwards");
        let banked = run.pot.0 + run.stake.0;
        let contract = env::current_account_id();
        self.mint(&caller, run.pot.0);
        self.move_coins(&contract, &caller, run.stake.0);
        if run.level >= 10 {
            let w = self.get_wins(&caller) + 1;
            self.wins.insert(caller.clone(), w);
        }
        self.record_earnings(&caller, banked);
        self.runs.remove(&caller);
        U128(banked)
    }

    /// Settles a run won at question 15. Pays 5,000 Ladder Coins and
    /// counts as a paid win. The 5,000 is minted as a ladder payout.
    /// The 100 stake stays with the treasury.
    pub fn win_run(&mut self) -> U128 {
        let caller = env::predecessor_account_id();
        let run = self.get_run(&caller);
        assert_eq!(run.level, 15, "Win is only settled at question 15");
        self.mint(&caller, LADDER[14]);
        let w = self.get_wins(&caller) + 1;
        self.wins.insert(caller.clone(), w);
        self.record_earnings(&caller, LADDER[14]);
        self.runs.remove(&caller);
        U128(LADDER[14])
    }

    /// After 3 paid wins, moves the player agent up one generation,
    /// up to generation 9. Each generation needs 3 more wins than the
    /// last (3 wins for gen 1, 6 for gen 2, ... 27 for gen 9).
    pub fn evolve(&mut self) -> u8 {
        let caller = env::predecessor_account_id();
        let gen = self.get_generation(&caller);
        assert!(gen < MAX_GENERATION, "Already at the final generation");
        let needed = WINS_TO_EVOLVE * (gen as u32 + 1);
        assert!(
            self.get_wins(&caller) >= needed,
            "Need more paid wins to evolve"
        );
        let next = gen + 1;
        self.generation.insert(caller.clone(), next);
        next
    }

    // ─── Seasons ─────────────────────────────────────────────────

    /// Players ranked by coins earned this season, then generation,
    /// then win rate (wins / runs played).
    pub fn get_leaderboard(&self) -> Vec<LeaderboardEntry> {
        let mut rows: Vec<LeaderboardEntry> = self
            .season_earnings
            .iter()
            .map(|(account_id, earned)| {
                let wins = self.get_wins(account_id);
                let played = self.get_runs_played(account_id);
                LeaderboardEntry {
                    account_id: account_id.clone(),
                    earned: U128(*earned),
                    generation: self.get_generation(&account_id),
                    wins,
                    runs_played: played,
                }
            })
            .collect();
        rows.sort_by(|a, b| {
            b.earned
                .0
                .cmp(&a.earned.0)
                .then(b.generation.cmp(&a.generation))
                .then(
                    (b.wins as u64 * 10_000 / b.runs_played.max(1) as u64)
                        .cmp(&(a.wins as u64 * 10_000 / a.runs_played.max(1) as u64)),
                )
        });
        rows.truncate(50);
        rows
    }

    /// Anyone may call it after the week ends. Crowns the top three
    /// (500 / 300 / 100 Ladder Coins), records them, and resets the
    /// season board.
    pub fn end_season(&mut self) -> Vec<AccountId> {
        assert!(
            env::block_timestamp().saturating_sub(self.season_started_at)
                >= SEASON_DURATION_NS,
            "The season has not ended yet"
        );
        let top = self.get_leaderboard();
        let mut winners: Vec<AccountId> = vec![];
        for (i, entry) in top.iter().take(3).enumerate() {
            self.mint(&entry.account_id, SEASON_PRIZES[i]);
            winners.push(entry.account_id.clone());
        }
        self.last_season_winners = winners.clone();
        self.season_earnings.clear();
        self.season += 1;
        self.season_started_at = env::block_timestamp();
        winners
    }

    // ─── Views ───────────────────────────────────────────────────

    pub fn get_run_view(&self, account_id: AccountId) -> Option<Run> {
        self.runs.get(&account_id).cloned()
    }

    pub fn get_season(&self) -> u64 {
        self.season
    }

    pub fn get_treasury(&self) -> AccountId {
        self.treasury.clone()
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn get_last_season_winners(&self) -> Vec<AccountId> {
        self.last_season_winners.clone()
    }

    pub fn get_player_stats(&self, account_id: AccountId) -> PlayerStats {
        PlayerStats {
            wins: self.get_wins(&account_id),
            generation: self.get_generation(&account_id),
            runs_played: self.get_runs_played(&account_id),
            earned_season: U128(self.season_earnings.get(&account_id).copied().unwrap_or(0)),
            balance: U128(self.balance_of(&account_id)),
        }
    }
}
