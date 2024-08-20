pub mod phonemes;
pub mod utils;
pub enum Race {
    Human,
    Dwarf,
    Elf,
}

pub enum NameLength {
    Short,
    Medium,
    Long,
}

pub enum NameType {
    FirstName,
    LastName,
}

pub enum Gender {
    Male,
    Female,
}

pub fn generate_name(race: Race, gender: Gender, size: NameLength) -> String {
    let name = match race {
        Race::Human => phonemes::human::get_name(&gender, &size),
        Race::Dwarf => phonemes::dwarf::get_name(&gender, &size),
        Race::Elf => phonemes::elf::get_name(&gender, &size),
    };

    name
}