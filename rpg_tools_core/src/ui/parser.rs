pub trait UiParser {
    /// Parse a string from a path.
    fn get_str<'a>(&self, name: &str) -> &'a str;
}
