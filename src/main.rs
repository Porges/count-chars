use std::{collections::BTreeMap, env, fs};
use unicode_categories::UnicodeCategories;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let files = env::args().nth(1).unwrap_or_else(|| "**/*.*".to_string());

    let mut map: BTreeMap<String, usize> = BTreeMap::new();

    for file in globwalk::glob(files).expect("unable to parse glob") {
        let entry = file.expect("unable to enumerate file");
        println!("Reading {}…", entry.path().display());
        let contents = fs::read_to_string(entry.path()).expect("couldn't read file");
        for g in contents.nfc().collect::<String>().graphemes(true) {
            match map.get_mut(g) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    map.insert(g.to_string(), 1);
                }
            }
        }
    }

    let mut vec = map.into_iter().collect::<Vec<(String, usize)>>();
    vec.sort_by(|x, y| y.1.cmp(&x.1));

    println!("{} distinct graphemes found", vec.len());

    for (g, count) in vec {
        println!("{}\t{count}", printable(g));
    }
}

fn printable(s: String) -> String {
    if s.len() == 1 {
        let char = s.chars().next().unwrap();
        if char.is_separator() || char.is_other() {
            return format!("U+{:04x}", char as u32);
        }
    }

    s
}
