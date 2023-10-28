use macro_core::visitor::UiVisitor;
use titlecase::titlecase;

pub struct EditorVisitor {
    first_variant: bool,
    lines: Vec<String>,
    path: Vec<String>,
    spaces: String,
    in_tuple: bool,
}

impl EditorVisitor {
    pub fn new(path: String, spaces: String) -> Self {
        Self {
            first_variant: false,
            lines: vec![],
            path: vec![path],
            spaces,
            in_tuple: false,
        }
    }

    pub fn get_lines(&self) -> &[String] {
        &self.lines
    }

    fn get_pretty_name(&self) -> String {
        prettify(&self.get_name())
    }

    fn get_name(&self) -> String {
        self.path.last().unwrap().clone()
    }

    fn enter(&mut self) {
        self.spaces.push_str("  ");
    }

    fn leave(&mut self) {
        self.spaces.pop();
        self.spaces.pop();
    }

    fn enter_list(&mut self) {
        self.lines.push(format!("{}<ul>", self.spaces));
        self.enter();
    }

    fn leave_list(&mut self) {
        self.leave();
        self.lines.push(format!("{}</ul>", self.spaces));
    }

    fn add_selection(&mut self, path: &str, variants: &[String]) {
        self.add_named_selection(&self.get_pretty_name(), path, variants, path);
    }

    fn add_named_selection(&mut self, name: &str, path: &str, variants: &[String], selected: &str) {
        self.lines.push(format!(
            "{}<b>{}:</b> {{{{ macros::add_select(name=\"{}\", options=[ {} ], selected={}, update=true) }}}}",
            self.spaces,
            name,
            path,
            variants.iter().map(|v| format!("\"{}\"", v)).collect::<Vec<_>>().join(","),
            selected,
        ));
    }
}

impl UiVisitor for EditorVisitor {
    fn get_path(&self) -> String {
        self.path.join(".")
    }

    fn enter_enum(&mut self, variants: &[String]) {
        self.first_variant = true;
        self.add_selection(&format!("{}.type", self.get_path()), variants);
        self.enter_list();
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
        self.leave_list();
    }

    fn enter_option(&mut self) {
        self.lines
            .push(format!("{}{{% if {} %}}", self.spaces, self.get_path(),));
        self.add_named_selection(
            &format!("{} Availability", self.get_pretty_name()),
            &format!("{}.availability", self.get_path()),
            &["true".to_string(), "false".to_string()],
            "\"true\"",
        );
        let name = self.get_name();
        self.leave_child();
        self.enter_child(&name);
    }

    fn leave_option(&mut self) {
        self.lines.push(format!("{}{{% else %}}", self.spaces));
        self.add_named_selection(
            &format!("{} Availability", self.get_pretty_name()),
            &format!("{}.availability", self.get_path()),
            &["true".to_string(), "false".to_string()],
            "\"false\"",
        );
        self.lines.push(format!("{}{{% endif %}}", self.spaces));
    }

    fn enter_struct(&mut self, in_tuple: bool) {
        if in_tuple {
            self.lines.pop();
            self.leave();
            self.path.pop();
        } else {
            self.lines
                .push(format!("{}<b>{}</b>", self.spaces, self.get_pretty_name()));
            self.enter_list();
        }
    }

    fn leave_struct(&mut self, in_tuple: bool) {
        if !in_tuple {
            self.leave_list();
        }
        self.in_tuple = in_tuple;
    }

    fn enter_child(&mut self, name: &str) {
        self.lines.push(format!("{}<li>", self.spaces));
        self.enter();
        self.path.push(name.to_string());
    }

    fn leave_child(&mut self) {
        if self.in_tuple {
            self.in_tuple = false;
        } else {
            self.path.pop();
            self.leave();
            self.lines.push(format!("{}</li>", self.spaces));
        }
    }

    fn add_integer(&mut self, name: &str) {
        let path = format!("{}.{}", self.get_path(), name);
        self.lines.push(format!(
            "{0}<li><b>{1}:</b> <input type=\"number\" step=\"1\" id=\"{2}\" name=\"{2}\" value=\"{{{{ {2} }}}}\" onchange=\"updateAppearancePreview();\"></li>",
            self.spaces,
            prettify(name),
            path,
        ));
    }

    fn add_simple_enum(&mut self, variants: &[String]) {
        self.add_selection(&self.get_path(), variants);
    }

    fn add_unit_variant(&mut self, _name: &str) {}
}

fn prettify(text: &str) -> String {
    let with_spaces = text.replace('_', " ");
    titlecase(&with_spaces)
}
