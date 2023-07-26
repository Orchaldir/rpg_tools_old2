pub trait UiVisitor {
    fn enter_struct(&mut self);
    fn leave_struct(&mut self);
}

pub struct ViewerVisitor {
    path: Vec<String>,
    spaces: String,
}

impl ViewerVisitor {
    pub fn new(path: String, spaces: String) -> Self {
        Self {
            path: vec![path],
            spaces,
        }
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
        self.enter();
    }

    fn leave_struct(&mut self) {
        self.leave();
    }
}

pub trait UI {
    /// Create a viewer ui.
    fn create_viewer(&self, visitor: &mut dyn UiVisitor, path: &str, spaces: &str);
}
