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
                println!("key = {:?}", String::from_utf8_lossy(iter.key().unwrap()));
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

#[cfg(test)]
mod tests {
    use super::*;
    use fake::Fake;
    use std::env;
    use web3::types::H160;

    /// converts array of bytes into fixed array of 16 or panic
    pub fn into20(src: &[u8]) -> [u8; 20] {
        src.try_into()
            .expect(format!("slice with incorrect length of {}", src.len()).as_str())
    }

    fn random_config() -> AirnodeConfig {
        let contract_address = H160::from(into20(&fake::vec![u8; 20]));
        AirnodeConfig {
            chain_id: (8..20).fake::<u64>(),
            contract_address,
            rpc_address: "http://localhost:8545/".to_owned(),
            min_block: None,
            batch_size: None,
        }
    }

    #[test]
    pub fn it_saves_1_reads_1() {
        let current_dir = env::current_dir().unwrap();
        let rnd_name = (8..20).fake::<String>();
        let data_dir = format!(
            "{}/_data/{}",
            current_dir.as_os_str().to_str().unwrap(),
            rnd_name
        );
        let db = Storage::init(&data_dir);
        let config = random_config();
        db.save(&config);
        let nodes = db.list();
        assert_eq!(nodes.len(), 1);
        assert_eq!(
            serde_json::to_string(&nodes[0]).unwrap(),
            serde_json::to_string(&config).unwrap()
        );
        println!("{}", serde_json::to_string(&nodes).unwrap());
        let node = AirnodeRef::new(config.chain_id, config.contract_address);
        let found_config = db.find(&node).unwrap();
        println!("{}", serde_json::to_string(&found_config).unwrap());
        rocksdb::DB::destroy(&rocksdb::Options::default(), &data_dir).unwrap();
    }
}
