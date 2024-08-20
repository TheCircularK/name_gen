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
    "An", "Mal", "Tav", "Cot", "Das", "Kal", "K'r", "Sil", "Toc", "Fel", "Gano", "Sha'", "Kru", "Dro",
    "Nok", "Tor", "Ner", "Vor", "Ul", "Nar", "Bar", "Var", "Gil", "Sol", "Mar", "Tal", "Zan", "Bel", "Mor", "Kar", "Ren", "Tal", "Krul",
    "Ras", "Vel", "Sor", "Len", "Ven", "Arg", "Fal", "Kal", "Sur", "Nol", "Kul", "Dor", "Bral", "Ur", "Bor", "Thal", "Vex", "Zar",
];

const FIRST_FEMALE: &[&str] = &[
    "An", "Mal", "Tav", "Cot", "Das", "Kal", "K'r", "Sil", "Toc", "Fel", "Gano", "Sha'", "Kru", "Dro",
    "Nok", "Tor", "Ner", "Vor", "Ul", "Nar", "Bar", "Var", "Gil", "Sol", "Mar", "Tal", "Zan", "Bel", "Mor", "Kar", "Ren", "Tal", "Krul",
    "Ras", "Vel", "Sor", "Len", "Ven", "Arg", "Fal", "Kal", "Sur", "Nol", "Kul", "Dor", "Bral", "Ur", "Bor", "Thal", "Vex", "Zar",
];

const LAST: &[&str] = &[
    "An", "Mal", "Tav", "Cot", "Das", "Kal", "K'r", "Sil", "Toc", "Fel", "Gano", "Sha'", "Kru", "Dro",
    "Nok", "Tor", "Ner", "Vor", "Ul", "Nar", "Bar", "Var", "Gil", "Sol", "Mar", "Tal", "Zan", "Bel", "Mor", "Kar", "Ren", "Tal", "Krul",
    "Ras", "Vel", "Sor", "Len", "Ven", "Arg", "Fal", "Kal", "Sur", "Nol", "Kul", "Dor", "Bral", "Ur", "Bor", "Thal", "Vex", "Zar",
];

const CONNECTORS: &[&str] = &[
    "l", "y", "r"
];