//! This library allows to encode and decode different types of data
//! during interaction between API3 Airnode and Ethereum smart contracts
//!
//! See details of protocol are at [Airnode Specification](https://github.com/api3dao/api3-docs/blob/master/airnode/airnode-abi-specifications.md)
//!
//! Parameters from contract event logs are consumed as `Vec<U256>`, which avoids reading 
//! random raw bytes and provides guarantee of a proper data alignment on input.
//!
//! ### decoding example
//! ```
//! use airnode_abi::ABI;
//! use ethereum_types::U256;
//!
//! fn main() {
//!     let data: Vec<U256> = vec![
//!         hex!("3162000000000000000000000000000000000000000000000000000000000000").into(),
//!         hex!("54657374427974657333324e616d650000000000000000000000000000000000").into(),
//!         hex!("536f6d6520627974657333322076616c75650000000000000000000000000000").into(),
//!     ];
//!     let res: ABI = ABI::decode(&data).unwrap();
//!     println!("{}", res);
//! }
//! ```
//!
//! ### encoding example
//! ```
//! use airnode_abi::{ABI, Param};
//! use ethereum_types::U256;
//!
//! fn main() {
//!     let param = Param::String {
//!         name: "hello".to_owned(),
//!         value: "world".to_owned(),
//!     };
//!     let res: Vec<U256> = ABI::new(vec![param]).encode().unwrap();
//!     println!("{}", res);
//! }
//! ```
//! Please see more examples for each type of the parameter in unit tests.

mod decode;
mod encode;

use decode::{chunk_to_address, chunk_to_int, chunk_to_str, chunk_to_vec};
use encode::{address_chunk, chunks, int_chunk, str_chunk32, str_chunks};
use ethereum_types::{H160, U256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;


#[derive(Debug, Error, Serialize)]
pub enum EncodingError {
    #[error("too many parameters, max is 31")]
    TooManyParams,
}

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum DecodingError {
    #[error("no input")]
    NoInput,
    #[error("schema is missing")]
    NoSchema,
    #[error("schema version is invalid")]
    InvalidVersion,
    #[error("invalid schema character {0}")]
    InvalidSchemaCharacter(char),
    #[error("invalid utf8 string")]
    InvalidUtf8String(String),
}

/// Atomic parameter in the Airnode ABI
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Param {
    /// parameter that embeds array of bytes (dynamic size)
    Bytes { name: String, value: Vec<u8> },
    /// parameter that embeds UTF-8 string (dynamic size)
    String { name: String, value: String },
    /// parameter that embeds EVM address (160 bits, H160)
    Address { name: String, value: H160 },
    /// parameters that embeds single 256 bits value
    Bytes32 { name: String, value: U256 },
    /// parameters that embeds signed 256 bits value (there is no type of I256 in Ethereum primitives)
    Int256 {
        name: String,
        value: U256,
        sign: i32, // we need to store the sign separately as we don't have that type
    },
    /// parameters that embeds unsigned 256 bits value
    Uint256 { name: String, value: U256 },
}

impl Param {
    /// returns name of the parameter
    pub fn get_name(&self) -> &str {
        match &self {
            Self::Bytes { name, value: _ } => name,
            Self::String { name, value: _ } => name,
            Self::Address { name, value: _ } => name,
            Self::Bytes32 { name, value: _ } => name,
            Self::Uint256 { name, value: _ } => name,
            Self::Int256 {
                name,
                value: _,
                sign: _,
            } => name,
        }
    }

    /// returns value of the parameter as string (for debugging purposes only)
    pub fn get_value(&self) -> String {
        match &self {
            Self::Bytes { name: _, value } => format!("{:x?}", value),
            Self::String { name: _, value } => value.clone(),
            Self::Address { name: _, value } => format!("{:?}", value),
            Self::Bytes32 { name: _, value } => format!("{:x?}", value),
            Self::Uint256 { name: _, value } => format!("{:x?}", value),
            Self::Int256 {
                name: _,
                value,
                sign,
            } => {
                if *sign >= 0 {
                    format!("{:x?}", value)
                } else {
                    format!("-{:x?}", value)
                }
            }
        }
    }

    /// returns character of the parameter for encoding
    /// - Upper case letters refer to dynamically sized types
    /// - Lower case letters refer to statically sized types
    pub fn get_char(&self) -> char {
        match &self {
            Self::Bytes { name: _, value: _ } => 'B',
            Self::String { name: _, value: _ } => 'S',
            Self::Address { name: _, value: _ } => 'a',
            Self::Bytes32 { name: _, value: _ } => 'b',
            Self::Uint256 { name: _, value: _ } => 'u',
            Self::Int256 {
                name: _,
                value: _,
                sign: _,
            } => 'i',
        }
    }

    /// returns whether the size of the parameter value is fixed
    pub fn is_fixed_size(&self) -> bool {
        match &self {
            Self::Bytes { name: _, value: _ } => false,
            Self::String { name: _, value: _ } => false,
            _ => true,
        }
    }

    /// returns encoded version of fixed size chunks
    pub fn fixed_chunks(&self) -> Vec<U256> {
        match &self {
            Self::Bytes { name, value: _ } => {
                // dynamic structure, second parameter is reserved to be overwritten later
                // it will contain the offset of the data
                vec![str_chunk32(name), U256::from(0)]
            }
            Self::String { name, value: _ } => {
                // dynamic structure, second parameter is reserved to be overwritten later
                // it will contain the offset of the data
                vec![str_chunk32(name), U256::from(0)]
            }
            Self::Address { name, value } => vec![str_chunk32(name), address_chunk(*value)],
            Self::Bytes32 { name, value } => vec![str_chunk32(name), value.clone()],
            Self::Uint256 { name, value } => vec![str_chunk32(name), value.clone()],
            Self::Int256 { name, value, sign } => vec![str_chunk32(name), int_chunk(*value, *sign)],
        }
    }

    /// returns encoded version of dynamic size chunks
    pub fn dynamic_chunks(&self) -> Vec<U256> {
        match &self {
            Self::Bytes { name: _, value } => vec![U256::from(value.len())]
                .into_iter()
                .chain(chunks(value).into_iter())
                .collect(),
            Self::String { name: _, value } => vec![U256::from(value.len())]
                .into_iter()
                .chain(str_chunks(value).into_iter())
                .collect(),
            _ => vec![],
        }
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({}={})",
            self.get_char(),
            self.get_name(),
            self.get_value()
        )
    }
}

