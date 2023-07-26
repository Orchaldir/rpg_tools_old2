pub trait UiVisitor {
    fn enter_struct(&mut self);
    fn leave_struct(&mut self);
}

pub struct ViewerVisitor {
    lines: Vec<String>,
    path: Vec<String>,
    spaces: String,
}

impl ViewerVisitor {
    pub fn new(path: String, spaces: String) -> Self {
        Self {
            lines: vec![],
            path: vec![path],
            spaces,
        }
    }

    pub fn get_lines(&self) -> &[String] {
        &self.lines
    }

    fn enter(&mut self) {
        self.spaces.push_str("  ");
    }

    fn leave(&mut self) {
        self.spaces.pop();
        self.spaces.pop();
    }
}

impl UiVisitor for ViewerVisitor {
    fn enter_struct(&mut self) {
        self.lines.push(format!("{}<ul>", self.spaces));
        self.enter();
    }

    fn leave_struct(&mut self) {
        self.leave();
        self.lines.push(format!("{}</ul>", self.spaces));
    }
}

pub trait UI {
    /// Create a viewer ui.
    fn create_viewer(&self, visitor: &mut dyn UiVisitor, path: &str, spaces: &str);
}
