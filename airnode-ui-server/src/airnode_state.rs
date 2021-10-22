use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use web3::types::U256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub last_value: U256,
    pub last_updated: DateTime<Utc>,
    pub decimals: usize,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub chain_id: u64,
    pub is_syncing: bool,
    pub last_block: u64,
    pub head_block: u64,
    pub filter_id: u64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirnodeState {
    pub balance: Option<Balance>,
    pub sync: Option<SyncState>,
}

// STATE
//   1) syncing state
//   2) collected stats for the menu
