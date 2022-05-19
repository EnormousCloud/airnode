use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;
use tracing_subscriber::prelude::*;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Jsonl,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseFormatError;
impl fmt::Display for ParseFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided output format was not `json` or `jsonl`".fmt(f)
    }
}

impl FromStr for OutputFormat {
    type Err = ParseFormatError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "json" => Ok(OutputFormat::Json),
            "jsonl" => Ok(OutputFormat::Jsonl),
            "" => Ok(OutputFormat::Jsonl),
            _ => Err(ParseFormatError),
        }
    }
}

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
    #[structopt(long, env = "RPC_BATCH_SIZE")]
    pub rpc_batch_size: Option<u64>,
    /// Number of the first block to start watching
    #[structopt(long, env = "RPC_MIN_BLOCK")]
    pub min_block: Option<u64>,
    /// Max block to stop contract events listening
    #[structopt(long, env = "RPC_MAX_BLOCK")]
    pub max_block: Option<u64>,
    /// API3 secondary voting agent address
    #[structopt(long, env = "ADDR_CONTRACT")]
    pub address_contract: String,
    /// format of output: "jsonl" or "json"
    #[structopt(short, long, default_value = "jsonl", case_insensitive = true)]
    pub format: OutputFormat,
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
