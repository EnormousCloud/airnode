pub mod airnode_config;
pub mod airnode_ops;
pub mod airnode_state;
pub mod args;
pub mod fees;
pub mod nice;
pub mod storage_config;
pub mod storage_ops;
pub mod storage_state;
pub mod usdprice;
pub mod web3sync;

use crate::airnode_config::AirnodeConfig;
use crate::storage_config::KVStore;
use hex_literal::hex;
use std::sync::{Arc, Mutex};
use warp::{Filter, Reply};

#[derive(Clone)]
pub struct State {
    pub db_config: storage_config::Storage,
}

pub fn nodes(data: &storage_config::Storage) -> impl warp::Reply {
    let res = vec![AirnodeConfig {
        chain_id: 1,
        contract_address: hex!("0000000000000000000000000000000000000000").into(),
        rpc_address: "http://localhost:8545/".to_owned(),
        min_block: None,
        batch_size: None,
    }];
    let body = warp::reply::json(&res);
    warp::reply::with_status(body, warp::http::StatusCode::OK).into_response()
}

pub fn routes(
    state: Arc<Mutex<crate::State>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_nodes = warp::path!("api" / "nodes").map({
        let state_rc = state.clone();
        move || {
            let state = state_rc.lock().unwrap();
            nodes(&state.db_config)
        }
    });
    api_nodes
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
    warp::serve(routes(state)).run(socket_addr).await;
    Ok(())
}
