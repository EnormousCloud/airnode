use jsonrpc_core::types::params::Params;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use web3::types::{Filter, Log, H160, U256};

pub struct EthClient {
    rpc_addr: String,
    agent: ureq::Agent,
}

#[derive(Debug, Clone, Deserialize)]
struct EvmNumericResult {
    pub result: U256,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcSingleResponse<T> {
    pub id: serde_json::Value,
    pub result: T,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcId {
    pub id: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcErrorResponse {
    pub id: serde_json::Value,
    pub error: RpcError,
}

#[derive(Debug, Clone, Serialize)]
pub struct RpcSingleRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Params,
}

pub type RpcBatchRequest = Vec<RpcSingleRequest>;
pub type RpcBatchResponse = Vec<Value>;

pub fn batch_fragment<T>(response: &RpcBatchResponse, id_match: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    for v in response {
        let id = v.as_object().unwrap().get("id").unwrap().as_str();
        if let Some(id_val) = id {
            if id_val == id_match {
                let s = serde_json::to_string(&v).unwrap();
                if let Ok(err) = serde_json::from_str::<RpcErrorResponse>(&s) {
                    return Err(anyhow::Error::msg(err.error.message));
                }
                let out: RpcSingleResponse<T> = serde_json::from_str(&s).unwrap();
                return Ok(out.result);
            }
        }
    }
    Err(anyhow::Error::msg("result not found in the batch"))
}

impl EthClient {
    pub fn new(rpc_addr: &str) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(60))
            .timeout_write(Duration::from_secs(5))
            .build();
        EthClient {
            agent,
            rpc_addr: rpc_addr.to_string(),
        }
    }

    pub fn execute_str<T>(&self, payload: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let rq = self
            .agent
            .post(&self.rpc_addr)
            .set("Content-Type", "application/json");
        tracing::debug!("JSONRPC request={}", payload);
        let response: String = match rq.send_string(&payload) {
            Ok(x) => x.into_string().unwrap(),
            Err(e) => return Err(anyhow::Error::new(e)),
        };
        if let Ok(err) = serde_json::from_str::<RpcErrorResponse>(&response) {
            return Err(anyhow::Error::msg(err.error.message));
        }

        tracing::debug!("JSONRPC response={}", response);
        Ok(serde_json::from_str::<T>(&response).unwrap())
    }

    pub fn execute<T>(&self, method: &str, params: Params) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let payload = serde_json::to_string(&RpcSingleRequest {
            jsonrpc: "2.0".to_owned(),
            id: "1".to_owned(),
            method: method.to_string(),
            params: params.clone(),
        })
        .unwrap();
        self.execute_str(&payload)
    }

    pub fn get_logs(&self, filter: &Filter) -> anyhow::Result<Vec<Log>> {
        let filter_str = serde_json::to_string(filter).expect("filter serialize failure");
        let payload = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getLogs\",\"params\":[{}],\"id\":\"1\"}}",
            filter_str
        );
        let res: RpcSingleResponse<Vec<Log>> = self.execute_str(&payload)?;
        Ok(res.result)
    }

    pub fn new_filter(&self, filter: &Filter) -> anyhow::Result<U256> {
        let filter_str = serde_json::to_string(filter).expect("filter serialize failure");
        let payload = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_newFilter\",\"params\":[{}],\"id\":\"1\"}}",
            filter_str
        );
        let res: RpcSingleResponse<U256> = self.execute_str(&payload)?;
        Ok(res.result)
    }

    pub fn filter_changes(&self, filter_id: U256) -> anyhow::Result<Vec<Log>> {
        let payload = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getFilterChanges\",\"params\":[\"0x{:x}\"],\"id\":\"1\"}}",
            filter_id
        );
        let res: RpcSingleResponse<Vec<Log>> = self.execute_str(&payload)?;
        Ok(res.result)
    }

    pub fn get_chain_id(&self) -> anyhow::Result<u64> {
        let payload = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"net_version\",\"id\":\"1\"}}");
        let out: RpcSingleResponse<serde_json::Value> = match self.execute_str(&payload) {
            Ok(x) => x,
            Err(x) => return Err(anyhow::Error::msg(x.to_string())),
        };
        match out.result {
            serde_json::Value::Number(n) => return Ok(n.as_u64().unwrap()),
            serde_json::Value::String(s) => return Ok(s.parse().unwrap()),
            _ => return Err(anyhow::Error::msg("result convertion failure")),
        }
    }

    pub fn get_max_block_number(&self) -> anyhow::Result<u64> {
        let payload = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_blockNumber\",\"params\":[],\"id\":\"1\"}}",
        );
        let res: RpcSingleResponse<U256> = self.execute_str(&payload)?;
        Ok(res.result.as_u64())
    }

    pub fn get_eth_balance(&self, address: H160) -> anyhow::Result<U256> {
        let payload = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBalance\",\"params\":[{},\"latest\"],\"id\":\"1\"}}",
            address
        );
        let res: RpcSingleResponse<U256> = self.execute_str(&payload)?;
        Ok(res.result)
    }
}
