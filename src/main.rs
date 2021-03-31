use std::env;

fn set_1_3(input: &str) {
    // input: 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
    // output: Cooking MC's like a pound of bacon
    let mut candidates: Vec<String> = Vec::new();
    for c in 0..127 {
        let cand = cryptopals::single_xor(input, c as u8);
        match cand {
            Ok(s) => candidates.push(s),
            Err(_) => println!("error on {}", c),
        }
    }

    let ranked = cryptopals::rank_strs(&candidates);

    for i in 0..5 {
        let (score, s) = ranked[i];
        println!("{} -- score: {}", s, score);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];

    set_1_3(input);
}
