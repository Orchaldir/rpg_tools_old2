pub mod viewer;

pub trait UiVisitor {
    fn enter_enum(&mut self);
    fn enter_tuple_variant(&mut self, name: &str);
    fn leave_enum(&mut self);

    fn enter_struct(&mut self);
    fn leave_struct(&mut self);

    fn enter_child(&mut self, name: &str);
    fn leave_child(&mut self);

    fn add_integer(&mut self, name: &str);
    fn add_simple_enum(&mut self);
    fn add_unit_variant(&mut self, name: &str);
}

pub trait UI {
    /// Create a viewer ui.
    fn create_viewer(visitor: &mut dyn UiVisitor, path: &str, spaces: &str);
}
