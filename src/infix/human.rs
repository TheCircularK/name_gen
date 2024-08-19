use crate::ComponentSize;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn get_infix(size: &ComponentSize) -> &'static str {
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
    "al", "an", "ar", "el", "en", "in", "on", "ir", "or", "ur", "is", "un", "ul", "ek", "er", "os", "ak", "ir", "az", "ir", "ok", "am", "is", "ur", "en", "em", "ir", "az", "ur", "ar", "es",
    "ath", "ik", "im", "um", "ot", "od", "il", "ig", "un", "id", "az", "iz", "am", "ar", "os", "eth", "in", "ar", "il", "ir", "as", "er", "el", "ik", "is", "uk", "ut", "ag"
];

const MEDIUM: &[&str] = &[
    "ran", "mor", "kar", "tor", "tar", "lam", "dor", "rin", "lar", "hal", "mar", "til", "vor", "kil", "zak", "dar", "vil", "reth", "ron", "kal", "garn", "thul", "dros", "val", "rek", "mir", "nos", "nar", "vor", "san", "rak", "dos", "kel", "ven", "kar", "gil",
    "tav", "marn", "san", "vor", "rik", "nil", "gor", "tha", "vor", "dir", "mal", "gorn", "tor", "sim", "vin", "dar", "vol", "kir", "gon", "vil", "kam", "tan", "vik", "gal", "zor", "val", "lun", "tril", "kem", "mar", "vos"
];

const LONG: &[&str] = &[
    "ander", "thul", "rik", "med", "shar", "gor", "ther", "anth", "mir", "ven", "goth", "marn", "drak", "thor", "ven", "zorn", "tharn", "vak", "glor", "mor", "nir", "athor", "zanth", "kor", "nal", "sil", "kast", "vol", "mor", "ther", "gar", "karn", "dor", "mas", "kin", "tor", "garth", "dral", "gan", "til", "vor", "mal", "broth", "til", "shor", "dran", "gol", "farn", "koth", "galor", "nam", "kal", "ven"
];
