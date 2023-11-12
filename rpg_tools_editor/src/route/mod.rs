use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::RpgData;
use rpg_tools_core::usecase::delete::DeleteResult;
use rpg_tools_core::utils::storage::{Entry, Id};
use std::sync::MutexGuard;

pub mod appearance;
pub mod character;
pub mod culture;
pub mod race;

pub fn get_failed_delete_template(
    data: MutexGuard<RpgData>,
    endpoint: &str,
    id: usize,
    name: &str,
    result: DeleteResult,
) -> Template {
    let characters = match result {
        DeleteResult::Blocked { characters } => characters
            .iter()
            .flat_map(|id| data.character_manager.get(*id))
            .map(|r| (r.id().id(), r.name()))
            .collect(),
        _ => Vec::new(),
    };

    Template::render(
        "generic/delete",
        context! {
            endpoint: endpoint,
            id: id,
            name: name,
            characters: characters,
        },
    )
}
