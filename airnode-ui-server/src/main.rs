pub mod airnode_config;
pub mod airnode_ops;
pub mod airnode_state;
pub mod args;
pub mod endpoints;
pub mod fees;
pub mod nice;
pub mod storage_config;
pub mod storage_ops;
pub mod storage_state;
pub mod usdprice;
pub mod web3sync;

use crate::storage_config::KVStore;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct State {
    pub db_config: storage_config::Storage,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => {
            panic!("Args parsing error: {}", e);
        }
    };

    let db_config = storage_config::Storage::init(&args.data_dir);
    let state = Arc::new(Mutex::new(State { db_config }));
    let socket_addr: std::net::SocketAddr = args.listen.parse().expect("invalid bind to listen");
    warp::serve(endpoints::routes(state)).run(socket_addr).await;
    Ok(())
}
