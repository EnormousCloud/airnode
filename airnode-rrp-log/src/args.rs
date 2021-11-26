use structopt::StructOpt;
use tracing_subscriber::prelude::*;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "airnode-rrp-log",
    about = "API3 Airnode smart contract request/response log viewer"
)]
pub struct Args {
    /// Ethereum JSON+RPC HTTP address
    #[structopt(long, default_value = "http://localhost:8545/", env = "RPC_ENDPOINT")]
    pub rpc_endpoint: String,
    /// Ethereum JSON+RPC batch size for reading. Light clients will require smaller sizes
    #[structopt(long, default_value = "1000", env = "RPC_BATCH_SIZE")]
    pub rpc_batch_size: u64,
    /// Number of the first block to start watching
    #[structopt(long, default_value = "1", env = "RPC_MIN_BLOCK")]
    pub min_block: u64,
    /// Max block to stop contract events listening
    #[structopt(long, env = "RPC_MAX_BLOCK")]
    pub max_block: Option<u64>,
    /// API3 secondary voting agent address
    #[structopt(long, env = "ADDR_CONTRACT")]
    pub address_contract: String,

    /// Pretty print JSON responses
    #[structopt(long)]
    pub pretty_print: bool,
    #[structopt(long, env = "BY_PROVIDER_ID")]
    pub by_provider_id: Option<String>,
    #[structopt(long, env = "BY_ENDPOINT_ID")]
    pub by_endpoint_id: Option<String>,
    #[structopt(long, env = "BY_TEMPLATE_ID")]
    pub by_template_id: Option<String>,
    #[structopt(long, env = "BY_REQUEST_ID")]
    pub by_request_id: Option<String>,
    #[structopt(long, env = "BY_ENDPOINTER_INDEX")]
    pub by_requester_index: Option<u64>,
    #[structopt(long, env = "BY_ADDRESS")]
    pub by_address: Option<String>,
    #[structopt(long, env = "BY_AIRNODE")]
    pub by_airnode: Option<String>,
}

pub fn parse() -> anyhow::Result<Args> {
    dotenv::dotenv().ok();
    let log_level: String = std::env::var("LOG_LEVEL").unwrap_or("info".to_owned());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_ansi(false)
        .with_level(false)
        .with_target(false);
    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::EnvFilter::try_new(&log_level))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    let res = Args::from_args();
    tracing::debug!("{:?}", res);
    Ok(res)
}
