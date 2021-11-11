use crate::airnode_config::AirnodeRef;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use structopt::StructOpt;
use web3::types::{H160, H256, U256};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// the last balance value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_value: Option<U256>,
    /// time of the last balance update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<DateTime<Utc>>,
    /// number of decimals (18)
    pub decimals: usize,
    /// symbol of the token network
    pub symbol: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SyncState {
    /// chain ID
    pub chain_id: u64,
    /// chain human-friendly name
    pub chain_name: String,
    /// whether the airnode is currently syncing
    pub is_syncing: bool,
    /// the last block that was processed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_block: Option<u64>,
    /// the head block in the blockchain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_block: Option<u64>,
    /// ID of the filter that is used to watch newcoming events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<u64>,
    /// time of the last update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AirnodeState {
    /// whether this airnode is owned
    /// by the private key/seed in the contaner environment
    pub owned: bool,
    /// chain ID
    pub chain_id: u64,
    /// address of the airnode
    pub address: H160,
    /// details about chain syncing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<SyncState>,
    /// extended public key of the Airnode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpubkey: Option<String>,
    /// current balance details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<Balance>,
    /// sponsor of this airnode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsor: Option<H160>,
    /// map of requests that were actually took place
    pub requests: Map<H256, u32>,
    /// map of endpoints that were actually used
    pub endpoints: Map<H256, u32>,
    /// map of templates that were actually used
    pub templates: Map<H256, u32>,
    /// map of functions that were actually used
    pub functions: Map<H256, u32>,
    /// list of whitelist addresses
    pub whitelisted: Vec<H160>,
    /// list of admins of this airnode
    pub admins: Vec<H160>,
    /// number of operations that happened
    pub operations_num: u32,
}

impl AirnodeState {
    pub fn new(node: &AirnodeRef) -> Self {
        Self {
            chain_id: node.chain_id,
            address: node.contract_address,
            ..Self::default()
        }
    }
}

#[derive(StructOpt, Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum AirnodeStateCmd {
    /// List states for all nodes
    List,
    /// Get the state of the single node
    Get,
}
