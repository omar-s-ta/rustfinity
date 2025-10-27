use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    if get_animals_in_section(section, registry).contains(&animal.to_string()) {
        return;
    }
    registry
        .entry(section.to_string())
        .or_default()
        .push(animal.to_string())
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    let mut values = registry.get(section).cloned().unwrap_or_default();
    values.sort();
    values
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut animals = registry
        .values()
        .flat_map(|value| value.clone())
        .collect::<Vec<_>>();

    animals.sort();
    animals
}
