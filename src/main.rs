use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];


    for c in 0..127 {
        let cand = cryptopals::single_xor(input, c as u8);
        match cand {
            Ok(s) => println!("{}", s),
            Err(_) => println!("error on {}", c)
        }
    }
}
