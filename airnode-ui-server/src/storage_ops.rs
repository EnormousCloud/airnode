use crate::airnode_ops::Operation;
use rocksdb::{Options, DB};
use std::sync::Arc;

pub trait LogIndex {
    fn init(data_dir: &str) -> Self;
    fn append(&self, v: &web3::types::Log) -> bool;
    fn count(&self) -> u64;
    fn list(&self) -> Vec<Operation>;
}
