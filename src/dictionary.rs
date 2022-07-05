use std::collections::HashMap;
use std::include_str;
pub(crate) fn generate_dictionary() -> HashMap<String, String> {
    let csv = include_str!("../dictionary.csv");
    let mut dictionary = HashMap::new();

    for (al, kana) in csv.lines().map(|line| {
        let mut parts = line.split(',');
        let al = parts.next().unwrap();
        let kana = parts.next().unwrap();
        (al.to_owned(), kana.to_owned())
    }) {
        dictionary.insert(al, kana);
    }

    dictionary
}
