use airnode_events::AirnodeEvent;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use web3::types::Log;
use web3::types::{Block, Transaction, TransactionReceipt as Receipt, H160, H256, U256};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRef {
    /// Block height
    #[serde(rename = "h")]
    pub height: u64,
    /// Transaction Block Index
    #[serde(rename = "ti")]
    pub tx_index: u64,
    /// Transaction Log Index
    #[serde(rename = "li")]
    pub log_index: u64,
}

impl OperationRef {
    pub fn to_string(&self) -> String {
        format!(
            "{:08x}.{:02x}.{:02x}",
            self.height, self.tx_index, self.log_index
        )
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        Vec::from(self.to_string().as_bytes())
    }
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
    /// Transaction Block Index
    #[serde(rename = "ti")]
    pub tx_index: u64,
    /// Transaction Log Index
    #[serde(rename = "li")]
    pub log_index: u64,
    /// Fees that were applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<TxFee>,
}

impl Operation {
    /// create operation record from the log file
    /// by pulling all, related to transaction
    pub fn new(log: &Log, rpc_address: &str) -> anyhow::Result<Self> {
    }
}
