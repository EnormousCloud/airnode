use crate::airnode_config::{AirnodeConfig, AirnodeRef};
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use std::sync::Arc;

pub trait KVStore {
    fn init(data_dir: &str) -> Self;
    fn save(&self, v: &AirnodeConfig) -> bool;
    fn list(&self) -> Vec<AirnodeConfig>;
    fn find(&self, k: &AirnodeRef) -> Option<AirnodeConfig>;
    fn delete(&self, k: &AirnodeRef) -> bool;
}

#[derive(Clone)]
pub struct Storage {
    db: Arc<DB>,
}

impl KVStore for Storage {
    fn init(data_dir: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_use_fsync(false);
        opts.set_keep_log_file_num(1);

        let mut cf_opts = Options::default();
        cf_opts.create_if_missing(true);

        let file_path = format!("{}/nodes", data_dir);
        let cf = ColumnFamilyDescriptor::new("nodes", cf_opts);
        Self {
            db: Arc::new(DB::open_cf_descriptors(&opts, file_path, vec![cf]).unwrap()),
        }
    }

    fn save(&self, v: &AirnodeConfig) -> bool {
        self.db.put(v.key(), v.as_bytes()).is_ok()
    }

    fn find(&self, k: &AirnodeRef) -> Option<AirnodeConfig> {
        match self.db.get(k.as_bytes()) {
            Ok(Some(v)) => {
                let res = match AirnodeConfig::from(v) {
                    Ok(x) => x,
                    Err(e) => {
                        println!("Finding '{}' throws error '{:?}'", k.to_string(), e);
                        return None;
                    }
                };
                Some(res)
            }
            Ok(None) => {
                println!("Finding '{}' returns None", k.to_string());
                None
            }
            Err(e) => {
                println!("Error retrieving value for {}: {}", k.to_string(), e);
                None
            }
        }
    }
    fn list(&self) -> Vec<AirnodeConfig> {
        let mut res = vec![];
        let mut iter = self.db.raw_iterator();
        iter.seek_to_first();
        while iter.valid() {
            if let Some(v) = iter.value() {
                if let Ok(config) = AirnodeConfig::from(v.to_vec()) {
                    res.push(config);
                }
            };
            iter.next();
        }

        res
    }

    fn delete(&self, k: &AirnodeRef) -> bool {
        self.db.delete(k.as_bytes()).is_ok()
    }
}

