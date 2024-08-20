use crate::utils::get_name_from_parts;
use crate::{NameLength, Gender};

pub fn get_name(gender: &Gender, length: &NameLength) -> String {
    let mut first_name = match gender {
        Gender::Male => get_name_from_parts(&FIRST_MALE, &length),
        Gender::Female => get_name_from_parts(&FIRST_FEMALE, &length),
    };

    let mut last_name = get_name_from_parts(&LAST, &length);

    format!("{} {}", first_name, last_name)
}

const FIRST_MALE: &[&str] = &[
    "kar", "tar", "gul", "gim", "dul", "tag", "ur", "az", "ab", "thor", "dain", "grun", "bar",
];

const FIRST_FEMALE: &[&str] = &[
    "kar", "tar", "gul", "gim", "dul", "tag", "ur", "az", "ab",
];

const LAST: &[&str] = &[
    "gul", "drum", "dor", "kar", "tar", "gul", "gim", "dul", "tag", "ur", "az", "ab",
];

const CONNECTORS: &[&str] = &[
    "k", "gk", "kr",
];