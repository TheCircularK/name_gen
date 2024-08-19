use crate::ComponentSize;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn get_prefix(size: &ComponentSize) -> &'static str {
    let mut rng = thread_rng();

    match size {
        ComponentSize::Short => SHORT.choose(&mut rng),
        ComponentSize::Medium => MEDIUM.choose(&mut rng),
        ComponentSize::Long => LONG.choose(&mut rng),
        _ => SHORT.choose(&mut rng),
    }
    .unwrap_or(&"")
}

const SHORT: &[&str] = &[
    "An", "Mal", "Tav", "Cot", "Das", "Kal", "K'r", "Sil", "Toc", "Fel", "Gano", "Sha'", "Kru", "Dro",
    "Nok", "Tor", "Ner", "Vor", "Ul", "Nar", "Bar", "Var", "Gil", "Sol", "Mar", "Tal", "Zan", "Bel", "Mor", "Kar", "Ren", "Tal", "Krul",
    "Ras", "Vel", "Sor", "Len", "Ven", "Arg", "Fal", "Kal", "Sur", "Nol", "Kul", "Dor", "Bral", "Ur", "Bor", "Thal", "Vex", "Zar",
];

const MEDIUM: &[&str] = &[
    "Ando", "Dase", "Gral", "Rake", "Segule", "Gethol", "Urko", "Fener", "Enkar", "Hannan", "Shelt", "Sorrit", "Icarium",
    "Vandar", "Korbal", "Kruppe", "Dracon", "Silchas", "Taysch", "Tehol", "Caladan", "Rylland", "Ganoes", "Coltaine", "Duiker", "Mallick",
    "Anomand", "Kallor", "Vorcan", "Mael", "Uru", "Kallor", "Vatt", "Bidith", "T'rech", "Estin", "Mappo",
];

const LONG: &[&str] = &[
    "Destriant", "Ublala", "Laseen", "Pannion", "Kilmandaros", "Burn", "Kallor", "Osserc", "Telorast", "Gedorian", "Poliel", "Challice",
    "Aragan", "Korbolo", "Mathok", "Rath'Fener", "Kilmandaros", "Enverias", "Pallid", "Lysara", "Jhistal", "Proconsul", "Temul",
    "Gelorast", "Gothos", "Malazan", "Mokorath", "Borsilad", "Hannagora", "Vel'alar"
];