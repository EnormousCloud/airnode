pub mod args;
pub mod filter;
pub mod logevent;
pub mod reader;

use crate::args::{Args, OutputFormat};
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
    pub format: OutputFormat,
    pub pretty_print: bool,
    pub log_events: Vec<LogEvent>,
}

impl State {
    pub fn new(args: &Args) -> anyhow::Result<Self> {
        Ok(Self {
            unknown: BTreeMap::new(),
            filtration: LogFiltration::new(args.clone()).unwrap(),
            format: args.format.clone(),
            log_events: vec![],
            pretty_print: args.pretty_print,
        })
    }
}

impl reader::EventHandler for State {
    fn on(&mut self, l: web3::types::Log) -> () {
        let hash = l.transaction_hash.unwrap();
        let topic = l.topics[0];
        let le = LogEvent::new(l);
        if le.is_unknown() {
            self.unknown.insert(topic, hash);
        }
        if self.filtration.allows(&le) {
            match self.format {
                OutputFormat::Jsonl => {
                    // immediate flush
                    if self.pretty_print {
                        tracing::info!("{}", serde_json::to_string_pretty(&le).unwrap());
                    } else {
                        tracing::info!("{}", serde_json::to_string(&le).unwrap());
                    }
                }
                OutputFormat::Json => {
                    // accumulation
                    self.log_events.push(le.clone());
                }
            };
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

    let mut state = State::new(&args).unwrap();
    let mut scanner = reader::Scanner::new(
        chain_id,
        args.min_block,
        args.max_block,
        args.rpc_batch_size,
    );
    let _ = scanner.scan_address(&web3, addr_contract, &mut state).await;
    if let OutputFormat::Json = state.format {
        let o = &state.log_events;
        if state.pretty_print {
            tracing::info!("{}", serde_json::to_string_pretty(o).unwrap());
        } else {
            tracing::info!("{}", serde_json::to_string(o).unwrap());
        }
    }

    if state.unknown.len() > 0 {
        return Err(anyhow::Error::msg("unknown events met"));
    }
    Ok(())
}
