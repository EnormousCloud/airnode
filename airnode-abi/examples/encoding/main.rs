use airnode_abi::{Param, ABI};
use ethereum_types::U256;

fn main() {
    let param = Param::String {
        name: "hello".to_owned(),
        value: "world".to_owned(),
    };
    let res: Vec<U256> = ABI::new(vec![param]).encode().unwrap();
    println!("{:#?}", res);
}
