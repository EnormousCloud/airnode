use web3::contract::{Contract, Options};
use web3::types::{H160, U256};
use tracing::warn;

#[derive(Debug)]
pub struct AirnodeInstance<T>
where
    T: web3::Transport,
{
    contract: Contract<T>,
}

impl<T: web3::Transport> AirnodeInstance<T> {
    pub fn new(web3: &web3::Web3<T>, address: H160) -> Self {
        let contract = Contract::from_json(
            web3.eth(),
            address,
            include_bytes!("./contracts/airnode.abi.json"),
        )
        .expect("fail contract::from_json(api3_convenience.abi.json)");
        AirnodeInstance { contract }
    }

    pub async fn get_templates(&self) -> Option<U256> {
        let value: U256 = match self.contract
        .query(
            "getTemplate",
            (U256::from_dec_str("51346484778186101048605968367225802300697788725479926120037132780973250912510").unwrap(), ),
            None,
            Options::default(),
            None,
        )
        .await {
            Ok(x) => x,
            Err(e) => {
                warn!("getTemplates {}", e);
                return None;
            }
        };
        Some(value)
    }

}