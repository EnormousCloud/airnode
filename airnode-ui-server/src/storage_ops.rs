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
        opts.create_missing_column_families(true);
        opts.set_use_fsync(false);
        opts.set_keep_log_file_num(1);

        let mut cf_opts = Options::default();
        cf_opts.create_if_missing(true);

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

#[cfg(test)]
mod tests {
    use super::*;
    use airnode_events::AirnodeEvent;
    use fake::{Fake, Faker};
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use std::env;
    use web3::types::{H160, H256};

    /// converts array of bytes into fixed array of 32 or panic
    pub fn into32(src: &[u8]) -> [u8; 32] {
        src.try_into()
            .expect(format!("slice with incorrect length of {}", src.len()).as_str())
    }

    /// converts array of bytes into fixed array of 16 or panic
    pub fn into20(src: &[u8]) -> [u8; 20] {
        src.try_into()
            .expect(format!("slice with incorrect length of {}", src.len()).as_str())
    }

    fn random_operation() -> Operation {
        let seed = into32(&fake::vec![u8; 32]);
        let ref mut r = StdRng::from_seed(seed);
        let topic = H256::from(into32(&fake::vec![u8; 32]));
        let event = AirnodeEvent::Unknown { topic };
        Operation {
            timestamp: Faker.fake_with_rng(r),
            event,
            height: Faker.fake_with_rng(r),
            from: None,
            block: H256::from(into32(&fake::vec![u8; 32])),
            transaction_hash: H256::from(into32(&fake::vec![u8; 32])),
            tx_index: (8..20).fake::<u64>(),
            log_index: (8..20).fake::<u64>(),
            fees: None,
        }
    }

    #[test]
    pub fn it_appends_and_reads() {
        let current_dir = env::current_dir().unwrap();
        let data_dir = format!("{}/_data", current_dir.as_os_str().to_str().unwrap());
        let address: H160 = H160::from(into20(&fake::vec![u8; 20]));
        let db = Storage::init(&data_dir, address);
        let first = random_operation();
        db.append(&first);
        let second = random_operation();
        db.append(&second);
        let ops = db.list();
        assert_eq!(ops.len(), 2);
        println!("{}", serde_json::to_string(&ops[0]).unwrap());
        println!("{}", serde_json::to_string(&ops[1]).unwrap());
        rocksdb::DB::destroy(&rocksdb::Options::default(), &data_dir).unwrap();
    }
}
