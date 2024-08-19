pub fn get_connector() -> &'static str {
    return HARD_CONNECTORS[1];
}

const HARD_CONNECTORS: &[&str] = &[
    "dr", "kr", "st", "gl", "th", "kh", "sk", "ch", "tr", "sh", "gr", "zr", "vr", "br", "pr", "vl", "kl", "str", "thr", "bl", "fr"
];

const SOFT_CONNECTORS: &[&str] = &[
    "l", "r", "n", "m", "v", "s", "y", "w"
];
