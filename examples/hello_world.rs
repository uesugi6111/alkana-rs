use alkana_rs::ALKANA;

fn main() {
    let hello = ALKANA.get_katakana("hello").unwrap();
    let world = ALKANA.get_katakana("world").unwrap();
    println!("{} {} !", hello, world);
}
