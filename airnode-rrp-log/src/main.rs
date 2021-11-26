pub mod args;
pub mod filter;
pub mod logevent;
pub mod reader;

use crate::filter::LogFiltration;
use crate::logevent::LogEvent;
use std::collections::BTreeMap;
use std::str::FromStr;
use web3::types::{H160, H256};

#[derive(Debug, Clone)]
pub struct State {
    // a map of unknown topics
    pub unknown: BTreeMap<H256, H256>,
    pub filtration: LogFiltration,
}

impl State {
    pub fn new() -> Self {
        Self {
            unknown: BTreeMap::new(),
            filtration: LogFiltration::default(),
        }
    }
}

impl reader::EventHandler for State {
    fn on(&mut self, l: web3::types::Log, pretty_print: bool) -> () {
        let hash = l.transaction_hash.unwrap();
        let topic = l.topics[0];
        let le = LogEvent::new(l);
        if le.is_unknown() {
            self.unknown.insert(topic, hash);
        }
        if self.filtration.allows(&le) {
            if pretty_print {
                tracing::info!("{}", serde_json::to_string_pretty(&le).unwrap());
            } else {
                tracing::info!("{}", serde_json::to_string(&le).unwrap());
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
    let _ = scanner
        .scan_address(&web3, addr_contract, &mut state, args.pretty_print)
        .await;
    if state.unknown.len() > 0 {
        return Err(anyhow::Error::msg("unknown events met"));
    }
    Ok(())
}
