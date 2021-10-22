use crate::airnode_config::AirnodeConfigCmd;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]

pub enum Command {
    Config(AirnodeConfigCmd),
    Server,
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "", about = "RPC Apps management CLI utility")]

pub struct Args {
    #[structopt(short, long, env = "DATA_DIR")]
    pub data_dir: String,
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

