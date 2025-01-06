use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    registry
        .entry(section.to_string())
        .and_modify(|v| {
            v.push(animal.to_string());
            v.dedup();
        })
        .or_insert(vec![animal.to_string()]);
    return ();
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    let mut vec = registry.get(section).cloned().unwrap_or_default();
    vec.sort();

    vec
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut vec: Vec<String> = registry.iter().flat_map(|(_, v)| v).cloned().collect();
    vec.sort();

    vec
}