/// Airnode ABI object that can be encoded into the vector of U256 and decoded from it
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ABI {
    /// Id of the ABI version. It is always "1" so far
    pub version: u8,
    /// Schema string. Each parameter is represented by a char
    pub schema: String,
    /// List of ABI parameters.
    pub params: Vec<Param>,
}

/// get parameters encoded into schema string.
/// Each parameter type will be represented by a char.
/// The first character, 1, represents the encoding version.
fn encode_schema(version: u8, params: &Vec<Param>) -> String {
    let s: String = params.iter().map(|p| p.get_char()).collect();
    format!("{}{}", version as char, s)
}

impl ABI {
    /// constructor of Airnode ABI from the list of parameters
    pub fn new(params: Vec<Param>) -> Self {
        Self {
            version: 0x31,
            schema: encode_schema(0x31, &params),
            params,
        }
    }

    /// constructor of Airnode ABI with no parameters
    pub fn none() -> Self {
        Self {
            version: 0x31,
            schema: "".to_owned(),
            params: vec![],
        }
    }

    /// constructor of Airnode ABI with a single parameter
    pub fn only(param: Param) -> Self {
        let params = vec![param];
        Self {
            version: 0x31,
            schema: encode_schema(0x31, &params),
            params: params,
        }
    }

    /// get parameter by its name
    pub fn get(&self, key: &str) -> Option<Param> {
        let filtered: Vec<&Param> = self
            .params
            .iter()
            .filter(|&x| x.get_name() == key)
            .collect();
        if filtered.len() > 0 {
            Some(filtered[0].clone())
        } else {
            None
        }
    }

    /// encodes ABI into vector or 256 bit values
    /// The function can encode up to 31 parameters (and 1 byte is used to encode the encoding version).
    pub fn encode(&self) -> Result<Vec<U256>, EncodingError> {
        if self.params.len() > 31 {
            return Err(EncodingError::TooManyParams);
        }
        let mut out = vec![str_chunk32(encode_schema(0x31, &self.params).as_str())];
        let mut m: HashMap<usize, usize> = HashMap::new();
        // first loop - pushing chunks of the fixed size
        self.params.iter().enumerate().for_each(|(i, p)| {
            if !p.is_fixed_size() {
                m.insert(i, out.len() + 1);
            }
            p.fixed_chunks().iter().for_each(|chunk| {
                out.push(chunk.clone());
            });
        });

        // second loop - pushing chunks of dynamic size and adjusting their offsets
        let mut offset: usize = out.len() * 0x20;
        self.params.iter().enumerate().for_each(|(i, p)| {
            let w_offset = m.get(&i);
            p.dynamic_chunks().iter().for_each(|chunk| {
                out[*w_offset.unwrap()] = U256::from(offset);
                out.push(chunk.clone());
            });
            offset = out.len() * 0x20;
        });
        Ok(out)
    }

