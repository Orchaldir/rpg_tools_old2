use rpg_tools_core::ui::parser::UiParser;
use url_encoded_data::UrlEncodedData;

pub struct UrlParser<'a> {
    data: UrlEncodedData<'a>,
}

impl<'a> UrlParser<'a> {
    pub fn new(data: UrlEncodedData<'a>) -> Self {
        Self { data }
    }
}

impl<'a> UiParser<'a> for UrlParser<'a> {
    fn get_str(&self, name: &str) -> Option<&'a str> {
        self.data.get_first(name)
    }
}
