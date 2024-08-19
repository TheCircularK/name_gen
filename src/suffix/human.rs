use crate::ComponentSize;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn get_suffix(size: &ComponentSize) -> &'static str {
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
    "or", "os", "on", "as", "in", "el", "ar", "an", "ek", "em", "im", "un", "eth", "ir", "ik", "al", "il", "ath", "or", "us", "en", "ol", "or", "in", "is", "um", "ir", "az", "ek", "is", "ul", "at", "ul", "az", "un", "ir", "en", "ir", "as",
    "ik", "ar", "is", "os", "an", "en", "ar", "or", "al", "el", "an", "eth", "ir", "in", "un", "az", "im", "ar", "us", "is", "um", "in", "ok", "el", "ol", "ik", "is", "un", "il", "os", "ir", "at", "ar", "an", "az", "is", "el", "ir", "as", "ul", "or"
];

const MEDIUM: &[&str] = &[
    "ander", "ion", "elus", "arak", "thar", "arin", "ath", "gar", "morn", "dris", "vor", "ras", "irad", "vak", "ilon", "kor", "arak", "til", "dor", "zan", "vor", "garth", "nor", "zarn", "ilad", "krin", "mas", "vern", "koth", "olam", "dar", "vak", "mor", "ath", "ver", "garn", "alon", "kam", "til", "vor", "ran", "zarn", "okar", "mar", "vor", "azan", "thel", "kan", "dar", "vorn", "zal", "kos", "garth", "dran", "mor", "sar"
];

const LONG: &[&str] = &[
    "athon", "morian", "theron", "daroth", "galen", "vanor", "hagan", "tharak", "arion", "norik", "dorath", "maron", "elion", "vandar", "arath", "valon", "garan", "dorin", "kathon", "morith", "vorian", "halon", "giloth", "virath", "kaleth", "vanor", "zelar", "dathor", "kaldor", "norath", "doreth", "varon", "tharath", "morith", "venor", "garan", "tilor", "maron", "kaldor", "valion", "galen", "norath", "thrion", "zareth", "khalor", "vornith", "darion", "vothan", "merith", "zathor", "keldor", "morion", "varkon"
];
