use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::CharacterId;
use rpg_tools_core::model::RpgData;
use rpg_tools_core::utils::relation::RelationStorage;
use rpg_tools_core::utils::storage::Element;
use rpg_tools_core::utils::storage::Id;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod relationship;
pub mod romantic;

#[derive(FromForm, Debug)]
pub struct RelationUpdate<'r> {
    character: usize,
    relation: &'r str,
}

pub fn get_edit_relations_template<'a, T: Clone + Display + Serialize>(
    data: &RpgData,
    id: CharacterId,
    relations: &RelationStorage<CharacterId, T>,
    title: &str,
    link: &str,
    types: Vec<T>,
) -> Option<Template> {
    let character = data.character_manager.get(id)?;
    let characters: Vec<(usize, &str)> = data
        .character_manager
        .get_all()
        .iter()
        .filter(|c| c.id().ne(&id))
        .map(|c| (c.id().id(), c.name()))
        .collect();

    Some(Template::render(
        "generic/edit_relations",
        context! {
            title: title,
            link: link,
            id: id.id(),
            name: character.name(),
            relations: get_relations(data, relations, id),
            characters: characters,
            types: types,
        },
    ))
}

pub fn get_relations<'a, T: Clone + Display>(
    data: &'a RpgData,
    relations: &'a RelationStorage<CharacterId, T>,
    id: CharacterId,
) -> Vec<(usize, &'a str, String)> {
    relations
        .get_all_of(id)
        .map(|relations| {
            relations
                .iter()
                .map(|(other_id, relation)| {
                    let name = data
                        .character_manager
                        .get(*other_id)
                        .map(|c| c.name())
                        .unwrap_or("Unknown");
                    (other_id.id(), name, relation.to_string())
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn save_relations<T: Clone + Display + Serialize>(
    data: &RpgData,
    relations: &RelationStorage<CharacterId, T>,
    filename: &str,
) {
    if let Err(e) = relations.save(&data.get_path(filename)) {
        println!("Failed to save {}: {}", filename, e);
    }
}
