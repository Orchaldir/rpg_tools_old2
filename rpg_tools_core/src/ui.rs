pub trait UI {
    /// Create a viewer ui.
    fn create_viewer(&self, path: &str);
}
