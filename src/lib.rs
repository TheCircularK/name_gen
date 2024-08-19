pub mod prefix;
pub mod connector;
pub mod infix;
pub mod suffix;

pub enum NameRace {
    Human,
    RegalHuman,
    EvilHuman,
    Dwarven,
    Elven,
}

pub enum ComponentSize {
    Default,
    Short,
    Medium,
    Long,
    Random,
}

pub trait PartProvider {
    fn get_part(&self) -> &str;
}

pub fn generate_name() -> String { // parameter not used until human generator works
    let size: ComponentSize = ComponentSize::Default;

    let prefix = prefix::human::get_prefix(&size);
    let infix = infix::human::get_infix(&size);
    let suffix = suffix::human::get_suffix(&size);

    let name = format!("{}{}{}", prefix, infix, suffix);

    return name;
}

