pub trait UiVisitor {
    fn enter_struct(&mut self);
    fn leave_struct(&mut self);

    fn enter_child(&mut self, name: &str);
    fn leave_child(&mut self);

    fn add_integer(&mut self, name: &str);
    fn add_simple_enum(&mut self);
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

    fn get_name(&self) -> String {
        self.path.last().unwrap().clone()
    }

    pub fn get_path(&self) -> String {
        self.path.join(".")
    }

    fn enter(&mut self) {
        self.spaces.push_str("  ");
    }

    fn leave(&mut self) {
        self.spaces.pop();
        self.spaces.pop();
    }

    fn add_field(&mut self, name: &str, path: &str) {
        self.lines.push(format!(
            "{}<li><b>{}:</b> {{ {} }}</li>",
            self.spaces, name, path
        ));
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

    fn enter_child(&mut self, name: &str) {
        self.path.push(name.to_string());
    }

    fn leave_child(&mut self) {
        self.path.pop();
    }

    fn add_integer(&mut self, name: &str) {
        self.add_field(name, &format!("{}.{}", self.get_path(), name));
    }

    fn add_simple_enum(&mut self) {
        self.add_field(&self.get_name(), &self.get_path());
    }
}

pub trait UI {
    /// Create a viewer ui.
    fn create_viewer(&self, visitor: &mut dyn UiVisitor, path: &str, spaces: &str);
}
