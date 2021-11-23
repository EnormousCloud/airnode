use crate::airnode_config::AirnodeRef;
use crate::airnode_ops::Operation;
use crate::web3sync::EthClient;
use airnode_events::AirnodeEvent;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use structopt::StructOpt;
use web3::types::{H160, H256, U256};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// the last balance value
    pub last_value: U256,
    /// time of the last balance update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<DateTime<Utc>>,
    /// number of decimals (18)
    pub decimals: usize,
    /// symbol of the token network
    pub symbol: String,
}

fn chain_symbol(chain_id: u64) -> String {
    if chain_id == 100 {
        return "xDAI".to_owned();
    }
    if chain_id == 30 || chain_id == 31 {
        return "RBTC".to_owned();
    }
    "ETH".to_owned()
}

fn chain_decimals(_chain_id: u64) -> usize {
    18
}

impl Balance {
    pub fn new(value: U256, chain_id: u64) -> Self {
        let now = Utc::now();
        Self {
            last_value: value,
            last_updated: Some(now),
            decimals: chain_decimals(chain_id),
            symbol: chain_symbol(chain_id),
        }
    }
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
pub struct AirnodeRR {
    /// request ID
    pub id: U256,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AirnodeState {
    /// extended public key of the Airnode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpubkey: Option<String>,
    /// sponsor of this airnode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsor: Option<H160>,
    /// map of requests that were actually took place
    pub requests: Map<H256, AirnodeRR>,
    /// map of endpoints that were actually used
    pub endpoints: Map<H256, u32>,
    /// map of templates that were actually used
    pub templates: Map<H256, u32>,
    /// map of functions that were actually used
    pub functions: Map<H256, u32>,
    /// list of whitelist addresses
    pub whitelisted: Vec<H160>,
    /// map of requesters (index -> admin)
    pub requesters: Map<u64, H160>,
    /// current balance of airnode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<Balance>,
    /// number of operations that happened with this airnode
    pub operations_num: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AirnodeRrpAdmin {
    /// address of the admin
    address: H160,
    /// rank of the admin
    rank: u64,
}
impl AirnodeRrpAdmin {
    pub fn new(address: H160, rank: u64) -> Self {
        Self { address, rank }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AirnodeRrpState {
    /// whether this airnode is owned
    /// by the private key/seed in the contaner environment
    pub owned: bool,
    /// chain ID
    pub chain_id: u64,
    /// address of the airnode
    pub contract_address: H160,
    /// details about chain syncing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<SyncState>,
    /// current balance of RRP contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<Balance>,
    /// Map of each provider (pre-alpha)
    pub providers: Map<U256, AirnodeState>,
    /// Map of each airnode (v0.2+)
    pub airnodes: Map<H160, AirnodeState>,
    /// number of operations that happened
    pub operations_num: u32,
    /// list of admins of this RRP contract
    pub admins: Map<H160, AirnodeRrpAdmin>,
}

impl AirnodeRrpState {
    pub fn new(node: &AirnodeRef) -> Self {
        Self {
            chain_id: node.chain_id,
            contract_address: node.contract_address,
            ..Self::default()
        }
    }
}

impl AirnodeRrpState {
    pub fn handle_op(&mut self, op: &Operation) {
        match op.event {
            AirnodeEvent::SetRankAdminned {
                adminned: _,
                caller_admin: _,
                target_admin,
                new_rank,
            } => {
                self.admins.insert(
                    target_admin.clone(),
                    AirnodeRrpAdmin::new(target_admin, new_rank.as_u64()),
                );
            }
            AirnodeEvent::SetRank {
                caller_admin: _,
                target_admin,
                new_rank,
            } => {
                self.admins.insert(
                    target_admin.clone(),
                    AirnodeRrpAdmin::new(target_admin, new_rank.as_u64()),
                );
            }
            _ => {}
        };
        if let Some(provider_id) = op.event.get_provider_id() {
            match &op.event {
                AirnodeEvent::ProviderCreatedA {
                    provider_id: _,
                    admin: _,
                    xpub,
                } => {
                    let mut provider = AirnodeState::default();
                    provider.xpubkey = Some(xpub.clone());
                    self.providers.insert(provider_id, provider);
                }
                AirnodeEvent::RequesterCreatedA {
                    requester_index,
                    admin,
                } => {
                    if let Some(provider) = self.providers.get_mut(&provider_id) {
                        let key = requester_index.as_u64();
                        provider.requesters.insert(key, *admin);
                    }
                }
                _ => {}
            };
            if let Some(provider) = self.providers.get_mut(&provider_id) {
                provider.operations_num += 1;
            }
        };
        self.operations_num += 1;
    }

    pub fn update_balance(&mut self, rpc_address: &str) {
        let client = EthClient::new(rpc_address);
        let balance = match client.get_eth_balance(self.contract_address) {
            Ok(x) => x,
            Err(e) => {
                tracing::warn!(
                    "Error of retrieving balance {} of RRP contract {:?}",
                    e,
                    self.contract_address,
                );
                return;
            }
        };
        self.balance = Some(Balance::new(balance, self.chain_id));
        for (node, state) in &mut self.airnodes {
            let balance = match client.get_eth_balance(*node) {
                Ok(x) => x,
                Err(e) => {
                    tracing::warn!("Error of retrieving balance {} of airnode {:?}", e, node);
                    return;
                }
            };
            state.balance = Some(Balance::new(balance, self.chain_id));
        }
    }
}

#[derive(StructOpt, Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum AirnodeStateCmd {
    /// List states for all nodes
    List {
        /// Skip syncing
        #[structopt(long)]
        no_sync: bool,
    },
    /// Get the state of the single node
    Get {
        /// Chain ID of RRP contract in case of "op" or "state" command
        #[structopt(long, default_value = "1")]
        chain_id: u64,
        /// Contract address of RRP contract in case of "op" or "state" command
        #[structopt(long, default_value = "")]
        contract_address: H160,
        /// Skip syncing
        #[structopt(long)]
        no_sync: bool,
    },
}
