use crate::route::relation::{get_edit_relations_template, RelationUpdate};
use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::Template;
use rpg_tools_core::model::character::relation::romantic::RomanticRelationship;
use rpg_tools_core::model::character::CharacterId;
use rpg_tools_core::model::RpgData;
use rpg_tools_core::utils::storage::Id;

#[get("/relation/romantic/edit/<id>")]
pub fn edit_romantic_relations(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");

    get_edit_template(&data, CharacterId::new(id))
}

#[get("/relation/romantic/delete/<from>/<to>")]
pub fn delete_romantic_relation(
    data: &State<EditorData>,
    from: usize,
    to: usize,
) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Delete romantic relationship between {} to {}", from, to);

    let from_id = CharacterId::new(from);
    let to_id = CharacterId::new(to);

    data.relations.romantic.delete(from_id, to_id);

    get_edit_template(&data, from_id)
}

#[post("/relation/romantic/update/<id>", data = "<update>")]
pub fn update_romantic_relation(
    data: &State<EditorData>,
    id: usize,
    update: Form<RelationUpdate<'_>>,
) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!(
        "Update romantic relationship between characters {} & {} to {}",
        id, update.character, update.relation,
    );

    let character_id = CharacterId::new(id);
    data.relations.romantic.add(
        character_id,
        CharacterId::new(update.character),
        update.relation.into(),
    );

    get_edit_template(&data, character_id)
}

fn get_edit_template(data: &RpgData, id: CharacterId) -> Option<Template> {
    get_edit_relations_template(
        data,
        id,
        &data.relations.romantic,
        "Romantic Relationships",
        "romantic",
        RomanticRelationship::get_all(),
    )
}
