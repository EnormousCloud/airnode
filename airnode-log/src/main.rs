use airnode_events::AirnodeEvent;
use serde::{Deserialize, Serialize};
use tracing::info;
use std::collections::BTreeMap;
use web3::types::H256;

#[derive(Default, Clone)]
pub struct State {}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct LogResponse {
    pub id: u64,
    pub result: Vec<web3::types::Log>,
}

use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_ansi(false)
        .with_level(false)
        .with_target(false);
    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    let b = include_bytes!("../../mocks/xdai-airnode.json");
    let logs: Vec<LogResponse> = serde_json::from_slice(b).unwrap();

    let mut unknown: BTreeMap<H256, H256> = BTreeMap::new();
    for lw in logs {
        for l in lw.result {
            let evt = AirnodeEvent::from_log(&l).unwrap();
            match evt {
                AirnodeEvent::Unknown => {
                    // info!("{:?}", serde_json::to_string(&l).unwrap());
                    // info!("{:?} {:?} {:?}", l.block_number.unwrap(), l.transaction_hash.unwrap(), evt);
                    unknown.insert(l.topics[0], l.transaction_hash.unwrap());
                }
                AirnodeEvent::Unclassified => {
                    // info!("{:?} {:?} {:?}", l.block_number.unwrap(), l.transaction_hash.unwrap(), evt);
                }
                _ => {
                    info!(
                        "{:?} {:?} {}",
                        l.block_number.unwrap(),
                        l.transaction_hash.unwrap(),
                        serde_json::to_string(&evt).unwrap(),
                    );
                }
            }
        }
    }
    for (topic, tx) in &unknown {
        tracing::warn!("unknown topic={:?} tx={:?}", topic, tx);
    }
    Ok(())

}