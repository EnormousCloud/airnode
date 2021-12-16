use airnode_events::AirnodeEvent;

use serde::{Deserialize, Serialize};
use web3::types::{Log, H256};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LogEvent {
    pub block_number: u64,
    pub transaction_hash: H256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<AirnodeEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl LogEvent {
    pub fn new(log: Log) -> Self {
        let block_number = log.block_number.unwrap().as_u64();
        let transaction_hash = log.transaction_hash.unwrap();
        let (event, error) = match AirnodeEvent::from_log(&log) {
            Ok(evt) => (Some(evt), None),
            Err(e) => (None, Some(format!("{}", e))),
        };
        Self {
            block_number,
            transaction_hash,
            event,
            error,
        }
    }

    pub fn is_unknown(&self) -> bool {
        let e = match &self.event {
            Some(x) => x,
            _ => return false,
        };
        match e {
            AirnodeEvent::Unknown{ topic: _ } => true,
            _ => false,
        }
    }
}

impl PartialEq for LogEvent {
    fn eq(&self, other: &Self) -> bool {
        if self.block_number != other.block_number {
            return false;
        }
        if self.transaction_hash != other.transaction_hash {
            return false;
        }
        // dummy way to compare
        serde_json::to_string(&self).unwrap() == serde_json::to_string(&other).unwrap()
    }
}
