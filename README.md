# alkana-rs

```:rust
use alkana_rs::ALKANA;

fn main() {
    let hello = ALKANA.get_katakana("hello").unwrap();
    let world = ALKANA.get_katakana("world").unwrap();
    println!("{} {} !", hello, world);
}

```

```:sh
ハロー ワールドゥ !
```

## Copyrights

Alphabetical word - katakana dictionary's data is from `bep-eng.dic`.

[Bilingual Emacspeak Project](http://www.argv.org/bep/)
(c) 1999-2002 Bilingual Emacspeak Project
