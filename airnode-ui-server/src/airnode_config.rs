use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use web3::types::H160;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        format!("{}.{}", self.contract_address, self.chain_id)
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        Vec::from(self.to_string().as_bytes())
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
        Vec::from(format!("{}.{}", self.contract_address, self.chain_id).as_bytes())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        Vec::from(serde_json::to_string(&self).unwrap().as_bytes())
    }
}

#[derive(StructOpt, Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum AirnodeConfigCmd {
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
    Get {
        #[structopt(long)]
        contract_address: H160,
        #[structopt(long)]
        chain_id: u64,
    },
    List {},
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
    Delete {
        #[structopt(long)]
        contract_address: H160,
        #[structopt(long)]
        chain_id: u64,
    },
}
