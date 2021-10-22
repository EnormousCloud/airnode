use airnode_events::AirnodeEvent;
use serde::{Deserialize, Serialize};
use web3::types::{Block, Log, Transaction, TransactionReceipt as Receipt, H160, H256, U256};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxFee {
    /// Gas Price
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    /// Gas amount (limit on transaction creation)
    pub gas: U256,
    /// Gas that was actually used from receipt
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<U256>,
    /// USD equivalent of the price paid for gas
    pub usd: Option<f64>,
}

/// Operation encapsulates business-level details of the event
/// together with protocol-level details ()
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// Timestamp of the event
    #[serde(rename = "tm")]
    pub timestamp: u64,
    /// Business logic of the event
    #[serde(rename = "e")]
    pub event: AirnodeEvent,
    /// Block height
    #[serde(rename = "h")]
    pub height: u64,
    /// Who called this transaction
    pub from: H160,
    /// Block hash
    #[serde(rename = "b")]
    pub block: H256,
    /// Transaction hash
    #[serde(rename = "tx")]
    pub transaction_hash: H256,
    /// Transaction Log Index
    #[serde(rename = "i")]
    pub log_index: u64,
    /// Fees that were applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<TxFee>,
}

impl Operation {
    /// create operation record from the log file
    /// by pulling all, related to transaction
    pub fn new(log: &Log, rpc_address: &str) -> anyhow::Result<Self> {}
}
