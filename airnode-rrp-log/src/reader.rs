use tracing::debug;
use web3::api::Eth;
use web3::transports::{Either, Http, Ipc};
use web3::types::{FilterBuilder, Log, H160};
use web3::{Transport, Web3};

pub trait EventHandler {
    fn on(&mut self, l: Log, pretty_print: bool) -> ();
}

pub async fn get_transport(source: &str) -> Either<Http, Ipc> {
    if source.contains(".ipc") {
        let transport = Ipc::new(source)
            .await
            .expect("Failed to connect to IPC file");
        debug!("Connected to {:?}", source);
        Either::Right(transport)
    } else {
        let transport = Http::new(source).expect("Invalid RPC HTTP endpoint");
        debug!("Connecting to {:?}", source);
        Either::Left(transport)
    }
}

#[derive(Clone, Debug)]
pub struct BlockBatch {
    pub from: u64,
    pub to: u64,
}

pub async fn get_batches<T: Transport>(
    eth: Eth<T>,
    genesis: u64,
    max: Option<u64>,
    batch_size: u64,
) -> Vec<BlockBatch> {
    let max_block: u64 = match max {
        Some(x) => x,
        None => eth
            .block_number()
            .await
            .expect("max block height failure")
            .as_u64(),
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
    chain_id: u64,
    min_block: u64,
    max_block: Option<u64>,
    batch_size: u64,
}

impl Scanner {
    pub fn new(chain_id: u64, min_block: u64, max_block: Option<u64>, batch_size: u64) -> Self {
        Self {
            chain_id,
            min_block,
            max_block,
            batch_size,
        }
    }

    pub async fn scan_address<T>(
        &mut self,
        web3: &Web3<T>,
        address: H160,
        handler: &mut impl EventHandler,
        pretty_print: bool,
    ) -> anyhow::Result<u64>
    where
        T: Transport,
    {
        let chain_id = self.chain_id;
        let mut last_block = self.min_block;
        for b in get_batches(web3.eth(), self.min_block, self.max_block, self.batch_size).await {
            debug!("reading blocks {:?}/{}", b, chain_id);
            let filter = FilterBuilder::default()
                .from_block(b.from.into())
                .to_block(b.to.into())
                .address(vec![address])
                .build();
            let logs: Vec<Log> = web3.eth().logs(filter).await?;
            if logs.len() > 0 {
                for l in logs {
                    handler.on(l.clone(), pretty_print);
                }
            }
            last_block = b.to;
        }
        Ok(last_block)
    }
}
