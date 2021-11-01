use crate::airnode_config::AirnodeRef;
use crate::storage_config;
use crate::storage_config::KVStore;
use std::collections::BTreeMap as Map;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;
use warp::{Filter, Reply};
use web3::types::H160;

pub fn json_error(code: StatusCode, msg: &str) -> warp::reply::Response {
    let mut res: Map<String, String> = Map::new();
    res.insert("error".to_owned(), msg.to_string());
    let body = warp::reply::json(&res);
    warp::reply::with_status(body, code).into_response()
}

pub fn nodes(data: &storage_config::Storage) -> impl warp::Reply {
    let res = data.list();
    let body = warp::reply::json(&res);
    warp::reply::with_status(body, warp::http::StatusCode::OK).into_response()
}

pub fn node(chain_id: u64, contract: H160, data: &storage_config::Storage) -> impl warp::Reply {
    let found = data.find(&AirnodeRef::new(chain_id, contract));
    match found {
        Some(res) => {
            let body = warp::reply::json(&res);
            return warp::reply::with_status(body, warp::http::StatusCode::OK).into_response();
        }
        None => return json_error(StatusCode::NOT_FOUND, "airnode node found"),
    }
}

pub fn routes(
    state: Arc<Mutex<crate::State>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_node = warp::path!("api" / "node" / u64 / H160).map({
        let state_rc = state.clone();
        move |chain_id: u64, addr: H160| {
            let state = state_rc.lock().unwrap();
            node(chain_id, addr, &state.db_config)
        }
    });

    let api_nodes = warp::path!("api" / "nodes").map({
        let state_rc = state.clone();
        move || {
            let state = state_rc.lock().unwrap();
            nodes(&state.db_config)
        }
    });
    api_node.or(api_nodes)
}
