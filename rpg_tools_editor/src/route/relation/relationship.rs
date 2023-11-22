use crate::route::relation::get_edit_relations_template;
use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::Template;
use rpg_tools_core::model::character::relation::relationship::Relationship;
use rpg_tools_core::model::character::CharacterId;
use rpg_tools_core::model::RpgData;
use rpg_tools_core::utils::storage::Id;

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
    get_edit_relations_template(
        &data,
        id,
        &data.relations.relationships,
        Relationship::get_all(),
    )
}
