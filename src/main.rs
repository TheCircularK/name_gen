use name_gen::{Race, Gender, NameLength};

fn main() {
    let race = Race::Human;
    let gender = Gender::Male;
    let size = NameLength::Short;
    let name = name_gen::generate_name(race, gender, size);
    println!("Human Male Short: {}", name);

    let race = Race::Dwarf;
    let gender = Gender::Female;
    let size = NameLength::Medium;
    let name = name_gen::generate_name(race, gender, size);
    println!("Dwarf Female Medium: {}", name);

    let race = Race::Elf;
    let gender = Gender::Female;
    let size = NameLength::Medium;
    let name = name_gen::generate_name(race, gender, size);
    println!("Elf Female Medium: {}", name);
}