//! Agent's Ladder — onchain game economy for the NEAR hackathon.
//!
//! The Ladder Coins economy settles onchain. Ladder Coins are an
//! INTERNAL on-chain game currency: they are minted and moved by this
//! contract, they are NOT redeemable for NEAR, and they are NOT
//! NEAR-backed. 1 coin = 0.001 testnet NEAR for accounting only.
//! Testnet NEAR pays transaction costs and funds the account
//! infrastructure; players never need to understand any of that.
//! The contract reads Genesis agent records from the Genesis Core
//! (`core.genesisdemo354551.testnet`) and never duplicates or changes
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
//! - The contract is the AUTHORITY on the answer key. The owner stores
//!   SHA-256 hashes of the correct answers (hashes only — the answers
//!   themselves are never stored on-chain). A run is started with 15 bank
//!   question IDs; the contract verifies each submitted answer against
//!   its stored hash for that question. A player cannot supply their own
//!   answer hashes or claim a wrong answer is correct. The question bank
//!   itself is public (it ships in the game), so the hashes are
//!   commitments for INTEGRITY, not confidentiality: they stop a player
//!   forging the answer key, they do not hide answers that are already
//!   public.
//! - A question set (the exact 15 bank question IDs) cannot be reused:
//!   replay protection stops a player farming a set they already know.
//! - While a Phone a Friend cross-contract call is in flight, the run is
//!   locked: no other run action is accepted until the callback lands.
//! - Season prizes (top three: 500 / 300 / 100) are capped by an explicit
//!   treasury budget. Prizes are minted only while the budget allows.

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

/// Fixed salt for the authoritative answer-key hashes. The salt is public
/// (it lives in this source); the hashes are commitments for INTEGRITY,
/// not confidentiality — see the module docs.
pub const ANSWER_KEY_SALT: &str = "ladder-answer-key-v1";
/// Total questions in the bank: 4 difficulty bands x 300.
/// Bank question IDs: easy 0..299, medium 300..599, hard 600..899,
/// veryhard 900..1199.
pub const QUESTION_BANK_SIZE: u32 = 1200;
/// Default cap on total Ladder Coins minted as season prizes (10 seasons).
pub const DEFAULT_SEASON_PRIZE_BUDGET: u128 = 9000;

/// Storage prefixes / keys for the answer-key state. These live OUTSIDE
/// the `Contract` struct on purpose: the live contract adopted them
/// without a state migration, so a redeploy keeps every existing
/// balance, run, win and season intact.
const QUESTION_HASHES_PREFIX: &[u8] = b"qh";
const USED_SETS_PREFIX: &[u8] = b"us";
const ANSWER_KEY_VERSION_KEY: &[u8] = b"akv";
const SEASON_PRIZE_BUDGET_KEY: &[u8] = b"spb";
const SEASON_PRIZES_PAID_KEY: &[u8] = b"spp";

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
    /// Bank question IDs, one per level (0..14), in run order.
    /// The contract verifies answers against its own authoritative
    /// answer key for these IDs; the player never supplies hashes.
    pub question_ids: Vec<u32>,
    /// Answer-key version this run was started under.
    pub answer_key_version: u64,
    pub used_5050: bool,
    pub used_audience: bool,
    pub used_phone: bool,
    /// True while a Phone a Friend cross-contract call is in flight.
    /// While true the run is locked: no other run action is accepted.
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

// ─── Answer-key state (outside the Contract struct; see above) ───

/// Authoritative answer-key hashes: bank question id -> SHA-256 hex of
/// "question_id:correct_answer:ANSWER_KEY_SALT".
fn question_hashes_map() -> LookupMap<u32, String> {
    LookupMap::new(QUESTION_HASHES_PREFIX)
}

/// Hashes of question sets already used (replay protection).
fn used_sets_map() -> LookupMap<String, bool> {
    LookupMap::new(USED_SETS_PREFIX)
}

fn read_u64(key: &[u8], default: u64) -> u64 {
    match env::storage_read(key) {
        Some(bytes) => {
            let mut arr = [0u8; 8];
            let len = bytes.len().min(8);
            arr[..len].copy_from_slice(&bytes[..len]);
            u64::from_le_bytes(arr)
        }
        None => default,
    }
}

fn write_u64(key: &[u8], value: u64) {
    env::storage_write(key, &value.to_le_bytes());
}

