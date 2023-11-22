use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::relation::relationship::Relationship;
use rpg_tools_core::model::character::CharacterId;
use rpg_tools_core::model::RpgData;
use rpg_tools_core::utils::relation::RelationStorage;
use rpg_tools_core::utils::storage::Element;
use rpg_tools_core::utils::storage::Id;
use std::fmt::Display;

#[get("/relation/relationship/edit/<id>")]
pub fn edit_relationships(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");

    get_edit_template(&data, CharacterId::new(id))
}

#[get("/relation/relationship/delete/<from>/<to>")]
pub fn delete_relationship(data: &State<EditorData>, from: usize, to: usize) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Delete relationship from {} to {}", from, to);

    let from_id = CharacterId::new(from);
    let to_id = CharacterId::new(to);

    data.relations.relationships.delete(from_id, to_id);

    get_edit_template(&data, from_id)
}

#[derive(FromForm, Debug)]
pub struct RelationshipUpdate<'r> {
    character: usize,
    relationship: &'r str,
}

#[post("/relation/relationship/update/<id>", data = "<update>")]
pub fn update_relationship(
    data: &State<EditorData>,
    id: usize,
    update: Form<RelationshipUpdate<'_>>,
) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!(
        "Update relationship {} for character {} to {}",
        update.relationship, id, update.character,
    );

    let character_id = CharacterId::new(id);
    data.relations.relationships.add(
        character_id,
        CharacterId::new(update.character),
        update.relationship.into(),
    );

    get_edit_template(&data, character_id)
}

fn get_edit_template(data: &RpgData, id: CharacterId) -> Option<Template> {
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
            title: "Relationships",
            link: "relationship",
            id: id.id(),
            name: character.name(),
            relations: get_relationships(data, &data.relations.relationships, id),
            characters: characters,
            types: Relationship::get_all(),
        },
    ))
}

pub fn get_relationships<'a, T: Clone + Display>(
    data: &'a RpgData,
    relations: &'a RelationStorage<CharacterId, T>,
    id: CharacterId,
) -> Vec<(usize, &'a str, String)> {
    relations
        .get_all_of(id)
        .map(|relations| {
            relations
                .into_iter()
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
