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
    "mara", "nela", "fera", "asan", "isil", "erin", "syl", "nas", "sel", "ella", "fael", "lara",
    "fina", "vira", "thel", "mira", "lira", "sila", "vela", "linu", "fira", "selin", "vala", "nira",
    "thira", "elea", "sorin", "vani", "ralia", "nesa", "silin"
];

const FIRST_FEMALE: &[&str] = &[
    "mara", "nela", "fera", "asan", "isil", "erin", "syl", "nas", "sel", "ella", "fael", "al", "ari",
];

const LAST: &[&str] = &[
    "leaf", "silver", "moon", "silmar", "norith", "galas", "mirith", "thalion", "elanor", "caelith", "laenar", "islin", "elthar", "lothien", "aranel", "ethryn",
    "faelen", "silwen", "elorin", "thalass", "vaslir", "tirion",
];

const CONNECTORS: &[&str] = &[
    "l", "y", 
];