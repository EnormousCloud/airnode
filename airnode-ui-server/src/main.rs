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
        Command::Server => {
            let state = Arc::new(Mutex::new(State {
                db_config,
                db_ops: Map::new(),
            }));
            let socket_addr: std::net::SocketAddr =
                args.listen.parse().expect("invalid bind to listen");
            warp::serve(endpoints::routes(state)).run(socket_addr).await;
        }
    }

    Ok(())
}