    /// decodes ABI from the vector or 256 bit values. Data should not contain schema
    pub fn decode_with_schema(schema: String, data: &Vec<U256>) -> Result<Self, DecodingError> {
        let input: Vec<U256> = vec![str_chunk32(&schema)]
            .into_iter()
            .chain(data.clone().into_iter())
            .collect();
        Self::decode(&input)
    }

    /// decodes ABI from the vector or 256 bit values
    pub fn decode(input: &Vec<U256>) -> Result<Self, DecodingError> {
        if input.len() < 1 {
            return Err(DecodingError::NoInput);
        }
        let schema_chunk = input.get(0).unwrap();
        if schema_chunk.is_zero() {
            return Err(DecodingError::NoSchema);
        }

        let schema: String = chunk_to_str(*schema_chunk);
        let mut params: Vec<Param> = vec![];
        if schema.len() > 1 {
            let ch_version = schema.chars().nth(0).unwrap();
            if ch_version != '1' {
                return Err(DecodingError::InvalidVersion);
            }
            let mut offs: usize = 1;
            let mut errors: Vec<DecodingError> = vec![];
            schema.chars().skip(1).for_each(|ch| {
                match Self::from_chunks(ch, &input, &mut offs) {
                    Ok(p) => params.push(p),
                    Err(e) => errors.push(e),
                }
            });
            if errors.len() > 0 {
                return Err(errors[0].clone());
            }
        }
        Ok(Self::new(params))
    }

