use kana::*;

use proconio::{input, marker::Chars, source::auto::AutoSource};
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let f = File::open("bep-eng.dic").unwrap();

    let n = 49289;
    let mut b = AutoSource::new(BufReader::new(f));
    input!(from  b, dic:[(String, String, i32); 49289]);
    for (al, kana, _) in dic.iter().take(5) {
        let a = al
            .chars()
            .map(|c| (c as u8 + 32) as char)
            .collect::<String>();
        println!("{} {}", a, half2kana(kana));
    }
}
