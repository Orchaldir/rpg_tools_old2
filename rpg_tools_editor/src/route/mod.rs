use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::RpgData;
use rpg_tools_core::usecase::delete::BlockingReason;
use rpg_tools_core::utils::storage::{Element, Id};
use std::sync::MutexGuard;

pub mod appearance;
pub mod character;
pub mod culture;
pub mod race;
pub mod relation;

pub fn get_failed_delete_template(
    data: MutexGuard<RpgData>,
    endpoint: &str,
    id: usize,
    name: &str,
    reason: BlockingReason,
) -> Template {
    let characters: Vec<(usize, &str)> = reason
        .characters
        .iter()
        .flat_map(|id| data.character_manager.get(*id))
        .map(|r| (r.id().id(), r.name()))
        .collect();

    Template::render(
        "generic/delete",
        context! {
            endpoint: endpoint,
            id: id,
            name: name,
            characters: characters,
            relations: reason.relations,
        },
    )
}
