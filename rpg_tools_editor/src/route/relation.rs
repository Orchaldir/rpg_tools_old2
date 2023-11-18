use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::{Character, CharacterId};
use rpg_tools_core::model::RpgData;
use rpg_tools_core::utils::storage::Element;
use rpg_tools_core::utils::storage::Id;

#[get("/relation/relationship/edit/<id>")]
pub fn edit_relationships(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| get_edit_template(&data, id, character))
}

#[get("/relation/relationship/delete/<from>/<to>")]
pub fn delete_relationship(data: &State<EditorData>, from: usize, to: usize) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Delete relationship from {} to {}", from, to);

    let from_id = CharacterId::new(from);
    let to_id = CharacterId::new(to);

    data.relations.relationships.delete(from_id, to_id);

    data.character_manager
        .get(from_id)
        .map(|character| get_edit_template(&data, from, character))
}

fn get_edit_template(data: &RpgData, id: usize, character: &Character) -> Template {
    Template::render(
        "generic/edit_relations",
        context! {
            title: "Relationships",
            link: "relationship",
            id: id,
            name: character.name(),
            relations: get_relationships(data, character),
        },
    )
}

pub fn get_relationships<'a>(
    data: &'a RpgData,
    character: &Character,
) -> Vec<(usize, &'a str, String)> {
    data.relations
        .relationships
        .get_all_of(character.id())
        .map(|relations| {
            relations
                .into_iter()
                .map(|(id, relation)| {
                    let name = data
                        .character_manager
                        .get(*id)
                        .map(|c| c.name())
                        .unwrap_or("Unknown");
                    (id.id(), name, relation.to_string())
                })
                .collect()
        })
        .unwrap_or_default()
}
