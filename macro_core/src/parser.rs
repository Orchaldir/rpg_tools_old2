use std::collections::HashMap;

pub trait UiParser<'a> {
    /// Parse a string from a path.
    fn get_str(&'a self, name: &str) -> Option<&'a str>;

    /// Parse an bool from a path.
    fn parse_bool(&'a self, path: &str) -> bool {
        self.get_str(path)
            .iter()
            .flat_map(|s| s.parse::<bool>().ok())
            .next()
            .unwrap_or(false)
    }

    /// Parse an integer from a path.
    fn parse_u32(&'a self, path: &str, default: u32) -> u32 {
        self.get_str(path)
            .iter()
            .flat_map(|s| s.parse::<u32>().ok())
            .next()
            .unwrap_or(default)
    }
}

pub fn get_enum<'a, T: From<&'a str>>(parser: &'a dyn UiParser<'a>, path: &str) -> T {
    parser.get_str(path).unwrap_or("").into()
}

pub struct MockParser<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> MockParser<'a> {
    pub fn new(data: HashMap<&'a str, &'a str>) -> Self {
        Self { data }
    }
}

impl<'a> UiParser<'a> for MockParser<'a> {
    fn get_str(&self, name: &str) -> Option<&'a str> {
        self.data.get(name).copied()
    }
}
