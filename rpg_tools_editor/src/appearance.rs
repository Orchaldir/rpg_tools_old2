use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::length::Length;

#[derive(FromForm, Debug)]
pub struct AppearanceUpdate<'r> {
    body_type: &'r str,
    height: u32,
}

impl<'r> AppearanceUpdate<'r> {
    pub fn apply(&self, appearance: &Appearance) -> Appearance {
        return match appearance {
            Appearance::HeadOnly { head, .. } => {
                Appearance::head(*head, Length::from_millimetre(self.height))
            }
            Appearance::Humanoid { body, head, .. } => {
                Appearance::humanoid(*body, *head, Length::from_millimetre(self.height))
            }
        };
    }
}
