use crate::logevent::LogEvent;
use web3::types::{H160, U256};

#[derive(Debug, Clone, Default)]
pub struct LogFiltration {
    pub extended: bool,
    pub by_provider_id: Option<U256>,
    pub by_endpoint_id: Option<U256>,
    pub by_template_id: Option<U256>,
    pub by_request_id: Option<U256>,
    pub by_requester_index: Option<U256>,
    pub by_address: Option<H160>,
    pub by_airnode: Option<H160>,
}

impl LogFiltration {
    pub fn allows(&self, le: &LogEvent) -> bool {
        if !self.extended {
            return true;
        }
        let lee = match le.event {
            Some(x) => x,
            None => return false,
        };
        if let Some(expected) = self.by_provider_id {
            match lee.get_provider_id() {
                Some(actual) => {
                    if expected != actual {
                        return true;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_endpoint_id {
            match lee.get_endpoint_id() {
                Some(actual) => {
                    if expected != actual {
                        return true;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_template_id {
            match lee.get_template_id() {
                Some(actual) => {
                    if expected != actual {
                        return true;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_request_id {
            match lee.get_request_id() {
                Some(actual) => {
                    if expected != actual {
                        return true;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_requester_index {
            match lee.get_requester_index() {
                Some(actual) => {
                    if expected != actual {
                        return true;
                    }
                }
                None => return false,
            }
        }
        if let Some(expected) = self.by_airnode {
            match lee.get_airnode() {
                Some(actual) => {
                    if expected != actual {
                        return true;
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