fn read_u128(key: &[u8], default: u128) -> u128 {
    match env::storage_read(key) {
        Some(bytes) => {
            let mut arr = [0u8; 16];
            let len = bytes.len().min(16);
            arr[..len].copy_from_slice(&bytes[..len]);
            u128::from_le_bytes(arr)
        }
        None => default,
    }
}

fn write_u128(key: &[u8], value: u128) {
    env::storage_write(key, &value.to_le_bytes());
}

fn read_answer_key_version() -> u64 {
    read_u64(ANSWER_KEY_VERSION_KEY, 0)
}

fn write_answer_key_version(v: u64) {
    write_u64(ANSWER_KEY_VERSION_KEY, v);
}

fn read_season_prize_budget() -> u128 {
    read_u128(SEASON_PRIZE_BUDGET_KEY, DEFAULT_SEASON_PRIZE_BUDGET)
}

fn write_season_prize_budget(v: u128) {
    write_u128(SEASON_PRIZE_BUDGET_KEY, v);
}

fn read_season_prizes_paid() -> u128 {
    read_u128(SEASON_PRIZES_PAID_KEY, 0)
}

fn write_season_prizes_paid(v: u128) {
    write_u128(SEASON_PRIZES_PAID_KEY, v);
}

/// The correct answer (0..3) for a bank question, derived by testing
/// each candidate against the stored authoritative hash. None when the
/// question has no stored hash.
fn correct_answer_for(question_id: u32) -> Option<u8> {
    let map = question_hashes_map();
    let expected = map.get(&question_id)?;
    for a in 0u8..4 {
        if question_hash(question_id, a, ANSWER_KEY_SALT) == *expected {
            return Some(a);
        }
    }
    None
}

/// True when `answer` is the correct answer for `question_id`, checked
/// against the stored authoritative hash. Panics when the question has
/// no stored hash.
fn is_correct_answer(question_id: u32, answer: u8) -> bool {
    let map = question_hashes_map();
    let expected = map
        .get(&question_id)
        .expect("Question has no stored answer hash");
    question_hash(question_id, answer, ANSWER_KEY_SALT) == *expected
}

/// Difficulty band (inclusive lower, exclusive upper bank question id)
/// for a run level: levels 0-1 easy, 2-4 medium, 5-9 hard, 10-14
/// veryhard.
fn level_band(level: u8) -> (u32, u32) {
    match level {
        0..=1 => (0, 300),
        2..=4 => (300, 600),
        5..=9 => (600, 900),
        _ => (900, 1200),
    }
}

