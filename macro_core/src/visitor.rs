pub trait UiVisitor {
    fn get_path(&self) -> String;

    fn enter_enum(&mut self, variants: &[String]);
    fn enter_tuple_variant(&mut self, name: &str);
    fn leave_enum(&mut self);

    fn enter_option(&mut self);
    fn leave_option(&mut self);

    fn enter_struct(&mut self, in_tuple: bool);
    fn leave_struct(&mut self, in_tuple: bool);

    fn enter_child(&mut self, name: &str);
    fn leave_child(&mut self);

    fn add_integer(&mut self, name: &str);
    fn add_simple_enum(&mut self, variants: &[String]);
    fn add_unit_variant(&mut self, name: &str);
}

pub trait UI {
    /// Visit the data structure with a visitor.
    fn visit(visitor: &mut dyn UiVisitor, spaces: &str, in_tuple: bool);
}

pub fn visit_option<T: UI>(visitor: &mut dyn UiVisitor, spaces: &str) {
    visitor.enter_option();
    T::visit(visitor, spaces, false);
    visitor.leave_option();
}