    /// decodes name and value from array of chunks, starting at the given `offset`
    /// and using type from `ch` character.
    /// Returns `Param` instance and updates `offset` with the bigger value.
    fn from_chunks(ch: char, arr: &Vec<U256>, offset: &mut usize) -> Result<Param, crate::DecodingError> {
        let name: String = chunk_to_str(arr[*offset]);
        *offset += 1;
        if ch == 'b' {
            let value = arr[*offset];
            *offset += 1;
            return Ok(Param::Bytes32 { name, value });
        } else if ch == 'u' {
            let value = arr[*offset];
            *offset += 1;
            return Ok(Param::Uint256 { name, value });
        } else if ch == 'a' {
            let value = chunk_to_address(arr[*offset]);
            *offset += 1;
            return Ok(Param::Address { name, value });
        } else if ch == 'i' {
            let (value, sign) = chunk_to_int(arr[*offset]);
            *offset += 1;
            return Ok(Param::Int256 { name, value, sign });
        } else if ch == 'B' || ch == 'S' {
            let value_index: usize = arr[*offset].as_usize(); // todo: handle failure
            *offset += 1;
            let data_offset = value_index / 32;
            let value_size: usize = arr[data_offset].as_usize(); // todo: handle failure
            let value = chunk_to_vec(arr, data_offset + 1, value_size);
            if ch == 'B' {
                return Ok(Param::Bytes { name, value });
            }
            let s = match String::from_utf8(value) {
                Ok(s) => s,
                Err(e) => return Err(DecodingError::InvalidUtf8String(format!("{}", e))),
            };
            return Ok(Param::String { name, value: s });
        }
        Err(DecodingError::InvalidSchemaCharacter(ch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use encode::into32;
    use ethereum_types::{H160, U256};
    use hex_literal::hex;
    use rand::{thread_rng, Rng};

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
        let param = Param::Bytes {
            name: rand_str(),
            value: rand_vec(16),
        };
        let value = ABI::only(param);
        let decoded = ABI::decode(&value.encode().unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_string() {
        let param = Param::String {
            name: rand_str(),
            value: rand_str(),
        };
        let value = ABI::only(param);
        let decoded = ABI::decode(&value.encode().unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_bytes32() {
        let r = rand_vec(32);
        let param = Param::Bytes32 {
            name: rand_str(),
            value: U256::from(into32(&r)),
        };
        let value = ABI::only(param);
        let decoded = ABI::decode(&value.encode().unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_address() {
        let r = rand_vec(20);
        let param = Param::Address {
            name: rand_str(),
            value: H160::from(H160::from_slice(&r)),
        };
        let value = ABI::only(param);
        let decoded = ABI::decode(&value.encode().unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_uint256() {
        let r = rand_vec(32);
        let input = U256::from(into32(&r));
        let param = Param::Uint256 {
            name: rand_str(),
            value: input,
        };
        let value = ABI::only(param);
        let decoded = ABI::decode(&value.encode().unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_int256_positive() {
        let mut r = rand_vec(32);
        r[0] &= 0b0111_1111; // Unset the first bit to get positive
        let input = U256::from(into32(&r));
        let param = Param::Uint256 {
            name: rand_str(),
            value: input,
        };
        let value = ABI::only(param);
        let decoded = ABI::decode(&value.encode().unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_int256_negative() {
        let mut r = rand_vec(32);
        r[0] |= 0b1000_0000; // Set the first bit to get negative
        let input = U256::from(into32(&r));
        let param = Param::Uint256 {
            name: rand_str(),
            value: input,
        };
        let value = ABI::only(param);
        let decoded = ABI::decode(&value.encode().unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_empty() {
        let value = ABI::none().encode().unwrap();
        let expected: U256 =
            hex!("3100000000000000000000000000000000000000000000000000000000000000").into();
        assert_eq!(value, vec![expected]);
    }

    #[test]
    fn it_decodes_empty() {
        let data: Vec<U256> =
            vec![hex!("3100000000000000000000000000000000000000000000000000000000000000").into()];
        let res = ABI::decode(&data).unwrap();
        assert_eq!(res, ABI::none());
    }

    #[test]
    fn it_decodes_address() {
        let data: Vec<U256> = vec![
            hex!("3161000000000000000000000000000000000000000000000000000000000000").into(),
            hex!("54657374416464726573734e616d650000000000000000000000000000000000").into(),
            hex!("0000000000000000000000004128922394c63a204dd98ea6fbd887780b78bb7d").into(),
        ];
        let res = ABI::decode(&data).unwrap();
        let expected = ABI::only(Param::Address {
            name: "TestAddressName".to_owned(),
            value: hex!("4128922394C63A204Dd98ea6fbd887780b78bb7d").into(),
        });
        assert_eq!(res, expected);
    }

    #[test]
    fn it_decodes_int256() {
        let data: Vec<U256> = vec![
            hex!("3169000000000000000000000000000000000000000000000000000000000000").into(),
            hex!("54657374496e744e616d65000000000000000000000000000000000000000000").into(),
            hex!("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc18").into(),
        ];
        let res = ABI::decode(&data).unwrap();
        let expected = ABI::only(Param::Int256 {
            name: "TestIntName".to_owned(),
            value: U256::from(1000),
            sign: -1,
        });
        assert_eq!(res, expected);
    }

    #[test]
    fn it_decodes_uint256() {
        let data: Vec<U256> = vec![
            hex!("3175000000000000000000000000000000000000000000000000000000000000").into(),
            hex!("5465737455496e744e616d650000000000000000000000000000000000000000").into(),
            hex!("00000000000000000000000000000000000000000000000000000000000007d0").into(),
        ];
        let res = ABI::decode(&data).unwrap();
        let expected = ABI::only(Param::Uint256 {
            name: "TestUIntName".to_owned(),
            value: U256::from(2000),
        });
        assert_eq!(res, expected);
    }

    #[test]
    fn it_decodes_bytes() {
        let data: Vec<U256> = vec![
            hex!("3142000000000000000000000000000000000000000000000000000000000000").into(),
            hex!("5465737442797465734e616d6500000000000000000000000000000000000000").into(),
            hex!("0000000000000000000000000000000000000000000000000000000000000060").into(),
            hex!("0000000000000000000000000000000000000000000000000000000000000003").into(),
            hex!("123abc0000000000000000000000000000000000000000000000000000000000").into(),
        ];
        let res = ABI::decode(&data).unwrap();
        let expected = ABI::only(Param::Bytes {
            name: "TestBytesName".to_owned(),
            value: vec![0x12, 0x3a, 0xbc],
        });
        assert_eq!(res, expected);
    }

    #[test]
    fn it_decodes_bytes32() {
        let data: Vec<U256> = vec![
            hex!("3162000000000000000000000000000000000000000000000000000000000000").into(),
            hex!("54657374427974657333324e616d650000000000000000000000000000000000").into(),
            hex!("536f6d6520627974657333322076616c75650000000000000000000000000000").into(),
        ];
        let res = ABI::decode(&data).unwrap();
        let expected = ABI::only(Param::Bytes32 {
            name: "TestBytes32Name".to_owned(),
            value: hex!("536f6d6520627974657333322076616c75650000000000000000000000000000").into(),
        });
        assert_eq!(res, expected);
    }

    #[test]
    fn it_decodes_string() {
        let data: Vec<U256> = vec![
            hex!("3153000000000000000000000000000000000000000000000000000000000000").into(),
            hex!("54657374537472696e674e616d65000000000000000000000000000000000000").into(),
            hex!("0000000000000000000000000000000000000000000000000000000000000060").into(),
            hex!("0000000000000000000000000000000000000000000000000000000000000011").into(),
            hex!("536f6d6520737472696e672076616c7565000000000000000000000000000000").into(),
        ];
        let res = ABI::decode(&data).unwrap();
        let expected = ABI::only(Param::String {
            name: "TestStringName".to_owned(),
            value: "Some string value".to_owned(),
        });
        assert_eq!(res, expected);
    }

    #[test]
    fn it_decodes_multiple() {
        let data: Vec<U256> = vec![
            hex!("3162615369427500000000000000000000000000000000000000000000000000").into(), //:00, 1baSiBu
            hex!("62797465733332206e616d650000000000000000000000000000000000000000").into(), //:20  "bytes name"
            hex!("62797465732033322076616c7565000000000000000000000000000000000000").into(), //:40  "bytes 32 value"
            hex!("77616c6c65740000000000000000000000000000000000000000000000000000").into(), //:60  "wallet"
            hex!("0000000000000000000000004128922394c63a204dd98ea6fbd887780b78bb7d").into(), //:80  0x4128922394C63A204Dd98ea6fbd887780b78bb7d
            hex!("737472696e67206e616d65000000000000000000000000000000000000000000").into(), //:a0  "string name"
            hex!("00000000000000000000000000000000000000000000000000000000000001a0").into(), //:c0  offs: 1a0
            hex!("62616c616e636500000000000000000000000000000000000000000000000000").into(), //:e0  "balance"
            hex!("ffffffffffffffffffffffffffffffffffffffffffffffff7538dcfb76180000").into(), //:10  -10000000000000000000
            hex!("6279746573206e616d6500000000000000000000000000000000000000000000").into(), //:12  "bytes name"
            hex!("00000000000000000000000000000000000000000000000000000000000001e0").into(), //:14  offs: 1e0
            hex!("686f6c6465727300000000000000000000000000000000000000000000000000").into(), //:16  "holders"
            hex!("000000000000000000000000000000000000000000000001158e460913d00000").into(), //:18  20000000000000000000
            hex!("000000000000000000000000000000000000000000000000000000000000000c").into(), //:1a  size: 12 bytes    [13]
            hex!("737472696e672076616c75650000000000000000000000000000000000000000").into(), //:1c  "string value"
            hex!("0000000000000000000000000000000000000000000000000000000000000003").into(), //:1e  size: 3 bytes     [15]
            hex!("123abc0000000000000000000000000000000000000000000000000000000000").into(), //:20  [0x12, 0x3a, 0xbc]
        ];
        let res = ABI::decode(&data).unwrap();
        let expected = ABI::new(vec![
            Param::Bytes32 {
                name: "bytes32 name".to_owned(),
                value: encode::str_chunk32("bytes 32 value"),
            },
            Param::Address {
                name: "wallet".to_owned(),
                value: hex!("4128922394C63A204Dd98ea6fbd887780b78bb7d").into(),
            },
            Param::String {
                name: "string name".to_owned(),
                value: "string value".to_owned(),
            },
            Param::Int256 {
                name: "balance".to_owned(),
                value: U256::from_dec_str("10000000000000000000").unwrap(),
                sign: -1,
            },
            Param::Bytes {
                name: "bytes name".to_owned(),
                value: hex!("123abc").into(),
            },
            Param::Uint256 {
                name: "holders".to_owned(),
                value: U256::from_dec_str("20000000000000000000").unwrap(),
            },
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    #[should_panic]
    fn it_shouldnt_decode_zero() {
        let data: Vec<U256> = vec![U256::from(0)];
        ABI::decode(&data).unwrap();
    }

    #[test]
    fn it_shouldnt_decode_invalid_version() {
        // checking every number except the valid version
        for i in 0..255 {
            let ch = format!("{:x}", i).chars().next().unwrap();
            if ch != '1' {
                break;
            }
            let mut b = rand_vec(32);
            b[0] = i;
            let data: Vec<U256> = vec![U256::from(into32(&b))];
            if let Ok(_) = ABI::decode(&data) {
                panic!("decodes data, started with 0x{:x} {:?}", i, data);
            }
        }
    }
}
