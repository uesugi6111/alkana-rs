mod dictionary;
use once_cell::sync::Lazy;

use std::{collections::HashMap, sync::RwLock};

pub static ALKANA: Lazy<Alkana> = Lazy::new(Alkana::new);
#[derive(Default)]
pub struct Alkana {
    dictionary: RwLock<HashMap<String, String>>,
}
use dictionary::generate_dictionary;
impl Alkana {
    pub fn new() -> Self {
        let dictionary = RwLock::new(generate_dictionary());
        Alkana { dictionary }
    }
    pub fn get_katakana(&self, al: &str) -> Option<String> {
        let al = al.to_lowercase();
        if let Ok(d) = self.dictionary.read() {
            d.get(&al).map(|s| s.to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ALKANA;
    #[test]
    fn a() {
        assert_eq!(
            "ジュガノフ".to_string(),
            ALKANA.get_katakana("zyuganov").unwrap()
        );
    }
}
