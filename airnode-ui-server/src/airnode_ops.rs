use crate::web3sync::{batch_fragment, RpcBatchRequest, RpcBatchResponse, RpcSingleRequest};
use airnode_events::AirnodeEvent;
use jsonrpc_core::types::params::Params;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<H160>,
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
        let event = AirnodeEvent::from_log(log)?;
        let block = log.block_hash.unwrap();
        let height = log.block_number.unwrap().as_u64();
        let transaction_hash = log.transaction_hash.unwrap();
        let log_index = log.transaction_log_index.unwrap().as_u64();
        let tx_index = log.transaction_index.unwrap().as_u64();
        let fees = None;
        let rpc_client = crate::web3sync::EthClient::new(rpc_address);
        // from getBlockByHash: timestamp
        // from getTransactionByHash: from
        // from getReceiptByHash: fees
        let txh = serde_json::Value::from(format!("{:?}", transaction_hash));
        let blockh = serde_json::Value::from(format!("{:?}", block));
        let rq1 = RpcSingleRequest {
            jsonrpc: "2.0".to_owned(),
            id: "block".to_owned(),
            method: "eth_getBlockByHash".to_owned(),
            params: Params::Array(vec![blockh]),
        };
        let rq2 = RpcSingleRequest {
            jsonrpc: "2.0".to_owned(),
            id: "hash".to_owned(),
            method: "eth_getTransactionByHash".to_owned(),
            params: Params::Array(vec![txh.clone()]),
        };
        let rq3 = RpcSingleRequest {
            jsonrpc: "2.0".to_owned(),
            id: "receipt".to_owned(),
            method: "eth_getTransactionReceipt".to_owned(),
            params: Params::Array(vec![txh.clone()]),
        };
        let batch: RpcBatchRequest = vec![rq1, rq2, rq3];
        let payload = serde_json::to_string(&batch).unwrap();
        let response: RpcBatchResponse = rpc_client.execute_str(&payload)?;

        let b: Block<H256> = batch_fragment(&response, "block").unwrap();
        let timestamp = b.timestamp.as_u64();
        let tx: Transaction = batch_fragment(&response, "hash").unwrap();
        let from = tx.from.clone();
        let _receipt: Receipt = batch_fragment(&response, "receipt").unwrap();

        Ok(Self {
            block,
            event,
            height,
            log_index,
            timestamp,
            transaction_hash,
            tx_index,
            from,
            fees,
        })
    }
}
