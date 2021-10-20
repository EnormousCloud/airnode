use crate::args::Args;
use crate::logevent::LogEvent;
use std::str::FromStr;
use web3::types::{H160, U256};

#[derive(Debug, Clone, Default)]
pub struct LogFiltration {
    pub by_provider_id: Option<U256>,
    pub by_endpoint_id: Option<U256>,
    pub by_template_id: Option<U256>,
    pub by_request_id: Option<U256>,
    pub by_requester_index: Option<U256>,
    pub by_address: Option<H160>,
    pub by_airnode: Option<H160>,
}

impl LogFiltration {
    pub fn new(args: Args) -> anyhow::Result<Self> {
        let by_provider_id = args
            .by_provider_id
            .map(|x| U256::from_str(&x).expect("invalid BY_PROVIDER_ID"));
        let by_endpoint_id = args
            .by_endpoint_id
            .map(|x| U256::from_str(&x).expect("invalid BY_ENDPOINT_ID"));
        let by_template_id = args
            .by_template_id
            .map(|x| U256::from_str(&x).expect("invalid BY_TEMPLATE_ID"));
        let by_request_id = args
            .by_request_id
            .map(|x| U256::from_str(&x).expect("invalid BY_REQUEST_ID"));
        let by_requester_index = args.by_requester_index.map(|x| U256::from(x));
        let by_address = args
            .by_address
            .map(|x| H160::from_str(&x).expect("invalid BY_ADDRESS"));
        let by_airnode = args
            .by_airnode
            .map(|x| H160::from_str(&x).expect("invalid BY_AIRNODE"));

        Ok(Self {
            by_provider_id,
            by_endpoint_id,
            by_template_id,
            by_request_id,
            by_requester_index,
            by_address,
            by_airnode,
        })
    }

    pub fn allows(&self, le: &LogEvent) -> bool {
        let lee = match &le.event {
            Some(x) => x,
            None => return false,
        };
        if let Some(expected) = self.by_provider_id {
            match lee.get_provider_id() {
                Some(actual) => {
                    if expected != actual {
                        return false;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_endpoint_id {
            match lee.get_endpoint_id() {
                Some(actual) => {
                    if expected != actual {
                        return false;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_template_id {
            match lee.get_template_id() {
                Some(actual) => {
                    if expected != actual {
                        return false;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_request_id {
            match lee.get_request_id() {
                Some(actual) => {
                    if expected != actual {
                        return false;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_requester_index {
            match lee.get_requester_index() {
                Some(actual) => {
                    if expected != actual {
                        return false;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_airnode {
            match lee.get_airnode() {
                Some(actual) => {
                    if expected != actual {
                        return false;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_address {
            let mut found = false;
            for addr in lee.get_addresses() {
                if expected == addr {
                    found = true
                }
            }
            if !found {
                return false;
            }
        }
        true
    }
}
