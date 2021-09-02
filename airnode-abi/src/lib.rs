mod helpers;

#[allow(unused_variables)]
use ethereum_types::{H160, U256};
use std::fmt;
use serde::{Deserialize, Serialize};
use helpers::{str_chunks, str_chunk32};

/// All Airnode ABI parameters, represended as a map.
/// In fact, this is just an alias to `BTreeMap<String, Param>`
pub type DecodedMap = std::collections::BTreeMap<String, Param>;

/// Atomic parameter in the Airnode ABI
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Param {
    /// parameter that embeds array of bytes (dynamic size)
    Bytes { name: String, value: Vec<u8> },
    /// parameter that embeds UTF-8 string (dynamic size)
    String { name: String, value: String },
    /// parameter that embeds EVM address (160 bits, H160)
    Address { name: String, value: H160 },
    /// parameters that embeds array of 256 bits values
    Bytes32 { name: String, value: Vec<U256> },
    /// parameters that embeds signed 256 bits value (there is no type of I256 in Ethereum primitives)
    Int256 {
        name: String,
        value: U256,
        sign: i32, // we need to store the sign separately as we don't have
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

    /// returns encoded version of this parameter values as array of 256-bit chunks.
    pub fn into_chunks(&self) -> Vec<U256> {
        match &self {
            Self::Bytes { name, value } => {
                vec![str_chunk32(name)]
            },
            Self::String { name, value } => {
                vec![str_chunk32(name)].into_iter().chain(str_chunks(value).into_iter()).collect()
            },
            Self::Address { name, value } => vec![
                str_chunk32(name),
            ],
            Self::Bytes32 { name, value} => vec![
                str_chunk32(name),
            ],
            Self::Uint256 { name, value } => vec![
                str_chunk32(name),
            ],
            Self::Int256 { name, value, sign } => vec![
                str_chunk32(name),
            ],
        }
    }

    /// decodes name and value from array of chunks, starting at the given `offset` 
    /// and using type from `ch` character.
    /// Returns `Param` instance and updates `offset` with the bigger value.
    pub fn from_chunks(ch: char, arr: &Vec<U256>, offset: &mut usize) -> Result<Self, ()>  {
        let name_chunk = arr[*offset];
        *offset += 1;
        Ok(Self::Bytes { name: "todo".to_string(), value: vec![] })
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
    /// List of ABI parameters.
    pub params: Vec<Param>,
}

impl ABI {
    /// constructor of Airnode ABI from the list of parameters
    pub fn new(params: Vec<Param>) -> Self {
        Self { version: 0x31, params }
    }

    /// constructor of Airnode ABI with no parameters
    pub fn none() -> Self {
        Self {
            version: 1,
            params: vec![],
        }
    }

    /// constructor of Airnode ABI
    pub fn only(params: Param) -> Self {
        Self {
            version: 1,
            params: vec![params],
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

    /// turn all parameters into map by parameters name (for debugging and testing)
    pub fn into_map(&self) -> DecodedMap {
        let mut map = DecodedMap::new();
        for p in &self.params {
            map.insert(p.get_name().to_owned(), p.clone());
        }
        map
    }

    /// get parameters encoded into schema string. 
    /// Each parameter type will be represented by a char.
    // The first character, 1, represents the encoding version. 
    pub fn schema(&self) -> String {
        let s: String = self.params.iter().map(|p| p.get_char()).collect();
        format!("1{}", s)
    }

    /// get parameters encoded into schema as the first chunk in encoded 
    /// Each parameter type will be represented by a char.
    // The first character, 1, represents the encoding version. 
    pub fn schema_chunk(&self) -> U256 {
        U256::from(self.schema().as_bytes())
    }

    /// encodes ABI into vector or 256 bit values
    /// The function can encode up to 31 parameters (and 1 byte is used to encode the encoding version). 
    pub fn encode(self) -> Result<Vec<U256>, ()> {
        if self.params.len() > 31 {
            return Err(())
        }
        let mut out = vec![self.schema_chunk()];
        self.params.iter().for_each(|p| {
            p.into_chunks().iter().for_each(|chunk| {
                out.push(chunk.clone());
            });
        });
        Ok(out)
    }

    /// decodes ABI from the vector or 256 bit values
    pub fn decode(input: Vec<U256>) -> Result<Self, ()> {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::{H160, U256};
    use hex_literal::hex;
    use rand::{thread_rng, Rng};
    use helpers::into32;

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
        let decoded = ABI::decode(value.clone().encode().unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_string() {
        let param = Param::String {
            name: rand_str(),
            value: rand_str(),
        };
        let value = ABI::only(param);
        let decoded = ABI::decode(value.clone().encode().unwrap()).unwrap();
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
        let param = Param::Bytes32 {
            name: rand_str(),
            value: input,
        };
        let value = ABI::only(param);
        let decoded = ABI::decode(value.clone().encode().unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_decodes_address() {
        let r = rand_vec(32);
        let param = Param::Address {
            name: rand_str(),
            value: H160::from(H160::from_slice(&r)),
        };
        let value = ABI::only(param);
        let decoded = ABI::decode(value.clone().encode().unwrap()).unwrap();
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
        let decoded = ABI::decode(value.clone().encode().unwrap()).unwrap();
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
        let decoded = ABI::decode(value.clone().encode().unwrap()).unwrap();
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
        let decoded = ABI::decode(value.clone().encode().unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn it_encodes_empty() {
        let value = ABI::none().encode().unwrap();
        let expected: U256 =
            hex!("3100000000000000000000000000000000000000000000000000000000000000").into();
        assert_eq!(vec![expected], value);
    }

    #[test]
    fn it_decodes_empty() {
        let data: Vec<U256> =
            vec![hex!("3100000000000000000000000000000000000000000000000000000000000000").into()];
        let res = ABI::decode(data).unwrap();
        assert_eq!(res, ABI::none());
    }

    #[test]
    fn it_decodes_address() {
        let data: Vec<U256> = vec![
            hex!("3161000000000000000000000000000000000000000000000000000000000000").into(),
            hex!("54657374416464726573734e616d650000000000000000000000000000000000").into(),
            hex!("0000000000000000000000004128922394c63a204dd98ea6fbd887780b78bb7d").into(),
        ];
        let res = ABI::decode(data).unwrap();
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
        let res = ABI::decode(data).unwrap();
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
        let res = ABI::decode(data).unwrap();
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
        let res = ABI::decode(data).unwrap();
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
        let res = ABI::decode(data).unwrap();
        let expected = ABI::only(Param::Bytes32 {
            name: "TestBytes32Name".to_owned(),
            value: vec![
                hex!("536f6d6520627974657333322076616c75650000000000000000000000000000").into(),
            ],
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
        let res = ABI::decode(data).unwrap();
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
            hex!("62797465733332206e616d650000000000000000000000000000000000000000").into(), //:20
            hex!("62797465732033322076616c7565000000000000000000000000000000000000").into(), //:40
            hex!("77616c6c65740000000000000000000000000000000000000000000000000000").into(), //:60
            hex!("0000000000000000000000004128922394c63a204dd98ea6fbd887780b78bb7d").into(), //:80
            hex!("737472696e67206e616d65000000000000000000000000000000000000000000").into(), //:a0
            hex!("00000000000000000000000000000000000000000000000000000000000001a0").into(), //:c0
            hex!("62616c616e636500000000000000000000000000000000000000000000000000").into(), //:e0
            hex!("ffffffffffffffffffffffffffffffffffffffffffffffff7538dcfb76180000").into(), //:10
            hex!("6279746573206e616d6500000000000000000000000000000000000000000000").into(), //:12
            hex!("00000000000000000000000000000000000000000000000000000000000001e0").into(), //:14
            hex!("686f6c6465727300000000000000000000000000000000000000000000000000").into(), //:16
            hex!("000000000000000000000000000000000000000000000001158e460913d00000").into(), //:18
            hex!("000000000000000000000000000000000000000000000000000000000000000c").into(), //:1a
            hex!("737472696e672076616c75650000000000000000000000000000000000000000").into(), //:1c
            hex!("0000000000000000000000000000000000000000000000000000000000000003").into(), //:1e
            hex!("123abc0000000000000000000000000000000000000000000000000000000000").into(), //:20
        ];
        let res = ABI::decode(data).unwrap();
        let bytes32_val = "bytes 32 value".as_bytes();
        let expected = ABI::new(vec![
            Param::Bytes32{ name: "bytes32 name".to_owned(), value: vec![U256::from(into32(bytes32_val))] },
            Param::Bytes{ name: "bytes name".to_owned(), value: hex!("123abc").into() },
            Param::String{ name: "string name".to_owned(), value: "string value".to_owned() },
            Param::Int256{ name: "balance".to_owned(), value: U256::from_dec_str("10000000000000000000").unwrap(), sign: -1 },
            Param::Uint256{ name: "holders".to_owned(), value: U256::from_dec_str("20000000000000000000").unwrap() },
            Param::Address{ name: "wallet".to_owned(), value: hex!("4128922394C63A204Dd98ea6fbd887780b78bb7d").into()},
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    #[should_panic]
    fn it_shouldnt_decode_zero() {
        let data: Vec<U256> = vec![U256::from(0)];
        ABI::decode(data).unwrap();
    }

    #[test]
    fn it_shouldnt_decode_invalid_version() {
        // checking every number except the valid version
        for i in 0..255 {
            if i != 0x31 {
                let data: Vec<U256> = vec![U256::from(i)];
                if let Ok(_) = ABI::decode(data) {
                    panic!("decodes data, started with 0x{:x}", i);
                }
            }
        }
    }
}
