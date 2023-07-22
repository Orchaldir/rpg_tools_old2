use rocket::form::Form;

#[derive(FromForm, Debug)]
pub struct AppearanceUpdate<'r> {
    body_type: &'r str,
    height: u32,
}
