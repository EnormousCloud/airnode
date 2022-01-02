use crate::web3sync::EthClient;
use tracing::debug;
use web3::types::{FilterBuilder, Log, H160};

pub trait EventHandler {
    fn on(&mut self, l: Log) -> ();
}

#[derive(Clone, Debug)]
pub struct BlockBatch {
    pub from: u64,
    pub to: u64,
}

fn get_batches(
    rpc_address: &str,
    genesis: u64,
    max: Option<u64>,
    batch_size: u64,
) -> Vec<BlockBatch> {
    let max_block: u64 = match max {
        Some(x) => x,
        None => EthClient::new(rpc_address)
            .get_max_block_number()
            .expect("max block height failure"),
    };
    let mut from = genesis;
    let mut res = vec![];
    while from <= max_block {
        let to = if from + batch_size > max_block {
            max_block
        } else {
            from + batch_size - 1
        };
        res.push(BlockBatch { from, to });
        from = from + batch_size
    }
    res
}

#[derive(Debug, Clone)]
pub struct Scanner {
    min_block: u64,
    max_block: Option<u64>,
    batch_size: u64,
}

pub fn scan(
    rpc_address: &str,
    contract_address: H160,
    min_block: u64,
    max_block: Option<u64>,
    batch_size: u64,
    handler: &mut dyn EventHandler,
) -> anyhow::Result<u64> {
    let mut last_block = min_block;
    let client = EthClient::new(rpc_address);
    for b in get_batches(rpc_address, min_block, max_block, batch_size) {
        debug!("reading blocks {:?} from {}", b, rpc_address);
        let filter = FilterBuilder::default()
            .from_block(b.from.into())
            .to_block(b.to.into())
            .address(vec![contract_address])
            .build();
        let logs: Vec<Log> = match client.get_logs(&filter) {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        if logs.len() > 0 {
            for l in logs {
                handler.on(l.clone());
            }
        }
        last_block = b.to;
    }
    Ok(last_block)
}
