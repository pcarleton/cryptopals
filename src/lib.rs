extern crate base64;
extern crate hex;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, CryptoError>;


#[derive(Debug)]
pub enum CryptoError {
    InvalidHexAscii(u8),
    OddHexStringLength(usize),
    MismatchedInputLengths,
}

fn ascii_to_bin(i: u8) -> Result<u8> {
    match i {
        48..=57 => Ok(i - 48),
        97..=122 => Ok((i - 97) + 10),
        _ => Err(CryptoError::InvalidHexAscii(i))
    }
}


fn bytes_to_hex(input: Vec<u8>) -> String {
    hex::encode(input)
}

pub fn xor(a: &str, b: &str) -> Result<String> {
    let a_hex = hex_to_bin(a)?;
    let b_hex = hex_to_bin(b)?;

    if a_hex.len() != b_hex.len() {
        return Err(CryptoError::MismatchedInputLengths);
    }

    let mut result : Vec<u8> = Vec::new();
    for i in 0..a_hex.len() {
        result.push(a_hex[i] ^ b_hex[i]);
    }


    Ok(bytes_to_hex(result))
}

pub fn single_xor(a: &str, k: u8) -> Result<String> {
    let a_hex = hex_to_bin(a)?;

    let mut result : Vec<u8> = Vec::new();
    for i in 0..a_hex.len() {
        result.push(a_hex[i] ^ k);
    }

    let st = std::str::from_utf8(&result).map_err(|_| CryptoError::MismatchedInputLengths)?;

    Ok(st.to_string())
}

pub fn hex_to_bin(input: &str) -> Result<Vec<u8>> {
    let mut output : Vec<u8> = Vec::new();
    
    if input.len() % 2 != 0 {
        return Err(CryptoError::OddHexStringLength(input.len()))
    }

    for pair in input.as_bytes().chunks(2) {
        let right =  ascii_to_bin(pair[0])?;
        let left =  ascii_to_bin(pair[1])?;
        output.push((right << 4) + left);
    }
    Ok(output)
}

pub fn hex_to_b64(input: &str) -> Result<String> {

    let bin = hex_to_bin(input)?;

    Ok(base64::encode(bin))
}

pub fn char_count(input: &str) -> Result<HashMap<char, u16>> {

    let mut counter = HashMap::new();

    for c in input.chars() {
        for lc in c.to_lowercase() {
            let count = counter.entry(lc).or_insert(0);
            *count += 1;
        }
    }
    

    Ok(counter)
}


#[cfg(test)]
mod tests {
    use super::{hex_to_bin, hex_to_b64, bytes_to_hex, xor, char_count};

    #[test]
    fn test_char_count() {
        let count = char_count("aaaaa").unwrap();
        assert_eq!(count.get(&'a'), Some(&(5 as u16)));

        let count2 = char_count("Aaaabb").unwrap();
        assert_eq!(count2.get(&'a'), Some(&(4 as u16)));
        assert_eq!(count2.get(&'b'), Some(&(2 as u16)));
    }

    #[test]
    fn test_hex_to_bin() {
        assert_eq!(hex_to_bin("01").unwrap(), vec![1 as u8]);
        assert_eq!(hex_to_bin("0a").unwrap(), vec![10 as u8]);
        assert_eq!(hex_to_bin("ff").unwrap(), vec![255 as u8]);
        assert_eq!(hex_to_bin("f0").unwrap(), vec![240 as u8]);
        assert_eq!(hex_to_bin("0f").unwrap(), vec![15 as u8]);
        assert_eq!(hex_to_bin("09").unwrap(), vec![9 as u8]);


        assert_eq!(hex_to_bin("01010a0a").unwrap(), vec![1 as u8, 1 as u8, 10 as u8, 10 as u8]);

    }

    #[test]
    fn test_hex_to_b64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_b64(input).unwrap(), expected);
    }

    #[test]
    fn test_bytes_to_hex() {
        assert_eq!(bytes_to_hex(vec![9, 16, 255]), "0910ff".to_string())
    }

    #[test]
    fn test_xor() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";

        let expected = "746865206b696420646f6e277420706c6179";

        assert_eq!(xor(&a, &b).unwrap(), expected.to_string())
    }
}

