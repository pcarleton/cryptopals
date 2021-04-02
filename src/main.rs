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

    let ranked = cryptopals::rank_strs(candidates);

    for i in 0..5 {
        let (score, s) = &ranked[i];
        println!("{} -- score: {}", s, score);
    }
}

use std::fs;
fn set_1_4(file_name: &str) {
    // input: ../inputs/1.4.txt
    // output: Now that the party is jumping
    let input = fs::read_to_string(file_name).unwrap();

    let mut all_cands: Vec<(usize, i16, String)> = Vec::new();
    let mut idx: usize = 0;
    // for line in input.split_whitespace() {
    //     // println!("{}", line);
    //     // TODO maybe use an Rc to avoid the clone here
    //     all_cands.push(
    //         cryptopals::xor_top_n(line, 5)
    //             .iter()
    //             .map(|(s, st)| (idx, *s, *st))
    //             .collect::<Vec<_>>(),
    //     );
    //     idx += 1;
    // }

    // let lines: Vec<&str> = input.split_whitespace().collect();
    // for i in 0..lines.len() {
    for line in input.split_whitespace() {
        // println!("{}", line);
        // TODO maybe use an Rc to avoid the clone here
        // let line = lines[i];
        let mut ranked = cryptopals::xor_top_n(line, 5);
        while let Some((score, st)) = ranked.pop() {
            all_cands.push((idx, score, st));
        }
        idx += 1;
    }

    // let mut flattened: Vec<(usize, i16, &String)> = all_cands.into_iter().flatten().collect();

    all_cands.sort_by(|(_, s1, _), (_, s2, _)| s2.cmp(&s1));

    for i in 0..5 {
        let (idx, score, s) = &all_cands[i];
        println!("{} -- score: {}, line: {}", s, score, idx);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];

    // set_1_3(input);
    set_1_4(input);
}
