use ethereum_types::U256;
use std::convert::TryInto;

// convert into fixed array of 32 or panic
// Fix of the size must be at compile time! use carefully, this function panics.
pub fn into32(src: &[u8]) -> [u8; 32] {
    src.try_into().expect(format!("slice with incorrect length of {}", src.len()).as_str())
}

pub fn rpad32(src: &[u8]) -> Vec<u8> {
    let mut s = vec![];
    for ch in src.iter() {
        s.push(*ch);
    }
    while (s.len() % 32) != 0 {
        s.push(0u8);
    }
    s
}

pub fn lpad32(src: &[u8]) -> Vec<u8> {
    let mut s = vec![];
    while (s.len() + src.len()) % 32 != 0 {
        s.push(0u8);
    }
    for ch in src.iter() {
        s.push(*ch);
    }
    s
}

// convert string less than 31 characters long.
// string is padded with 0 to the right.
pub fn str_chunk32(src: &str) -> U256 {
    if src.len() > 31 {
        panic!("string length {} is over 31 bytes", src.len());
    }
    let out = rpad32(src.as_bytes());
    U256::from(into32(&out))
}

// convert string of unlimited length into array of 256 bits
// every string is padded with 0 to the right.
pub fn str_chunks(src: &str) -> Vec<U256> {
    let mut chunk: Vec<u8> = vec![];
    let mut out = vec![];
    for s in src.as_bytes() {
        chunk.push(*s);
        if chunk.len() == 32 {
            out.push(U256::from(into32(&chunk)));
            chunk = vec![];
        }
    }

    if chunk.len() > 0 {
        let v = rpad32(&chunk);
        out.push(U256::from(into32(&v)));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::U256;
    use hex_literal::hex;

    #[test]
    fn it_can_rpad32() {
        let padded = rpad32(&vec![ 0x31, 0x75 ]);
        assert_eq!(padded.len(), 32);
        let expected: U256 = hex!("3175000000000000000000000000000000000000000000000000000000000000").into();
        assert_eq!(U256::from(padded.as_slice()), expected);
    }

    #[test]
    fn it_can_lpad32() {
        let padded = lpad32(&vec![ 0x31, 0x75 ]);
        assert_eq!(padded.len(), 32);
        let expected: U256 = hex!("0000000000000000000000000000000000000000000000000000000000003175").into();
        assert_eq!(U256::from(padded.as_slice()), expected);
    }

    #[test]
    fn it_pads_hello() {
        let res = str_chunk32("hello");
        let expected = U256::from(hex!("68656c6c6f000000000000000000000000000000000000000000000000000000")).into();
        assert_eq!(res, expected);
    }

    #[test]
    #[should_panic]
    fn it_fails_on_chunk_over32() {
        str_chunk32("12345678901234567890123456789012345");
    }

    #[test]
    fn it_creates_long_string_chunks() {
        let res = str_chunks("1234567890123456789012345678901234567890");
        let expected = vec![
            hex!("3132333435363738393031323334353637383930313233343536373839303132").into(),
            hex!("3334353637383930000000000000000000000000000000000000000000000000").into(),
        ];
        assert_eq!(res, expected);
    }

}