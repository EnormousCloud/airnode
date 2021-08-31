use ethereum_types::{H160, U256};
use std::fmt;
use std::str::FromStr;

pub type DecodedMap = std::collections::HashMap<String, String>;

#[derive(Debug, PartialEq, Clone)]
pub enum ABIParameter {
    Bytes {
        name: String,
        value: Vec<u8>,
    },
    String {
        name: String,
        value: String,
    },
    Address {
        name: String,
        value: H160,
    },
    Bytes32 {
        name: String,
        value: Vec<U256>,
    },
    Int256 {
        name: String,
        value: U256,
        sign: i32, // we need to store the sign separately as we don't have 
    },
    Uint256 {
        name: String,
        value: U256,
    },
}

impl ABIParameter {
    pub fn encode(parameters: Vec<Self>) -> Vec<U256> {
        let mut out = vec![];
        out
    }

    pub fn decode(input: Vec<U256>) -> Result<Self, ()> {
        Err(())
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum ABIParameterType {
    Bytes,
    String,
    Address,
    Bytes32,
    Int256,
    Uint256,
}


// Upper case letters refer to dynamically sized types
// Lower case letters refer to statically sized types
#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum ABIParameterTypeShort {
    B,
    S,
    a,
    b,
    i,
    u,
}

impl ABIParameterTypeShort {
    pub fn into(&self) -> ABIParameterType {
        match &self {
            ABIParameterTypeShort::B => ABIParameterType::Bytes,
            ABIParameterTypeShort::S => ABIParameterType::String,
            ABIParameterTypeShort::a => ABIParameterType::Address,
            ABIParameterTypeShort::b => ABIParameterType::Bytes32,
            ABIParameterTypeShort::i => ABIParameterType::Int256,
            ABIParameterTypeShort::u => ABIParameterType::Uint256,
        }
    }
}

impl ABIParameterType {
    pub fn into(&self) -> ABIParameterTypeShort {
        match &self {
            ABIParameterType::Bytes => ABIParameterTypeShort::B,
            ABIParameterType::String => ABIParameterTypeShort::S,
            ABIParameterType::Address => ABIParameterTypeShort::a,
            ABIParameterType::Bytes32 => ABIParameterTypeShort::b,
            ABIParameterType::Int256 => ABIParameterTypeShort::i,
            ABIParameterType::Uint256 => ABIParameterTypeShort::u,
        }
    }
}

impl fmt::Display for ABIParameterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}", self).to_lowercase())
    }
}

impl fmt::Display for ABIParameterTypeShort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}", self).to_lowercase())
    }
}

impl FromStr for ABIParameterType {
    type Err = ();
    fn from_str(input: &str) -> Result<ABIParameterType, Self::Err> {
        match input.to_lowercase().as_str() {
            "bytes" => Ok(ABIParameterType::Bytes),
            "string" => Ok(ABIParameterType::String),
            "address" => Ok(ABIParameterType::Address),
            "bytes32" => Ok(ABIParameterType::Bytes32),
            "int256" => Ok(ABIParameterType::Int256),
            "uint256" => Ok(ABIParameterType::Uint256),
            _ => Err(()),
        }
    }
}

impl FromStr for ABIParameterTypeShort {
    type Err = ();
    fn from_str(input: &str) -> Result<ABIParameterTypeShort, Self::Err> {
        match input.to_lowercase().as_str() {
            "B" => Ok(ABIParameterTypeShort::B),
            "S" => Ok(ABIParameterTypeShort::S),
            "a" => Ok(ABIParameterTypeShort::a),
            "b" => Ok(ABIParameterTypeShort::b),
            "i" => Ok(ABIParameterTypeShort::i),
            "u" => Ok(ABIParameterTypeShort::u),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
