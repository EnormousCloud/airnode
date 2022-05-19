use tracing::debug;
use web3::api::Eth;
use web3::transports::Http;
use web3::types::{FilterBuilder, Log, H160};
use web3::{Transport, Web3};

pub trait EventHandler {
    fn on(&mut self, l: Log) -> ();
}

pub async fn get_transport(source: &str) -> Http {
    let transport = Http::new(source).expect("Invalid RPC HTTP endpoint");
    debug!("Connecting to {:?}", source);
    transport
}

#[derive(Clone, Debug, Default)]
pub struct BlockBatch {
    pub from: Option<u64>,
    pub to: Option<u64>,
}

pub async fn get_batches<T: Transport>(
    eth: Eth<T>,
    min: Option<u64>,
    max: Option<u64>,
    batch_size: Option<u64>,
) -> Vec<BlockBatch> {
    if let None = min {
        if let None = max {
            return vec![BlockBatch::default()];
        }
    }
    let b_size = match batch_size {
        Some(x) => x,
        None => {
            // in case of batch size absense - we have a single batch
            return vec![BlockBatch { from: min, to: max }];
        }
    };

    let max_block: u64 = match max {
        Some(x) => x,
        None => eth
            .block_number()
            .await
            .expect("max block height failure")
            .as_u64(),
    };
    let mut from: u64 = match min {
        Some(x) => x,
        None => 1,
    };

    let mut res = vec![];
    while from <= max_block {
        let to = if from + b_size > max_block {
            max_block
        } else {
            from + b_size - 1
        };
        res.push(BlockBatch {
            from: Some(from),
            to: Some(to),
        });
        from = from + b_size
    }
    res
}

#[derive(Debug, Clone)]
pub struct Scanner {
    chain_id: u64,
    min_block: Option<u64>,
    max_block: Option<u64>,
    batch_size: Option<u64>,
}

impl Scanner {
    pub fn new(
        chain_id: u64,
        min_block: Option<u64>,
        max_block: Option<u64>,
        batch_size: Option<u64>,
    ) -> Self {
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
    ) -> anyhow::Result<Option<u64>>
    where
        T: Transport,
    {
        let chain_id = self.chain_id;
        let mut last_block = self.min_block;
        let batches =
            get_batches(web3.eth(), self.min_block, self.max_block, self.batch_size).await;
        for b in batches {
            debug!("reading blocks {:?}/{}", b, chain_id);
            let mut filter = FilterBuilder::default().address(vec![address]);
            if let Some(from) = b.from {
                filter = filter.from_block(from.into());
            }
            if let Some(to) = b.to {
                filter = filter.to_block(to.into());
            }
            let logs: Vec<Log> = web3.eth().logs(filter.build()).await?;
            if logs.len() > 0 {
                for l in logs {
                    handler.on(l.clone());
                }
            }
            last_block = b.to;
        }
        Ok(last_block)
    }
}
