use std::collections::HashMap;
pub(crate) fn generate_dictionary() -> HashMap<String, String> {
    let mut dictionary = HashMap::new();

    dictionary.insert("zyuganov".to_string(), "ジュガノフ".to_string());
    dictionary.insert("zygote".to_string(), "ザイゴウトゥ".to_string());
    dictionary.insert("zygomatic".to_string(), "ザイゴウマティク".to_string());
    dictionary.insert("zygoma".to_string(), "ザイゴウマ".to_string());
    dictionary.insert("zvegintzov".to_string(), "ズベジンツフ".to_string());

    dictionary
}
