use crate::ui::UiVisitor;
use titlecase::titlecase;

pub struct ViewerVisitor {
    first_variant: bool,
    lines: Vec<String>,
    path: Vec<String>,
    spaces: String,
}

impl ViewerVisitor {
    pub fn new(path: String, spaces: String) -> Self {
        Self {
            first_variant: false,
            lines: vec![],
            path: vec![path],
            spaces,
        }
    }

    pub fn get_lines(&self) -> &[String] {
        &self.lines
    }

    fn get_name(&self) -> String {
        prettify(self.path.last().unwrap())
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
}

impl UiVisitor for ViewerVisitor {
    fn enter_enum(&mut self, _variants: &[String]) {
        self.first_variant = true;
        self.lines.push(format!(
            "{}<b>{}</b>: {{{{ {}.type }}}}",
            self.spaces,
            self.get_name(),
            self.get_path()
        ));
        self.lines.push(format!("{}<ul>", self.spaces));
        self.enter();
    }

    fn enter_tuple_variant(&mut self, name: &str) {
        if self.first_variant {
            self.lines.push(format!(
                "{}{{% if {}.type == \"{}\" %}}",
                self.spaces,
                self.get_path(),
                name
            ));
            self.first_variant = false;
        } else {
            self.leave();
            self.lines.push(format!(
                "{}{{% elif {}.type == \"{}\" %}}",
                self.spaces,
                self.get_path(),
                name
            ));
        }
        self.enter();
    }

    fn leave_enum(&mut self) {
        self.leave();
        self.lines.push(format!("{}{{% endif %}}", self.spaces));
        self.leave_struct();
    }

    fn enter_struct(&mut self) {
        self.lines
            .push(format!("{}<b>{}</b>", self.spaces, self.get_name()));
        self.lines.push(format!("{}<ul>", self.spaces));
        self.enter();
    }

    fn leave_struct(&mut self) {
        self.leave();
        self.lines.push(format!("{}</ul>", self.spaces));
    }

    fn enter_child(&mut self, name: &str) {
        self.lines.push(format!("{}<li>", self.spaces));
        self.enter();
        self.path.push(name.to_string());
    }

    fn leave_child(&mut self) {
        self.path.pop();
        self.leave();
        self.lines.push(format!("{}</li>", self.spaces));
    }

    fn add_integer(&mut self, name: &str) {
        self.lines.push(format!(
            "{}<li><b>{}:</b> {{{{ {}.{} }}}}</li>",
            self.spaces,
            prettify(name),
            self.get_path(),
            name
        ));
    }

    fn add_simple_enum(&mut self, _variants: &[String]) {
        self.lines.push(format!(
            "{}<b>{}:</b> {{{{ {} }}}}",
            self.spaces,
            self.get_name(),
            self.get_path()
        ));
    }

    fn add_unit_variant(&mut self, _name: &str) {}
}

fn prettify(text: &str) -> String {
    let with_spaces = text.replace('_', " ");
    titlecase(&with_spaces)
}
