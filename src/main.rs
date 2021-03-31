use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];

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
