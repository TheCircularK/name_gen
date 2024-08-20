use rand::seq::SliceRandom;

use crate::NameLength;

pub fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &c.collect::<String>(),
    }
}

pub fn get_name_from_parts(phonemes: &[&str], length: &NameLength) -> String {
    let mut rng = rand::thread_rng();

    let num_phonemes = match length {
        NameLength::Short => 1,
        NameLength::Medium => 2,
        NameLength::Long => 3,
    };

    let mut name = String::new();

    for _ in 0..num_phonemes {
        let phoneme = phonemes.choose(&mut rng).unwrap_or(&"");
        name.push_str(phoneme);
    }

    let name = capitalize_first_letter(&name);

    name
}