/// Hash of a question set (sorted ids, comma-joined) for replay
/// protection.
fn question_set_hash(question_ids: &[u32]) -> String {
    let mut sorted = question_ids.to_vec();
    sorted.sort_unstable();
    let input = sorted
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(",");
    to_hex(&env::sha256(input.as_bytes()))
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

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner can call this"
        );
    }

    /// Panics when a Phone a Friend call is in flight on this run.
    /// While a cross-contract call is pending the run is locked: no
    /// other run action is accepted until the callback lands.
    fn assert_run_unlocked(run: &Run) {
        assert!(
            !run.phone_pending,
            "A Phone a Friend call is in progress"
        );
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
                    "Ladder Coin is an internal on-chain game currency. It is not redeemable for NEAR. 1 coin = 0.001 testnet NEAR for accounting only."
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

    // ─── Answer key (owner only) ─────────────────────────────────

    /// Owner-only: store authoritative answer-key hashes, in batches.
    /// Each entry is (bank question id, SHA-256 hex of
    /// "question_id:correct_answer:ANSWER_KEY_SALT"). The caller
    /// computes the hashes off-chain; the contract never sees the
    /// answers in plain text. Bumps the answer-key version. May be
    /// called repeatedly to populate the key in chunks. Do not change
    /// hashes for questions in runs that are currently in progress.
    pub fn set_question_hashes(&mut self, entries: Vec<(u32, String)>) {
        self.assert_owner();
        assert!(!entries.is_empty(), "No entries");
        let mut map = question_hashes_map();
        for (qid, hash) in entries {
            assert!(qid < QUESTION_BANK_SIZE, "Question id out of range");
            assert_eq!(hash.len(), 64, "Hash must be 64 hex chars");
            map.insert(qid, hash);
        }
        write_answer_key_version(read_answer_key_version() + 1);
    }

    /// The stored authoritative hash for a bank question, if any.
    pub fn get_question_hash(&self, question_id: u32) -> Option<String> {
        question_hashes_map().get(&question_id).cloned()
    }

    /// The current answer-key version (0 = key not yet populated).
    pub fn get_answer_key_version(&self) -> u64 {
        read_answer_key_version()
    }

    /// Owner-only: set the cap on total Ladder Coins minted as season
    /// prizes. Defaults to 9000 (10 seasons at 500+300+100).
    pub fn set_season_prize_budget(&mut self, amount: u128) {
        self.assert_owner();
        write_season_prize_budget(amount);
    }

    /// The cap on total Ladder Coins minted as season prizes.
    pub fn get_season_prize_budget(&self) -> U128 {
        U128(read_season_prize_budget())
    }

    /// Total Ladder Coins minted as season prizes so far.
    pub fn get_season_prizes_paid(&self) -> U128 {
        U128(read_season_prizes_paid())
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
    /// contract). `question_ids` must be 15 bank question IDs, one per
    /// level, in run order: levels 0-1 easy (0..299), 2-4 medium
    /// (300..599), 5-9 hard (600..899), 10-14 veryhard (900..1199).
    /// The contract verifies every ID against its authoritative answer
    /// key — the player supplies question IDs, never answer hashes, so
    /// the answer key cannot be forged. The exact question set must be
    /// fresh (replay protection). Fails if a run is already in progress
    /// or the balance is too low.
    pub fn start_run(&mut self, question_ids: Vec<u32>) -> Run {
        let caller = env::predecessor_account_id();
        assert!(
            self.runs.get(&caller).is_none(),
            "A run is already in progress"
        );
        assert_eq!(question_ids.len(), 15, "Need exactly 15 question ids");
        let key_map = question_hashes_map();
        for (level, qid) in question_ids.iter().enumerate() {
            assert!(
                key_map.get(qid).is_some(),
                "Question has no stored answer hash"
            );
            let (lo, hi) = level_band(level as u8);
            assert!(
                *qid >= lo && *qid < hi,
                "Question id is not from the level's difficulty band"
            );
        }
        let set_hash = question_set_hash(&question_ids);
        let mut used = used_sets_map();
        assert!(
            used.get(&set_hash).is_none(),
            "This question set was already used"
        );
        used.insert(set_hash, true);
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
            question_ids,
            answer_key_version: read_answer_key_version(),
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

    /// Records an answer for the current level. `question_id` is the
    /// LEVEL (0..14); questions are answered in order. The submitted
    /// answer is verified against the contract's authoritative answer
    /// key for the question at this level — the player does not supply
    /// a correct answer, so the answer key cannot be forged.
    /// A correct answer moves the run up one rung and grows the pot.
    /// A wrong answer, or answering after the 60-second window, ends the
    /// run and the player LOSES EVERYTHING from the run: no stake back,
    /// no pot, no safe point payout.
    pub fn answer(&mut self, question_id: u32, answer: u8) -> AnswerResult {
        let caller = env::predecessor_account_id();
        let mut run = self.get_run(&caller);
        Self::assert_run_unlocked(&run);
        assert!(run.level < 15, "Run already won, call win_run");
        assert_eq!(
            question_id as usize, run.level as usize,
            "Answer the questions in order"
        );
        assert!(answer < 4, "Answer must be 0..3");
        if self.run_timed_out(&run) {
            self.end_run_lost(&caller);
            return AnswerResult {
                correct: false,
                level: run.level,
                pot: U128(0),
                outcome: "timeout_lost_everything".to_string(),
            };
        }
        let bank_qid = run.question_ids[run.level as usize];
        if is_correct_answer(bank_qid, answer) {
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
    /// onchain randomness). The correct answer is derived from the
    /// contract's authoritative answer key for the current question.
    pub fn use_5050(&mut self) -> Vec<u8> {
        let caller = env::predecessor_account_id();
        let mut run = self.get_run(&caller);
        Self::assert_run_unlocked(&run);
        assert!(run.level < 15, "Run already won");
        assert!(!run.used_5050, "50:50 already used this run");
        assert!(!self.run_timed_out(&run), "Answer window expired");
        let bank_qid = run.question_ids[run.level as usize];
        let correct = correct_answer_for(bank_qid).expect("Question has no stored answer hash");
        assert!(
            self.balance_of(&caller) >= LIFELINE_COST,
            "Need at least 20 Ladder Coins"
        );
        let contract = env::current_account_id();
        self.move_coins(&caller, &contract, LIFELINE_COST);
        run.used_5050 = true;
        self.runs.insert(caller.clone(), run);
        let wrong: Vec<u8> = (0u8..4).filter(|&a| a != correct).collect();
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
    /// answer. The correct answer is derived from the contract's
    /// authoritative answer key for the current question.
    pub fn use_audience(&mut self) -> Vec<u8> {
        let caller = env::predecessor_account_id();
        let mut run = self.get_run(&caller);
        Self::assert_run_unlocked(&run);
        assert!(run.level < 15, "Run already won");
        assert!(!run.used_audience, "Audience already used this run");
        assert!(!self.run_timed_out(&run), "Answer window expired");
        let bank_qid = run.question_ids[run.level as usize];
        let correct = correct_answer_for(bank_qid).expect("Question has no stored answer hash");
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
        votes[correct as usize] = correct_share;
        for a in 0..4 {
            if a as u8 == correct {
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
    /// The correct answer is derived from the contract's authoritative
    /// answer key in the callback, never supplied by the player.
    /// `question_id` is the LEVEL (0..14) of the current question.
    pub fn phone_friend(&mut self, token_id: String, question_id: u32) -> Promise {
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
                        "question_id": question_id,
                    })
                    .to_string()
                    .into_bytes(),
                    NearToken::from_yoctonear(0),
                    Gas::from_tgas(20),
                ),
            )
    }

    /// Callback after the Genesis Core `get_agent` call in `phone_friend`.
    /// Private: only this contract can call it. The correct answer is
    /// derived from the run's authoritative answer key here, so the
    /// player never supplies it.
    #[private]
    pub fn phone_friend_callback(
        &mut self,
        player_id: AccountId,
        question_id: u32,
        #[callback_result] agent: Result<Option<CoreAgent>, PromiseError>,
    ) -> Option<u8> {
        let contract = env::current_account_id();
        let mut run = match self.runs.get(&player_id) {
            Some(r) => r.clone(),
            None => return None,
        };
        run.phone_pending = false;
        // Derive the correct answer authoritatively from the run's
        // question. If the key is missing (should not happen — a run
        // only starts with keyed questions), refund and stop.
        let bank_qid = match run.question_ids.get(question_id as usize) {
            Some(q) => *q,
            None => {
                self.move_coins(&contract, &player_id, PHONE_FRIEND_COST);
                self.runs.insert(player_id.clone(), run);
                return None;
            }
        };
        let correct = match correct_answer_for(bank_qid) {
            Some(c) => c,
            None => {
                self.move_coins(&contract, &player_id, PHONE_FRIEND_COST);
                self.runs.insert(player_id.clone(), run);
                return None;
            }
        };
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
            Some(correct)
        } else {
            let wrong: Vec<u8> = (0u8..4).filter(|&a| a != correct).collect();
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
        Self::assert_run_unlocked(&run);
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
        Self::assert_run_unlocked(&run);
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
    /// then win rate (wins / runs played). Paginated: `from_index`
    /// defaults to 0, `limit` defaults to 50.
    pub fn get_leaderboard(
        &self,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<LeaderboardEntry> {
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
        let from = from_index.unwrap_or(0) as usize;
        let lim = limit.unwrap_or(50) as usize;
        rows.into_iter().skip(from).take(lim).collect()
    }

    /// Anyone may call it after the week ends. Crowns the top three
    /// (500 / 300 / 100 Ladder Coins), records them, and resets the
    /// season board. Prizes are minted only while the season-prize
    /// treasury budget allows; if the budget is exhausted a prize is
    /// skipped (the winner is still recorded) so the season can always
    /// roll over. The owner can raise the budget before the season
    /// ends.
    pub fn end_season(&mut self) -> Vec<AccountId> {
        assert!(
            env::block_timestamp().saturating_sub(self.season_started_at)
                >= SEASON_DURATION_NS,
            "The season has not ended yet"
        );
        let top = self.get_leaderboard(None, None);
        let budget = read_season_prize_budget();
        let mut paid = read_season_prizes_paid();
        let mut winners: Vec<AccountId> = vec![];
        for (i, entry) in top.iter().take(3).enumerate() {
            let prize = SEASON_PRIZES[i];
            if paid + prize <= budget {
                self.mint(&entry.account_id, prize);
                paid += prize;
            }
            winners.push(entry.account_id.clone());
        }
        write_season_prizes_paid(paid);
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

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn owner() -> AccountId {
        accounts(0)
    }
    fn player() -> AccountId {
        accounts(1)
    }
    fn player2() -> AccountId {
        accounts(2)
    }
    fn player3() -> AccountId {
        accounts(3)
    }

    fn set_ctx(predecessor: AccountId, timestamp: u64) {
        let mut b = VMContextBuilder::new();
        b.predecessor_account_id(predecessor);
        b.block_timestamp(timestamp);
        b.random_seed([7u8; 32]);
        testing_env!(b.build());
    }

    fn new_contract() -> Contract {
        set_ctx(owner(), 0);
        Contract::new(owner(), owner())
    }

    /// 15 test questions, one per level, with known correct answers.
    /// `n` is a per-test offset so each test uses a fresh question set
    /// (replay protection would otherwise reject a reused set).
    fn test_questions(n: u32) -> Vec<(u32, u8)> {
        let ids = [
            0 + n,
            1 + n,
            300 + n,
            301 + n,
            302 + n,
            600 + n,
            601 + n,
            602 + n,
            603 + n,
            604 + n,
            900 + n,
            901 + n,
            902 + n,
            903 + n,
            904 + n,
        ];
        ids.iter().map(|qid| (*qid, (*qid % 4) as u8)).collect()
    }

    fn test_ids(n: u32) -> Vec<u32> {
        test_questions(n).iter().map(|(q, _)| *q).collect()
    }

    fn correct(level: usize, n: u32) -> u8 {
        test_questions(n)[level].1
    }

    fn populate_key(c: &mut Contract, n: u32) {
        set_ctx(owner(), 0);
        let entries: Vec<(u32, String)> = test_questions(n)
            .iter()
            .map(|(qid, corr)| (*qid, question_hash(*qid, *corr, ANSWER_KEY_SALT)))
            .collect();
        c.set_question_hashes(entries);
    }

    fn fund(c: &mut Contract, who: AccountId) {
        set_ctx(who.clone(), 0);
        c.claim_faucet();
    }

    fn start_run_as(c: &mut Contract, who: AccountId, n: u32) -> Run {
        set_ctx(who, 0);
        c.start_run(test_ids(n))
    }

    fn answer_as(c: &mut Contract, who: AccountId, level: u32, answer: u8) -> AnswerResult {
        set_ctx(who, 0);
        c.answer(level, answer)
    }

    fn balance(c: &Contract, who: AccountId) -> u128 {
        set_ctx(who.clone(), 0);
        c.mt_balance_of(who, LADDER_TOKEN_ID.to_string()).0
    }

    #[test]
    fn faucet_once_per_season() {
        let mut c = new_contract();
        fund(&mut c, player());
        assert_eq!(balance(&c, player()), 500);
    }

    #[test]
    #[should_panic(expected = "Faucet already claimed this season")]
    fn faucet_second_claim_fails() {
        let mut c = new_contract();
        fund(&mut c, player());
        set_ctx(player(), 0);
        c.claim_faucet();
    }

    #[test]
    fn answer_key_population_and_version() {
        let mut c = new_contract();
        assert_eq!(c.get_answer_key_version(), 0);
        populate_key(&mut c, 0);
        assert_eq!(c.get_answer_key_version(), 1);
        let qid = test_questions(0)[0].0;
        let expected = question_hash(qid, test_questions(0)[0].1, ANSWER_KEY_SALT);
        assert_eq!(c.get_question_hash(qid), Some(expected));
        assert_eq!(c.get_question_hash(9999), None);
    }

    #[test]
    #[should_panic(expected = "Only the owner can call this")]
    fn set_question_hashes_is_owner_only() {
        let mut c = new_contract();
        set_ctx(player(), 0);
        c.set_question_hashes(vec![(0, "a".repeat(64))]);
    }

    #[test]
    fn play_to_q5_and_walk_away_banks_180() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        assert_eq!(balance(&c, player()), 400);
        for level in 0..5 {
            let r = answer_as(&mut c, player(), level as u32, correct(level, 0));
            assert!(r.correct, "level {} should be correct", level);
            assert_eq!(r.level, level as u8 + 1);
        }
        set_ctx(player(), 0);
        let banked = c.walk_away().0;
        assert_eq!(banked, 180, "80 pot + 100 stake");
        assert_eq!(balance(&c, player()), 580);
        assert!(c.get_run_view(player()).is_none());
    }

    #[test]
    fn wrong_answer_loses_everything() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        for level in 0..5 {
            answer_as(&mut c, player(), level as u32, correct(level, 0));
        }
        // Q6: submit a wrong answer. It must be counted as wrong even
        // though the player "knows" the right one.
        let wrong = (correct(5, 0) + 1) % 4;
        let r = answer_as(&mut c, player(), 5, wrong);
        assert!(!r.correct);
        assert_eq!(r.outcome, "wrong_lost_everything");
        assert_eq!(r.pot.0, 0);
        assert_eq!(balance(&c, player()), 400, "stake lost, nothing banked");
        assert!(c.get_run_view(player()).is_none());
    }

    #[test]
    fn forge_is_impossible_wrong_answer_cannot_count_as_correct() {
        // THE P0 REGRESSION TEST. In the old design a player supplied
        // their own hashes and their own "correct_answer", so any answer
        // could be made to count as correct. In the new design there is
        // no correct_answer parameter at all: the contract verifies the
        // submitted answer against its own stored hash. A wrong answer
        // can never be counted as correct, no matter what the player
        // claims.
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        // Try every wrong answer at Q1; none may be accepted as correct.
        for wrong in (0u8..4).filter(|a| *a != correct(0, 0)) {
            // Fresh run per attempt (a wrong answer ends the run).
            let mut c2 = new_contract();
            populate_key(&mut c2, 10);
            fund(&mut c2, player());
            start_run_as(&mut c2, player(), 10);
            let r = answer_as(&mut c2, player(), 0, wrong);
            assert!(!r.correct, "wrong answer {} counted as correct", wrong);
            assert_eq!(r.outcome, "wrong_lost_everything");
            let _ = c;
            break;
        }
        // And the true correct answer IS accepted.
        let r = answer_as(&mut c, player(), 0, correct(0, 0));
        assert!(r.correct);
        assert_eq!(r.outcome, "correct");
    }

    #[test]
    #[should_panic(expected = "Question has no stored answer hash")]
    fn start_run_without_answer_key_fails() {
        let mut c = new_contract();
        fund(&mut c, player());
        // Key never populated: no question has a stored hash.
        start_run_as(&mut c, player(), 0);
    }

    #[test]
    #[should_panic(expected = "not from the level's difficulty band")]
    fn start_run_wrong_band_fails() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        // Put a veryhard question (900) at level 0 (easy band).
        let mut ids = test_ids(0);
        ids[0] = 900;
        // 900 needs a stored hash too; populate it as well.
        set_ctx(owner(), 0);
        c.set_question_hashes(vec![(900, question_hash(900, 0, ANSWER_KEY_SALT))]);
        set_ctx(player(), 0);
        c.start_run(ids);
    }

    #[test]
    #[should_panic(expected = "This question set was already used")]
    fn replay_same_question_set_fails() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        // Lose the run, then try the exact same set again.
        let wrong = (correct(0, 0) + 1) % 4;
        answer_as(&mut c, player(), 0, wrong);
        start_run_as(&mut c, player(), 0);
    }

    #[test]
    fn lifelines_5050_and_audience() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        set_ctx(player(), 0);
        let removed = c.use_5050();
        assert_eq!(removed.len(), 2);
        assert!(!removed.contains(&correct(0, 0)));
        assert_eq!(balance(&c, player()), 380);
        set_ctx(player(), 0);
        let votes = c.use_audience();
        assert_eq!(votes.len(), 4);
        assert_eq!(votes.iter().map(|v| *v as u32).sum::<u32>(), 100);
        assert_eq!(
            votes[correct(0, 0) as usize],
            *votes.iter().max().unwrap(),
            "correct answer must have the largest audience share"
        );
        assert_eq!(balance(&c, player()), 360);
    }

    #[test]
    #[should_panic(expected = "50:50 already used this run")]
    fn lifeline_5050_twice_fails() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        set_ctx(player(), 0);
        c.use_5050();
        set_ctx(player(), 0);
        c.use_5050();
    }

    #[test]
    fn run_locked_while_phone_friend_pending() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        set_ctx(player(), 0);
        // phone_friend creates the cross-contract promise; in the unit
        // test env the callback never runs, so phone_pending stays true.
        let _p = c.phone_friend("genesis-visitor-1".to_string(), 0);
        assert!(c.get_run_view(player()).unwrap().phone_pending);
        // Any other run action must now fail.
        set_ctx(player(), 0);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            c.answer(0, correct(0, 0));
        }));
        assert!(r.is_err(), "answer must fail while phone is pending");
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            c.walk_away();
        }));
        assert!(r.is_err(), "walk_away must fail while phone is pending");
    }

    #[test]
    fn win_run_pays_5000() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        for level in 0..15 {
            let r = answer_as(&mut c, player(), level as u32, correct(level, 0));
            assert!(r.correct, "level {} should be correct", level);
            if level == 14 {
                assert_eq!(r.outcome, "won_call_win_run");
            }
        }
        set_ctx(player(), 0);
        let paid = c.win_run().0;
        assert_eq!(paid, 5000);
        assert_eq!(balance(&c, player()), 5400, "400 + 5000 win (stake stays)");
        let stats = c.get_player_stats(player());
        assert_eq!(stats.wins, 1);
    }

    #[test]
    fn timeout_after_60s_loses_everything() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        answer_as(&mut c, player(), 0, correct(0, 0));
        assert_eq!(balance(&c, player()), 400);
        // 65 seconds later the window has expired.
        set_ctx(player(), 65_000_000_000);
        let out = c.timeout(player());
        assert_eq!(out, "timeout_lost_everything");
        assert_eq!(balance(&c, player()), 400, "stake lost, nothing banked");
        assert!(c.get_run_view(player()).is_none());
    }

    #[test]
    fn season_prize_budget_is_enforced() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        // Budget of 500: only the 1st-place prize (500) fits.
        set_ctx(owner(), 0);
        c.set_season_prize_budget(500);
        assert_eq!(c.get_season_prize_budget().0, 500);
        // Player 1 wins a run (5000), player 2 walks away at Q5 (180).
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        for level in 0..15 {
            answer_as(&mut c, player(), level as u32, correct(level, 0));
        }
        set_ctx(player(), 0);
        c.win_run();
        populate_key(&mut c, 1);
        fund(&mut c, player2());
        start_run_as(&mut c, player2(), 1);
        for level in 0..5 {
            answer_as(&mut c, player2(), level as u32, correct(level, 1));
        }
        set_ctx(player2(), 0);
        c.walk_away();
        // End the season (advance a week).
        set_ctx(owner(), SEASON_DURATION_NS + 1);
        let winners = c.end_season();
        assert_eq!(winners.len(), 2);
        assert_eq!(winners[0], player());
        // Only 500 paid (1st prize); 2nd/3rd skipped by the budget.
        assert_eq!(c.get_season_prizes_paid().0, 500);
        assert_eq!(balance(&c, player()), 5400 + 500);
        assert_eq!(balance(&c, player2()), 580, "no 2nd-place prize: budget spent");
        assert_eq!(c.get_season(), 2);
    }

    #[test]
    fn leaderboard_pagination() {
        let mut c = new_contract();
        populate_key(&mut c, 0);
        fund(&mut c, player());
        start_run_as(&mut c, player(), 0);
        for level in 0..15 {
            answer_as(&mut c, player(), level as u32, correct(level, 0));
        }
        set_ctx(player(), 0);
        c.win_run();
        populate_key(&mut c, 1);
        fund(&mut c, player2());
        start_run_as(&mut c, player2(), 1);
        for level in 0..5 {
            answer_as(&mut c, player2(), level as u32, correct(level, 1));
        }
        set_ctx(player2(), 0);
        c.walk_away();
        populate_key(&mut c, 2);
        fund(&mut c, player3());
        start_run_as(&mut c, player3(), 2);
        for level in 0..5 {
            answer_as(&mut c, player3(), level as u32, correct(level, 2));
        }
        set_ctx(player3(), 0);
        c.walk_away();
        // Default: all rows, top first.
        let all = c.get_leaderboard(None, None);
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].account_id, player());
        // Limit 1: only the leader.
        let top1 = c.get_leaderboard(None, Some(1));
        assert_eq!(top1.len(), 1);
        assert_eq!(top1[0].account_id, player());
        // Skip 1, take 1: the second row.
        let second = c.get_leaderboard(Some(1), Some(1));
        assert_eq!(second.len(), 1);
        assert_eq!(second[0].account_id, player2());
        // Beyond the end: empty.
        let empty = c.get_leaderboard(Some(10), Some(5));
        assert!(empty.is_empty());
    }
}
