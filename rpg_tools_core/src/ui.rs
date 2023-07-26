pub trait UiVisitor {
    fn enter_struct(&mut self);
    fn leave_struct(&mut self);
}

pub struct ViewerVisitor {
    spaces: String,
}

impl ViewerVisitor {
    pub fn new(spaces: String) -> Self {
        Self { spaces }
    }
}

impl UiVisitor for ViewerVisitor {
    fn enter_struct(&mut self) {
        self.spaces.push_str("  ");
    }

    fn leave_struct(&mut self) {
        self.spaces.pop();
        self.spaces.pop();
    }
}

pub trait UI {
    /// Create a viewer ui.
    fn create_viewer(&self, visitor: &mut dyn UiVisitor, path: &str, spaces: &str);
}
