use crate::airnode_config::AirnodeConfigCmd;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]

pub enum Command {
    /// Nodes configuration: commands manage Airnode RRP contracts
    Config(AirnodeConfigCmd),
    /// Start HTTP server
    Server,
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "", about = "RPC Apps management CLI utility")]

pub struct Args {
    /// Directory for persistent data storage
    #[structopt(short, long, default_value = "./_data", env = "DATA_DIR")]
    pub data_dir: String,
    /// Command to exectue
    #[structopt(subcommand)]
    pub cmd: Command,
    /// Net listening address of HTTP server in case of "server" command
    #[structopt(long, default_value = "0.0.0.0:8000", env = "LISTEN")]
    pub listen: String,
}

pub fn parse() -> anyhow::Result<Args> {
    dotenv::dotenv().ok();
    let log_level: String = std::env::var("LOG_LEVEL").unwrap_or("debug".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(log_level))
        .init();
    let res = Args::from_args();
    tracing::debug!("{:?}", res);
    Ok(res)
}
