extern crate base64;
extern crate hex;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, CryptoError>;

#[derive(Debug)]
pub enum CryptoError {
    InvalidHexAscii(u8),
    OddHexStringLength(usize),
    MismatchedInputLengths,
    Utf8(std::str::Utf8Error),
    NoneError,
}

fn ascii_to_bin(i: u8) -> Result<u8> {
    match i {
        48..=57 => Ok(i - 48),
        97..=122 => Ok((i - 97) + 10),
        _ => Err(CryptoError::InvalidHexAscii(i)),
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

    let mut result: Vec<u8> = Vec::new();
    for i in 0..a_hex.len() {
        result.push(a_hex[i] ^ b_hex[i]);
    }

    Ok(bytes_to_hex(result))
}

pub fn single_xor(a: &str, k: u8) -> Result<String> {
    let a_hex = hex_to_bin(a)?;

    let mut result: Vec<u8> = Vec::new();
    for i in 0..a_hex.len() {
        result.push(a_hex[i] ^ k);
    }

    let st = std::str::from_utf8(&result).map_err(|e| CryptoError::Utf8(e))?;

    Ok(st.to_owned())
}

pub fn encrypt_repeating_xor(a: &str, key: &str) -> Result<String> {
    let mut result: Vec<u8> = Vec::new();
    let key_bytes = key.as_bytes();
    let mut key_iter = (0..key_bytes.len()).cycle();
    for b in a.as_bytes() {
        let idx = key_iter.next().ok_or(CryptoError::NoneError)?;
        result.push(b ^ key_bytes[idx]);
    }

    // let st = std::str::from_utf8(&result).map_err(|e| CryptoError::Utf8(e))?;

    Ok(bytes_to_hex(result))
}

pub fn hex_to_bin(input: &str) -> Result<Vec<u8>> {
    let mut output: Vec<u8> = Vec::new();

    if input.len() % 2 != 0 {
        return Err(CryptoError::OddHexStringLength(input.len()));
    }

    for pair in input.as_bytes().chunks(2) {
        let right = ascii_to_bin(pair[0])?;
        let left = ascii_to_bin(pair[1])?;
        output.push((right << 4) + left);
    }
    Ok(output)
}

pub fn hex_to_b64(input: &str) -> Result<String> {
    let bin = hex_to_bin(input)?;

    Ok(base64::encode(bin))
}

pub fn char_count(input: &str) -> HashMap<char, i16> {
    let mut counter = HashMap::new();

    for c in input.chars() {
        for lc in c.to_lowercase() {
            let count = counter.entry(lc).or_insert(0);
            *count += 1;
        }
    }

    counter
}

fn score_str(input: &str) -> i16 {
    let counter = char_count(input);
    let score = score_count(&counter);

    score
}

pub fn rank_strs(mut input: Vec<String>) -> Vec<(i16, String)> {
    let mut output = Vec::new();
    // for s in input.iter() {
    //     output.push((score_str(s), *s));
    // }

    while input.len() > 0 {
        let s = match input.pop() {
            Some(x) => x,
            None => break,
        };
        let score = score_str(&s);
        output.push((score, s));
    }

    // let mut output = input.iter().map(|s| (score_str(s), *s)).collect::<Vec<_>>();

    output.sort_by(|(s1, _), (s2, _)| s2.cmp(&s1));
    output
}

fn ranked_candidates<'a>(input: &'a str) -> Vec<(i16, String)> {
    let mut candidates: Vec<String> = Vec::new();
    for c in 0..255 {
        let cand = single_xor(input, c as u8);
        match cand {
            Ok(s) => candidates.push(s),
            Err(e) => (), //println!("error on {}: {:?}", c, e),
        }
    }

    let ranked = rank_strs(candidates);

    ranked
}

pub fn xor_top_n<'a>(input: &'a str, n: usize) -> Vec<(i16, String)> {
    // let mut top_n: Vec<(i16, String)> = Vec::new();

    let ranked = ranked_candidates(input);

    ranked[0..std::cmp::min(ranked.len(), n)].to_vec()
}

fn score_count(counter: &HashMap<char, i16>) -> i16 {
    let mut score = 0 as i16;

    for (c, c_count) in counter {
        let i_score = match c {
            'e' => 10, // was 12
            't' => 9,
            'a' => 8,
            'o' => 8,
            'i' => 7,
            'n' => 6,
            's' => 6,
            'r' => 6,
            'h' => 5,
            'd' => 4,
            'l' => 3,
            'u' => 2,
            'c' => 2,
            'm' => 2,
            'f' => 2,
            'y' => 2,
            'w' => 2,
            'g' => 2,
            'p' => 1,
            'b' => 1,
            'v' => 1,
            'k' => 1,
            'x' => 0,
            'q' => 0,
            'j' => 0,
            'z' => 0,
            ' ' => 2,
            // These are pretty arbitrary
            '&' => -2,
            '@' => -2,
            '*' => -3,
            '\n' => -3,
            _ => -1,
        };

        score += i_score * c_count;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_xor() {
        let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let expected =
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let output = encrypt_repeating_xor(text, key).unwrap();

        assert_eq!(expected, output);
    }

    #[test]
    fn test_rank_strs() {
        let input = vec!["etaoin".to_owned(), "zzzzz".to_owned()];

        let output = rank_strs(input);

        let (_, first) = &output[0];

        assert_eq!("etaoin", first);
    }

    #[test]
    fn test_score_str() {
        assert_eq!(score_str(&"e"), 10);

        assert_eq!(score_str(&"zz  z;;;$$"), -1);
    }

    #[test]
    fn test_char_count() {
        let count = char_count("aaaaa");
        assert_eq!(count.get(&'a'), Some(&(5 as i16)));

        let count2 = char_count("Aaaabb");
        assert_eq!(count2.get(&'a'), Some(&(4 as i16)));
        assert_eq!(count2.get(&'b'), Some(&(2 as i16)));
    }

    #[test]
    fn test_hex_to_bin() {
        assert_eq!(hex_to_bin("01").unwrap(), vec![1 as u8]);
        assert_eq!(hex_to_bin("0a").unwrap(), vec![10 as u8]);
        assert_eq!(hex_to_bin("ff").unwrap(), vec![255 as u8]);
        assert_eq!(hex_to_bin("f0").unwrap(), vec![240 as u8]);
        assert_eq!(hex_to_bin("0f").unwrap(), vec![15 as u8]);
        assert_eq!(hex_to_bin("09").unwrap(), vec![9 as u8]);

        assert_eq!(
            hex_to_bin("01010a0a").unwrap(),
            vec![1 as u8, 1 as u8, 10 as u8, 10 as u8]
        );
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
