use crate::airnode_config::{AirnodeConfig, AirnodeConfigCmd, AirnodeRef};
use crate::storage_config;
use crate::storage_config::KVStore;
use crate::storage_ops;
use crate::storage_ops::LogIndex;
use crate::AppState;
use std::collections::BTreeMap as Map;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;
use warp::reply::{json, with_status, Response};
use warp::{Filter, Rejection, Reply};
use web3::types::H160;

pub fn json_error(code: StatusCode, msg: &str) -> Response {
    let mut res: Map<String, String> = Map::new();
    res.insert("error".to_owned(), msg.to_string());
    with_status(json(&res), code).into_response()
}

pub fn api_nodes(data: &storage_config::Storage) -> Response {
    let res = data.list();
    with_status(json(&res), StatusCode::OK).into_response()
}

pub fn api_node(data: &storage_config::Storage, chain_id: u64, contract: H160) -> Response {
    let found = data.find(&AirnodeRef::new(chain_id, contract));
    match found {
        Some(res) => with_status(json(&res), StatusCode::OK).into_response(),
        None => json_error(StatusCode::NOT_FOUND, "airnode node found"),
    }
}

pub fn api_node_add(
    data: &storage_config::Storage,
    contract_address: H160,
    rpc_address: &str,
    min_block: Option<u64>,
    batch_size: Option<u64>,
) -> Response {
    let config = match AirnodeConfig::new(&rpc_address, contract_address, min_block, batch_size) {
        Ok(x) => x,
        Err(e) => return json_error(StatusCode::BAD_REQUEST, &e.to_string()),
    };
    let _ = data.save(&config);
    with_status(json(&config), StatusCode::CREATED).into_response()
}

pub fn api_node_update(
    data: &storage_config::Storage,
    contract_address: H160,
    chain_id: u64,
    rpc_address: &str,
    batch_size: Option<u64>,
) -> Response {
    let found = data.find(&AirnodeRef::new(chain_id, contract_address));
    match found {
        Some(config) => {
            let mut cfg = config.clone();
            if rpc_address.len() > 0 {
                cfg.rpc_address = rpc_address.to_string();
            }
            if let Some(bs) = batch_size {
                cfg.batch_size = Some(bs);
            }
            data.save(&cfg);
            with_status(json(&cfg), StatusCode::OK).into_response()
        }
        None => json_error(StatusCode::NOT_FOUND, "airnode not found"),
    }
}

pub fn api_node_delete(
    data: &storage_config::Storage,
    contract_address: H160,
    chain_id: u64,
) -> Response {
    let found = data.find(&AirnodeRef::new(chain_id, contract_address));
    match found {
        Some(_) => {
            let res = data.delete(&AirnodeRef::new(chain_id, contract_address));
            with_status(json(&res), StatusCode::OK).into_response()
        }
        None => json_error(StatusCode::NOT_FOUND, "airnode not found"),
    }
}

fn with_state(
    state: Arc<Mutex<AppState>>,
) -> impl Filter<Extract = (Arc<Mutex<AppState>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

fn with_json_config() -> impl Filter<Extract = (AirnodeConfigCmd,), Error = warp::Rejection> + Clone
{
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn routes_nodes(
    state: Arc<Mutex<AppState>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let h_get = warp::path!("api" / "nodes" / u64 / H160)
        .and(with_state(state.clone()))
        .map(
            |chain_id: u64, addr: H160, state_rc: Arc<Mutex<AppState>>| {
                let db = &state_rc.lock().unwrap().db_config;
                api_node(db, chain_id, addr)
            },
        );
    let h_list = warp::path!("api" / "nodes")
        .and(with_state(state.clone()))
        .map(|state_rc: Arc<Mutex<AppState>>| {
            let db = &state_rc.lock().unwrap().db_config;
            api_nodes(&db)
        });
    let h_add = warp::path!("api" / "nodes")
        .and(warp::post())
        .and(with_json_config())
        .and(with_state(state.clone()))
        .map(|cmd: AirnodeConfigCmd, state_rc: Arc<Mutex<AppState>>| {
            let db = &state_rc.lock().unwrap().db_config;
            match cmd {
                AirnodeConfigCmd::Add {
                    contract_address,
                    rpc_address,
                    min_block,
                    batch_size,
                } => api_node_add(&db, contract_address, &rpc_address, min_block, batch_size),
                _ => return json_error(StatusCode::BAD_REQUEST, "bad request to add node"),
            }
        });
    let h_update = warp::path!("api" / "nodes")
        .and(warp::put())
        .and(with_json_config())
        .and(with_state(state.clone()))
        .map(|cmd: AirnodeConfigCmd, state_rc: Arc<Mutex<AppState>>| {
            let db = &state_rc.lock().unwrap().db_config;
            match cmd {
                AirnodeConfigCmd::Update {
                    contract_address,
                    chain_id,
                    rpc_address,
                    batch_size,
                } => api_node_update(&db, contract_address, chain_id, &rpc_address, batch_size),
                _ => return json_error(StatusCode::BAD_REQUEST, "bad request to add node"),
            }
        });
    let h_delete = warp::path!("api" / "nodes" / u64 / H160)
        .and(warp::delete())
        .and(with_state(state.clone()))
        .map(
            |chain_id: u64, contract_address: H160, state_rc: Arc<Mutex<AppState>>| {
                let db = &state_rc.lock().unwrap().db_config;
                return api_node_delete(&db, contract_address, chain_id);
            },
        );
    h_get.or(h_list).or(h_add).or(h_update).or(h_delete)
}

pub fn api_operations(data: &storage_ops::Storage) -> Response {
    let res = data.rev_list();
    with_status(json(&res), StatusCode::OK).into_response()
}

pub fn routes_ops(
    state: Arc<Mutex<AppState>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let h_list = warp::path!("api" / "operations" / u64 / H160)
        .and(with_state(state.clone()))
        .map(
            |chain_id: u64, contract: H160, state_rc: Arc<Mutex<AppState>>| {
                let node = AirnodeRef::new(chain_id, contract);
                match state_rc.lock().unwrap().db_ops.get(&node) {
                    Some(db) => api_operations(db),
                    None => return json_error(StatusCode::NOT_FOUND, "airnode not found"),
                }
            },
        );
    h_list
}

pub fn routes_states(
    state: Arc<Mutex<AppState>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let h_get = warp::path!("api" / "states" / u64 / H160)
        .and(with_state(state.clone()))
        .map(
            |chain_id: u64, contract: H160, state_rc: Arc<Mutex<AppState>>| {
                let node = AirnodeRef::new(chain_id, contract);
                match state_rc.lock().unwrap().states.get(&node) {
                    Some(state) => with_status(json(state), StatusCode::OK).into_response(),
                    None => return json_error(StatusCode::NOT_FOUND, "airnode not found"),
                }
            },
        );
    let h_list = warp::path!("api" / "states")
        .and(with_state(state.clone()))
        .map(|state_rc: Arc<Mutex<AppState>>| {
            let mut out = vec![];
            for (_, nodestate) in &state_rc.lock().unwrap().states {
                out.push(nodestate.clone());
            }
            with_status(json(&out), StatusCode::OK).into_response()
        });
    h_get.or(h_list)
}

pub fn routes(
    state: Arc<Mutex<AppState>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    routes_nodes(state.clone())
        .or(routes_ops(state.clone()))
        .or(routes_states(state))
}
