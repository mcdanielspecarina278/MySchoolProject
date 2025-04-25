use std::collections::HashSet;

fn main() {
    let words: HashSet<_> = ["hello", "world", "rust", "code"].iter().cloned().collect();

    for &word in words.iter() {
        println!("{}", word);
    }
}
