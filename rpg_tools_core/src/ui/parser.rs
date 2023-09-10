pub trait UiParser {
    /// Parse a string from a path.
    fn get_str<'a>(&self, name: &str) -> Option<&'a str>;

    /// Parse an integer from a path.
    fn parse_u32(&self, path: &str) -> Option<u32> {
        self.get_str(path)
            .iter()
            .flat_map(|s| s.parse::<u32>().ok())
            .next()
    }
}