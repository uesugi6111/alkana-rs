use kana::*;

use proconio::{input, source::auto::AutoSource};
use std::fs::File;
use std::io::BufReader;
fn main() {
    let f = File::open("bep-eng.dic").unwrap();

    let b = AutoSource::new(BufReader::new(f));
    input!(from  b, dic:[(String, String, i32); 49289]);
    println!("use std::collections::HashMap;");
    println!("pub(crate) fn generate_dictionary() -> HashMap<String, String> {{");
    println!("    let mut dictionary = HashMap::new();");
    println!();

    for (al, kana, _) in dic.iter().take(5) {
        let a = al
            .chars()
            .map(|c| (c as u8 + 32) as char)
            .collect::<String>();
        println!(
            r#"    dictionary.insert("{}".to_string(), "{}".to_string());"#,
            a,
            half2kana(kana)
        );
    }
    println!(
        r#"
    dictionary
}}"#
    );
}
