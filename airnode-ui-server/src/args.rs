use crate::airnode_config::AirnodeConfigCmd;
use crate::airnode_ops::AirnodeOpsCmd;
use crate::airnode_state::AirnodeStateCmd;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]

pub enum Command {
    /// commands to manage Airnode RRP contracts
    Config(AirnodeConfigCmd),
    /// commands to manage operations
    Op(AirnodeOpsCmd),
    /// commands to read and return state of the airnode
    State(AirnodeStateCmd),
    /// start HTTP server
    Server {
        /// Net listening address of HTTP server in case of "server" command
        #[structopt(long, default_value = "0.0.0.0:8000", env = "LISTEN")]
        listen: String,
    },
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "airnode-ui-server",
    about = "Airnode UI server and management utility"
)]

pub struct Args {
    /// Directory for persistent data storage
    #[structopt(short, long, default_value = "./_data", env = "DATA_DIR")]
    pub data_dir: String,
    /// Command to exectue
    #[structopt(subcommand)]
    pub cmd: Command,
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
