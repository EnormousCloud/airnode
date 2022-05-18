use ethereum_types::{BigEndianHash, H160, H256, U256, U512};
use std::str::{self, FromStr};

/// decode chunk into string (it is right padded with zeros)
pub fn chunk_to_str(src: U256) -> anyhow::Result<String> {
    let mut arr: Vec<u8> = vec![];
    let mut i = 31;
    loop {
        let b = src.byte(i);
        if b == 0u8 {
            break;
        }
        arr.push(b);
        if i == 0 {
            break;
        }
        i -= 1;
    }
    match str::from_utf8(&arr) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(anyhow::Error::msg(format!(
            "{} in chunk {:?}",
            e,
            H256::from_uint(&src)
        ))),
    }
}

/// attempt to find pattern of YYYY-MM-DD date in that string
pub fn str_to_date(src: &str) -> Option<(i32, u32, u32)> {
    if src.len() != 10 {
        return None;
    }
    let parts: Vec<&str> = src.split("-").collect();
    if parts.len() != 3 {
        return None;
    }
    if let Ok(year) = parts.get(0).unwrap().parse::<i32>() {
        if let Ok(month) = parts.get(1).unwrap().parse::<u32>() {
            if let Ok(day) = parts.get(2).unwrap().parse::<u32>() {
                // there is not much of date validation here
                if year > 0 && month > 0 && day > 0 {
                    return Some((year, month, day));
                }
            }
        }
    }
    None
}

/// decode chunk into signed integer
pub fn chunk_to_int(src: U256) -> (U256, i32) {
    if src.bit(32 * 8 - 1) {
        let a = U512::from_str("10000000000000000000000000000000000000000000000000000000000000000")
            .unwrap()
            - U256::from(src);
        let U512(ref arr) = a;
        let mut ret = [0; 4];
        ret[0] = arr[0];
        ret[1] = arr[1];
        ret[2] = arr[2];
        ret[3] = arr[3];
        return (U256(ret), -1);
    }
    (src, 1)
}

/// decode chunk into EVM address hash
pub fn chunk_to_address(src: U256) -> H160 {
    let mut s = String::from("");
    let mut i = 19;
    // compile a string of 20-byte hex value
    loop {
        let b = src.byte(i);
        s.push_str(format!("{:02x}", b).as_str());
        if i == 0 {
            break;
        }
        i -= 1;
    }
    H160::from_str(&s).unwrap()
}

/// decode chunk into vector of bytes
pub fn chunk_to_vec(arr: &Vec<U256>, offset: usize, sz: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(sz);
    let mut row_index = offset;
    let mut col_index = 0;
    let mut index = 0;
    // compile a string of 20-byte hex value
    loop {
        let b = arr[row_index].byte(31 - col_index);
        out.push(b);
        col_index += 1;
        if col_index % 32 == 0 {
            row_index += 1;
            col_index = 0;
        }
        index += 1;
        if index >= sz {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::U256;
    use hex_literal::hex;

    #[test]
    fn it_can_decode_ascii_str() {
        let expected = "hello";
        let res = chunk_to_str(
            U256::from(hex!(
                "68656c6c6f000000000000000000000000000000000000000000000000000000"
            ))
            .into(),
        )
        .unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn it_can_decode_utf8_str() {
        let expected = "你好世界";
        let res = chunk_to_str(
            U256::from(hex!(
                "E4BDA0E5A5BDE4B896E7958C0000000000000000000000000000000000000000"
            ))
            .into(),
        )
        .unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn it_can_decode_negative_int() {
        let (res, sign) = chunk_to_int(
            hex!("ffffffffffffffffffffffffffffffffffffffffffffffff7538dcfb76180000").into(),
        );
        assert_eq!(sign, -1);
        assert_eq!(res, U256::from_dec_str("10000000000000000000").unwrap());
    }

    #[test]
    fn it_can_decode_positive_int() {
        let (res, sign) = chunk_to_int(
            hex!("0000000000000000000000000000000000000000000000000000001234567890").into(),
        );
        assert_eq!(sign, 1);
        assert_eq!(res, U256::from_str("1234567890").unwrap());
    }

    #[test]
    fn it_can_decode_address() {
        let expected = hex!("4128922394C63A204Dd98ea6fbd887780b78bb7d").into();
        let res = chunk_to_address(
            hex!("0000000000000000000000004128922394c63a204dd98ea6fbd887780b78bb7d").into(),
        );
        assert_eq!(res, expected);
    }

    #[test]
    fn it_can_decode_short_vec() {
        let expected = vec![0x12, 0x3a, 0xbc];
        let arr =
            vec![hex!("123abc0000000000000000000000000000000000000000000000000000000000").into()];
        let res = chunk_to_vec(&arr, 0, 3);
        assert_eq!(res, expected);
    }
}
