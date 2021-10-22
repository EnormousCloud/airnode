pub mod airnode_config;
pub mod airnode_ops;
pub mod airnode_state;
pub mod args;
pub mod web3sync;

fn main() {
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => {
            panic!("Args parsing error: {}", e);
        }
    };

    println!("done");
}
