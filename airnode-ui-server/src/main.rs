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

use crate::airnode_config::{AirnodeConfig, AirnodeConfigCmd, AirnodeRef};
use crate::airnode_ops::AirnodeOpsCmd;
use crate::airnode_state::{AirnodeState, AirnodeStateCmd};
use crate::args::Command;
use crate::storage_config::KVStore;
use crate::storage_ops::LogIndex;
use std::collections::BTreeMap as Map;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use web3::types::H160;

#[derive(Clone)]
pub struct State {
    pub db_config: storage_config::Storage,
    pub db_ops: Map<AirnodeRef, storage_ops::Storage>,
    pub db_state: Map<AirnodeRef, AirnodeState>,
}

pub fn cli_config(db_config: storage_config::Storage, cmd: AirnodeConfigCmd) -> anyhow::Result<()> {
    match cmd {
        AirnodeConfigCmd::Add {
            contract_address,
            rpc_address,
            min_block,
            batch_size,
        } => {
            let config =
                AirnodeConfig::new(&rpc_address, contract_address, min_block, batch_size).unwrap(); // failure if there is no connection
            let _ = db_config.save(&config);
        }
        AirnodeConfigCmd::Get {
            contract_address,
            chain_id,
        } => {
            let found = db_config.find(&AirnodeRef::new(chain_id, contract_address));
            match found {
                Some(config) => {
                    println!("{}", serde_json::to_string(&config).unwrap());
                }
                None => println!(
                    "ERROR: Contract {}, chain_id {} not found",
                    contract_address, chain_id,
                ),
            }
        }
        AirnodeConfigCmd::List => {
            let list = db_config.list();
            println!("{}", serde_json::to_string(&list).unwrap());
        }
        AirnodeConfigCmd::Update {
            contract_address,
            chain_id,
            rpc_address,
            batch_size,
        } => {
            let found = db_config.find(&AirnodeRef::new(chain_id, contract_address));
            match found {
                Some(config) => {
                    let mut cfg = config.clone();
                    if rpc_address.len() > 0 {
                        cfg.rpc_address = rpc_address;
                    }
                    if let Some(bs) = batch_size {
                        cfg.batch_size = Some(bs);
                    }
                    db_config.save(&cfg);
                    println!("UPDATED");
                }
                None => println!(
                    "ERROR: Contract {}, chain_id {} not found",
                    contract_address, chain_id,
                ),
            }
        }
        AirnodeConfigCmd::Delete {
            contract_address,
            chain_id,
        } => {
            let found = db_config.find(&AirnodeRef::new(chain_id, contract_address));
            match found {
                Some(config) => {
                    db_config.delete(&AirnodeRef {
                        chain_id: config.chain_id,
                        contract_address: config.contract_address,
                    });
                    println!("DELETED");
                }
                None => println!(
                    "ERROR: Contract {}, chain_id {} not found",
                    contract_address, chain_id,
                ),
            }
        }
    }
    Ok(())
}

pub fn cli_op(db_ops: storage_ops::Storage, cmd: AirnodeOpsCmd) -> anyhow::Result<()> {
    match cmd {
        AirnodeOpsCmd::Truncate => {
            let _ = db_ops.truncate();
            println!("OK");
        }
        AirnodeOpsCmd::List => {
            let list = db_ops.list();
            println!("{}", serde_json::to_string(&list).unwrap());
        }
    }
    Ok(())
}

pub fn cli_state(
    db_config: storage_config::Storage,
    chain_id: u64,
    contract_str: &str,
    data_dir: &str,
    cmd: AirnodeStateCmd,
) -> anyhow::Result<()> {
    match cmd {
        AirnodeStateCmd::Get => {
            let contract = H160::from_str(&contract_str).expect("Contract address is required");
            let node = AirnodeRef::new(chain_id, contract);
            let mut state: AirnodeState = AirnodeState::new(&node);
            let db_ops = storage_ops::Storage::init(&data_dir, node);
            for op in db_ops.list() {
                // state.handle_op(&op);
            }
            println!("{}", serde_json::to_string(&state).unwrap());
        }
        AirnodeStateCmd::List => {
            let rc_list: Arc<Mutex<Vec<AirnodeState>>> = Arc::new(Mutex::new(vec![]));
            let dir: Arc<String> = Arc::new(data_dir.to_string());
            let mut threads = vec![];
            for config in db_config.list() {
                let rcc = rc_list.clone();
                let data_path = dir.clone();
                threads.push(std::thread::spawn(move || {
                    let node = AirnodeRef::new(config.chain_id, config.contract_address);
                    let mut state: AirnodeState = AirnodeState::new(&node);
                    let db_ops = storage_ops::Storage::init(&data_path, node);
                    for op in db_ops.list() {
                        // state.handle_op(&op);
                    }
                    let mut rc = rcc.lock().unwrap();
                    rc.push(state.clone());
                }));
            }
            // wait for result
            for t in threads {
                let _ = t.join();
            }
            let rcc = rc_list.clone();
            let rc = rcc.lock().unwrap();
            println!("{}", serde_json::to_string(rc.as_slice()).unwrap());
        }
    }
    Ok(())
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
    match args.cmd {
        Command::Config(cmd) => {
            return cli_config(db_config, cmd);
        }
        Command::Op(cmd) => {
            let chain_id = args.chain_id;
            let contract = H160::from_str(&args.contract).expect("Contract address is required");
            let node = AirnodeRef::new(chain_id, contract);
            let db_ops = storage_ops::Storage::init(&args.data_dir, node);
            return cli_op(db_ops, cmd);
        }
        Command::State(cmd) => {
            return cli_state(
                db_config,
                args.chain_id,
                &args.contract,
                &args.data_dir,
                cmd,
            );
        }
        Command::Server => {
            let mut db_ops = Map::new();
            let mut db_state = Map::new();
            for node in db_config.list() {
                let key = AirnodeRef::new(node.chain_id, node.contract_address);
                db_ops.insert(key.clone(), storage_ops::Storage::init(&args.data_dir, key));
            }
            let state = Arc::new(Mutex::new(State {
                db_config,
                db_ops,
                db_state,
            }));
            let socket_addr: std::net::SocketAddr =
                args.listen.parse().expect("invalid bind to listen");
            warp::serve(endpoints::routes(state)).run(socket_addr).await;
        }
    }

    Ok(())
}
