use serde::Serialize;
use std::str::FromStr;
use web3::types::H160;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Input<T> {
    pub s: String,
    pub value: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub msg: Option<String>,
}

impl Input<u64> {
    pub fn u64(value: u64) -> Self {
        Self {
            value,
            s: format!("{}", value),
            msg: None,
        }
    }
    pub fn parse_u64(&mut self, s: &str) -> bool {
        self.s = s.to_owned();
        match s.parse::<u64>() {
            Ok(x) => {
                self.value = x;
                self.msg = None;
            }
            Err(e) => {
                self.msg = Some(format!("{}", e));
            }
        }
        true
    }
}

impl Input<Option<u64>> {
    pub fn opt_u64() -> Self {
        Self {
            s: "".to_owned(),
            value: None,
            msg: None,
        }
    }
    pub fn parse_opt_u64(&mut self, s: &str) -> bool {
        self.s = s.to_owned();
        if s == "" || s == "0" {
            self.value = None;
            self.msg = None;
        } else {
            match s.parse::<u64>() {
                Ok(x) => {
                    self.value = Some(x);
                    self.msg = None;
                }
                Err(e) => {
                    self.msg = Some(format!("{}", e));
                }
            }
        }
        true
    }
}

impl Input<H160> {
    pub fn address(value: H160) -> Self {
        Self {
            s: format!("{:x}", value),
            value,
            msg: None,
        }
    }
    pub fn no_address() -> Self {
        Self {
            s: "".to_owned(),
            value: H160::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            msg: None,
        }
    }
    pub fn parse_address(&mut self, s: &str) -> bool {
        self.s = s.to_owned();
        let no0x = s.clone().replace("0x", "");
        if no0x == "" {
            self.msg = Some("Please provide address".to_owned());
        } else {
            match H160::from_str(&no0x) {
                Ok(x) => {
                    self.value = x.clone();
                    self.msg = None;
                    return true;
                }
                Err(e) => {
                    self.msg = Some(format!("{}", e));
                }
            }
        }
        true
    }
}

impl Input<String> {
    pub fn str(value: &str) -> Self {
        Self {
            value: value.to_string(),
            s: value.to_string(),
            msg: None,
        }
    }
    pub fn parse_url(&mut self, s: &str) -> bool {
        self.s = s.to_owned();
        match url::Url::parse(&s) {
            Ok(u) => {
                if u.scheme() == "http" || u.scheme() == "https" {
                    self.msg = None;
                    self.value = s.to_string();
                } else {
                    self.msg = Some("Invalid URL. Only http or https is allowed".to_owned());
                }
            }
            Err(e) => self.msg = Some(format!("{}", e)),
        }
        true
    }
}
