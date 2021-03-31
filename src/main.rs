use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];


    //for 'a'..'z' {

    let output = cryptopals::hex_to_b64(input).unwrap();
    println!("{}", output);
}
