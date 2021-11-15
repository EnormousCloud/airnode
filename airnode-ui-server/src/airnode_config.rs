use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};
use structopt::StructOpt;
use web3::types::H160;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct AirnodeRef {
    pub chain_id: u64,
    pub contract_address: H160,
}
impl AirnodeRef {
    pub fn new(chain_id: u64, contract_address: H160) -> Self {
        Self {
            chain_id,
            contract_address,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}.{:?}", self.chain_id, self.contract_address)
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        Vec::from(self.to_string())
    }
}

impl Ord for AirnodeRef {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.chain_id, &self.contract_address).cmp(&(other.chain_id, &other.contract_address))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirnodeConfig {
    pub chain_id: u64,
    pub contract_address: H160,
    pub rpc_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_block: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u64>,
}

impl AirnodeConfig {
    pub fn from(src: Vec<u8>) -> anyhow::Result<Self> {
        let s = match String::from_utf8(src) {
            Ok(x) => x,
            Err(e) => return Err(anyhow::Error::new(e)),
        };
        Ok(match serde_json::from_str(&s) {
            Ok(x) => x,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        })
    }
    pub fn key(&self) -> Vec<u8> {
        Vec::from(format!("{}.{:?}", self.chain_id, self.contract_address).as_bytes())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        Vec::from(serde_json::to_string(&self).unwrap().as_bytes())
    }

    pub fn new(
        rpc_address: &str,
        contract_address: H160,
        min_block: Option<u64>,
        batch_size: Option<u64>,
    ) -> anyhow::Result<Self> {
        let client = crate::web3sync::EthClient::new(&rpc_address);
        let chain_id = match client.get_chain_id() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        Ok(Self {
            chain_id,
            contract_address,
            rpc_address: rpc_address.to_string(),
            min_block,
            batch_size,
        })
    }
}

#[derive(StructOpt, Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum AirnodeConfigCmd {
    /// Add airnode RRP contract to the configuration
    Add {
        #[structopt(long)]
        contract_address: H160,
        #[structopt(long, env = "RPC_ENDPOINT")]
        rpc_address: String,
        #[structopt(long)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[structopt(long, env = "RPC_MIN_BLOCK")]
        min_block: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[structopt(long, env = "RPC_BATCH_SIZE")]
        batch_size: Option<u64>,
    },
    /// Get airnode RRP contract configuration
    Get {
        #[structopt(long)]
        contract_address: H160,
        #[structopt(long)]
        chain_id: u64,
    },
    /// Get the list of RRP contracts
    List,
    /// Update airnode RRP contract configuration
    Update {
        #[structopt(long)]
        contract_address: H160,
        #[structopt(long)]
        chain_id: u64,
        #[structopt(long, env = "RPC_ENDPOINT")]
        rpc_address: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[structopt(long, env = "RPC_BATCH_SIZE")]
        batch_size: Option<u64>,
    },
    /// Delete airnode RRP configuration and erase all its history
    Delete {
        #[structopt(long)]
        contract_address: H160,
        #[structopt(long)]
        chain_id: u64,
    },
}
