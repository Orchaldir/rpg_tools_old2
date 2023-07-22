/// The gender of the [`character`](crate::model::character::Character).
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum Gender {
    #[default]
    Female,
    Genderless,
    Male,
}
