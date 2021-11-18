pub mod airnode_config;
pub mod airnode_ops;
pub mod airnode_state;
pub mod args;
pub mod endpoints;
pub mod fees;
pub mod nice;
pub mod reader;
pub mod storage_config;
pub mod storage_ops;
pub mod usdprice;
pub mod web3sync;

use crate::airnode_config::{AirnodeConfig, AirnodeConfigCmd, AirnodeRef};
use crate::airnode_ops::AirnodeOpsCmd;
use crate::airnode_state::{AirnodeState, AirnodeStateCmd};
use crate::args::Command;
use crate::storage_config::KVStore;
use crate::storage_ops::LogIndex;
use std::collections::BTreeMap as Map;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db_config: storage_config::Storage,
    pub db_ops: Map<AirnodeRef, storage_ops::Storage>,
    pub states: Map<AirnodeRef, AirnodeState>,
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
            println!("{}", serde_json::to_string(&config).unwrap());
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

pub struct OpSaver {
    pub rpc_addr: String,
    pub db_ops: storage_ops::Storage,
}

impl reader::EventHandler for OpSaver {
    fn on(&mut self, l: web3::types::Log) {
        let op = airnode_ops::Operation::new(&l, &self.rpc_addr).unwrap();
        self.db_ops.append(&op);
    }
}

pub fn cli_op(
    db_config: storage_config::Storage,
    db_ops: storage_ops::Storage,
    cmd: AirnodeOpsCmd,
) -> anyhow::Result<()> {
    match cmd {
        AirnodeOpsCmd::Truncate { .. } => {
            let _ = db_ops.truncate();
            println!("OK");
        }
        AirnodeOpsCmd::List {
            chain_id,
            contract_address,
            no_sync,
        } => {
            if !no_sync {
                let node = AirnodeRef::new(chain_id, contract_address);
                let config = db_config.find(&node).unwrap();
                let min_block = std::cmp::max(config.min_block.unwrap(), db_ops.max_height() + 1);
                tracing::info!(
                    "rrp contract {} scanning from block {}",
                    contract_address,
                    min_block
                );
                crate::reader::scan(
                    &config.rpc_address,
                    contract_address,
                    min_block,
                    None,
                    config.batch_size.unwrap(),
                    &mut OpSaver {
                        db_ops: db_ops.clone(),
                        rpc_addr: config.rpc_address.to_string(),
                    },
                )
                .unwrap();
            }
            let list = db_ops.list();
            println!("{}", serde_json::to_string(&list).unwrap());
        }
    }
    Ok(())
}

pub fn cli_state(
    db_config: storage_config::Storage,
    data_dir: &str,
    cmd: AirnodeStateCmd,
) -> anyhow::Result<()> {
    match cmd {
        AirnodeStateCmd::Get {
            chain_id,
            contract_address,
            no_sync,
        } => {
            let node = AirnodeRef::new(chain_id, contract_address);
            let config = db_config.find(&node).expect("airnode not found");
            let mut state: AirnodeState = AirnodeState::new(&node);
            let db_ops = storage_ops::Storage::init(&data_dir, node);

            if !no_sync {
                let min_block = std::cmp::max(config.min_block.unwrap(), db_ops.max_height() + 1);
                tracing::info!(
                    "rrp contract {} scanning from block {}",
                    contract_address,
                    min_block
                );
                crate::reader::scan(
                    &config.rpc_address,
                    contract_address,
                    min_block,
                    None,
                    config.batch_size.unwrap(),
                    &mut OpSaver {
                        db_ops: db_ops.clone(),
                        rpc_addr: config.rpc_address.to_string(),
                    },
                )
                .unwrap();
            }

            for op in db_ops.list() {
                state.handle_op(&op);
            }
            state.update_balance(&config.rpc_address);
            println!("{}", serde_json::to_string(&state).unwrap());
        }
        AirnodeStateCmd::List { no_sync } => {
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
                    if !no_sync {
                        let min_block =
                            std::cmp::max(config.min_block.unwrap(), db_ops.max_height() + 1);
                        tracing::info!(
                            "rrp contract {} scanning from block {}",
                            config.contract_address,
                            min_block
                        );
                        crate::reader::scan(
                            &config.rpc_address,
                            config.contract_address,
                            min_block,
                            None,
                            config.batch_size.unwrap(),
                            &mut OpSaver {
                                db_ops: db_ops.clone(),
                                rpc_addr: config.rpc_address.to_string(),
                            },
                        )
                        .unwrap();
                    }

                    for op in db_ops.list() {
                        state.handle_op(&op);
                    }
                    state.update_balance(&config.rpc_address);
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
            let (chain_id, contract_address) = match cmd {
                AirnodeOpsCmd::List {
                    chain_id,
                    contract_address,
                    no_sync: _,
                } => (chain_id, contract_address),
                AirnodeOpsCmd::Truncate {
                    chain_id,
                    contract_address,
                } => (chain_id, contract_address),
            };
            let node = AirnodeRef::new(chain_id, contract_address);
            let db_ops = storage_ops::Storage::init(&args.data_dir, node);
            return cli_op(db_config, db_ops, cmd);
        }
        Command::State(cmd) => {
            return cli_state(db_config, &args.data_dir, cmd);
        }
        Command::Server { listen, no_sync } => {
            // connecting to databases of operations:
            let mut db_ops = Map::new();
            for node in db_config.list() {
                let key = AirnodeRef::new(node.chain_id, node.contract_address);
                db_ops.insert(key.clone(), storage_ops::Storage::init(&args.data_dir, key));
            }
            // starting threads to
            let app_state = Arc::new(Mutex::new(AppState {
                db_config: db_config.clone(),
                db_ops: db_ops.clone(),
                states: Map::new(),
            }));
            let mut threads = vec![];
            for config in db_config.list() {
                let rcc = app_state.clone();

                threads.push(std::thread::spawn(move || {
                    let node = AirnodeRef::new(config.chain_id, config.contract_address);
                    let default_state = AirnodeState::new(&node);
                    let db = {
                        let mut rc = rcc.lock().unwrap();
                        rc.states.insert(node.clone(), default_state);
                        rc.db_ops.get(&node).unwrap().clone()
                    };
                    let ops = {
                        if !no_sync {
                            let min_block =
                                std::cmp::max(config.min_block.unwrap(), db.max_height() + 1);
                            tracing::info!(
                                "rrp contract {} scanning from block {}",
                                config.contract_address,
                                min_block
                            );
                            crate::reader::scan(
                                &config.rpc_address,
                                config.contract_address,
                                min_block,
                                None,
                                config.batch_size.unwrap(),
                                &mut OpSaver {
                                    db_ops: db.clone(),
                                    rpc_addr: config.rpc_address.to_string(),
                                },
                            )
                            .unwrap();
                        }
                        db.list()
                    };
                    tracing::info!("rrp contract: found {} operations", ops.len());
                    let mut state: AirnodeState = {
                        let rc = rcc.lock().unwrap();
                        rc.states.get(&node).unwrap().clone()
                    };
                    for op in ops {
                        state.handle_op(&op);
                    }
                    state.update_balance(&config.rpc_address);
                    let mut rc = rcc.lock().unwrap();
                    rc.states.insert(node.clone(), state.clone());
                }));
            }

            let socket_addr: std::net::SocketAddr = listen.parse().expect("invalid bind to listen");
            warp::serve(endpoints::routes(app_state))
                .run(socket_addr)
                .await;
        }
    }

    Ok(())
}
