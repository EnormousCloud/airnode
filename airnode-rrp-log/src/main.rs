pub mod args;
pub mod filter;
pub mod logevent;
pub mod reader;

use airnode_events::AirnodeEvent;
use serde::Serialize;
use std::collections::BTreeMap;
use std::str::FromStr;
use tracing::info;
use web3::types::{H160, H256};

#[derive(Debug, Clone)]
pub struct State {
    // a map of unknown topics
    pub unknown: BTreeMap<H256, H256>,
}

impl State {
    pub fn new() -> Self {
        Self {
            unknown: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct Out {
    block: u64,
    tx: H256,
    event: AirnodeEvent,
}

impl reader::EventHandler for State {
    fn on(&mut self, l: web3::types::Log) -> () {
        let evt = AirnodeEvent::from_log(&l).unwrap();
        match evt {
            AirnodeEvent::Unknown => {
                // info!("{:?}", serde_json::to_string(&l).unwrap());
                // info!("{:?} {:?} {:?}", l.block_number.unwrap(), l.transaction_hash.unwrap(), evt);
                self.unknown
                    .insert(l.topics[0], l.transaction_hash.unwrap());
            }
            AirnodeEvent::Unclassified => {
                // Unclassified: this is what is ok to skip silently
                // info!("{:?} {:?} {:?}", l.block_number.unwrap(), l.transaction_hash.unwrap(), evt);
            }
            _ => {
                let out = Out {
                    block: l.block_number.unwrap().as_u64(),
                    tx: l.transaction_hash.unwrap(),
                    event: evt,
                };
                info!("{}", serde_json::to_string(&out).unwrap());
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => return Err(anyhow::Error::msg(format!("args parsing error {}", e))),
    };
    let transport = reader::get_transport(&args.rpc_endpoint).await;
    let web3 = web3::Web3::new(transport);
    let chain_id = web3.eth().chain_id().await?.as_u64();
    let addr_contract =
        H160::from_str(args.address_contract.as_str()).expect("ADDR_CONTRACT is missing");

    let mut state = State::new();
    let mut scanner = reader::Scanner::new(
        chain_id,
        args.min_block,
        args.max_block,
        args.rpc_batch_size,
    );
    let _ = scanner.scan_address(&web3, addr_contract, &mut state).await;
    if state.unknown.len() > 0 {
        return Err(anyhow::Error::msg("unknown events met"));
    }
    Ok(())
}
