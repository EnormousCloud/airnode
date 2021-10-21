use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use web3::types::H160;

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

