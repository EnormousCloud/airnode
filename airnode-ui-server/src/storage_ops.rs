use crate::airnode_ops::Operation;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use std::sync::Arc;
use web3::types::H160;

pub trait LogIndex {
    fn init(data_dir: &str, airnode: H160) -> Self;
    fn append(&self, v: &Operation) -> bool;
    fn list(&self) -> Vec<Operation>;
}

#[derive(Clone)]
pub struct Storage {
    db: Arc<DB>,
}

impl LogIndex for Storage {
    fn init(data_dir: &str, airnode: H160) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_use_fsync(false);
        opts.set_keep_log_file_num(1);

        let mut cf_opts = Options::default();
        cf_opts.set_max_write_buffer_number(16);

        let file_path = format!("{}/{:?}", data_dir, airnode);
        let cf = ColumnFamilyDescriptor::new("log", cf_opts);
        Self {
            db: Arc::new(DB::open_cf_descriptors(&opts, file_path, vec![cf]).unwrap()),
        }
    }

    fn append(&self, v: &Operation) -> bool {
        self.db.put(v.as_ref().as_bytes(), v.as_bytes()).is_ok()
    }

    fn list(&self) -> Vec<Operation> {
        let mut out = vec![];
        let mut iter = self.db.raw_iterator();
        iter.seek_to_first();
        while iter.valid() {
            if let Ok(op) = Operation::from_bytes(iter.value().unwrap()) {
                out.push(op.clone())
            }
            iter.next();
        }
        out
    }
}
