use ethereum_types::{H160, U256};
use std::fmt;
use std::str::FromStr;

pub type DecodedMap = std::collections::HashMap<String, ABIParameter>;

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

#[derive(Debug, PartialEq, Clone)]
pub struct ABIParameters {
    pub version: u8,
    pub params: Vec<ABIParameter>,
}

impl ABIParameters {
    pub fn new(params: Vec<ABIParameter>) -> Self {
        Self { version: 1, params }
    }

    pub fn only(params: ABIParameter) -> Self {
        Self {
            version: 1,
            params: vec![params],
        }
    }

    pub fn encode(self) -> Vec<U256> {
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
    use super::{ABIParameter, ABIParameters};
    use ethereum_types::{H160, U256};
    use hex_literal::hex;
    use rand::{thread_rng, Rng};
    use std::convert::TryInto;

    fn into32(src: &[u8]) -> [u8; 32] {
        src.try_into().expect("slice with incorrect length")
    }

    fn rand_str() -> String {
        thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }

    fn rand_vec(sz: usize) -> Vec<u8> {
        thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(sz)
            .collect()
    }

    #[test]
    fn it_encodes_decodes_bytes() {
        let param = ABIParameter::Bytes {
            name: rand_str(),
            value: rand_vec(16),
        };
        let value = ABIParameters::only(param);
        let decoded = ABIParameters::decode(value.clone().encode()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_string() {
        let param = ABIParameter::String {
            name: rand_str(),
            value: rand_str(),
        };
        let value = ABIParameters::only(param);
        let decoded = ABIParameters::decode(value.clone().encode()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_bytes32() {
        let mut input: Vec<U256> = vec![];
        let n: u8 = ((rand::random::<f32>() * 10.0) as u8) / 10;
        for _ in 1..n {
            let r = rand_vec(32);
            input.push(U256::from(into32(&r)));
        }
        let param = ABIParameter::Bytes32 {
            name: rand_str(),
            value: input,
        };
        let value = ABIParameters::only(param);
        let decoded = ABIParameters::decode(value.clone().encode()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_address() {
        let r = rand_vec(32);
        let param = ABIParameter::Address {
            name: rand_str(),
            value: H160::from(H160::from_slice(&r)),
        };
        let value = ABIParameters::only(param);
        let decoded = ABIParameters::decode(value.clone().encode()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_uint256() {
        let r = rand_vec(32);
        let input = U256::from(into32(&r));
        let param = ABIParameter::Uint256 {
            name: rand_str(),
            value: input,
        };
        let value = ABIParameters::only(param);
        let decoded = ABIParameters::decode(value.clone().encode()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_int256_positive() {
        let mut r = rand_vec(32);
        r[0] &= 0b0111_1111; // Unset the first bit to get positive
        let input = U256::from(into32(&r));
        let param = ABIParameter::Uint256 {
            name: rand_str(),
            value: input,
        };
        let value = ABIParameters::only(param);
        let decoded = ABIParameters::decode(value.clone().encode()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_int256_negative() {
        let mut r = rand_vec(32);
        r[0] |= 0b1000_0000; // Set the first bit to get negative
        let input = U256::from(into32(&r));
        let param = ABIParameter::Uint256 {
            name: rand_str(),
            value: input,
        };
        let value = ABIParameters::only(param);
        let decoded = ABIParameters::decode(value.clone().encode()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_empty() {
        let value = ABIParameters::new(vec![]).encode();
        let expected: U256 =
            hex!("3100000000000000000000000000000000000000000000000000000000000000").into();
        assert_eq!(vec![expected], value);
    }
}
