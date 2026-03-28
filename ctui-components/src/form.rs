//! Form component for structured input handling.

use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

#[derive(Clone, Debug)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub value: String,
    pub field_type: FieldType,
    pub required: bool,
    pub error: Option<String>,
    pub style: Style,
    pub label_style: Style,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum FieldType {
    Text,
    Password,
    Number,
    Email,
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::Text
    }
}

impl Default for FormField {
    fn default() -> Self {
        Self {
            name: String::new(),
            label: String::new(),
            value: String::new(),
            field_type: FieldType::default(),
            required: false,
            error: None,
            style: Style::default(),
            label_style: Style::default(),
        }
    }
}

impl FormField {
    pub fn new(name: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            ..Self::default()
        }
    }

    pub fn field_type(mut self, field_type: FieldType) -> Self {
        self.field_type = field_type;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = style;
        self
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }

    pub fn validate(&mut self) -> bool {
        if self.required && self.value.trim().is_empty() {
            self.error = Some("This field is required".to_string());
            return false;
        }
        if self.field_type == FieldType::Email && !self.value.trim().is_empty() {
            let has_at = self.value.contains('@');
            let has_dot = self.value.contains('.');
            if !has_at || !has_dot {
                self.error = Some("Invalid email format".to_string());
                return false;
            }
        }
        if self.field_type == FieldType::Number && !self.value.trim().is_empty() {
            if self.value.parse::<f64>().is_err() {
                self.error = Some("Must be a valid number".to_string());
                return false;
            }
        }
        self.error = None;
        true
    }
}

#[derive(Clone, Debug)]
pub struct Form {
    fields: Vec<FormField>,
    focused_index: Option<usize>,
    submit_label: String,
    cancel_label: String,
    show_buttons: bool,
    style: Style,
    label_width: usize,
    field_spacing: u16,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            fields: Vec::new(),
            focused_index: None,
            submit_label: "Submit".to_string(),
            cancel_label: "Cancel".to_string(),
            show_buttons: true,
            style: Style::default(),
            label_width: 10,
            field_spacing: 1,
        }
    }
}

impl Form {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fields(mut self, fields: Vec<FormField>) -> Self {
        self.fields = fields;
        self
    }

    pub fn add_field(mut self, field: FormField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit_label = label.into();
        self
    }

    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn show_buttons(mut self, show: bool) -> Self {
        self.show_buttons = show;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn label_width(mut self, width: usize) -> Self {
        self.label_width = width;
        self
    }

    pub fn field_spacing(mut self, spacing: u16) -> Self {
        self.field_spacing = spacing;
        self
    }

    pub fn focus(&mut self, index: usize) {
        if index < self.fields.len() {
            self.focused_index = Some(index);
        }
    }

    pub fn focus_next(&mut self) {
        match self.focused_index {
            Some(idx) if idx + 1 < self.fields.len() => {
                self.focused_index = Some(idx + 1);
            }
            Some(_) => {
                self.focused_index = Some(0);
            }
            None => {
                self.focused_index = Some(0);
            }
        }
    }

    pub fn focus_prev(&mut self) {
        match self.focused_index {
            Some(0) => {
                self.focused_index = Some(self.fields.len().saturating_sub(1));
            }
            Some(idx) => {
                self.focused_index = Some(idx - 1);
            }
            None => {
                self.focused_index = Some(self.fields.len().saturating_sub(1));
            }
        }
    }

    pub fn focused(&self) -> Option<usize> {
        self.focused_index
    }

    pub fn fields_ref(&self) -> &[FormField] {
        &self.fields
    }

    pub fn fields_mut(&mut self) -> &mut [FormField] {
        &mut self.fields
    }

    pub fn get_value(&self, name: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|f| f.name == name)
            .map(|f| f.value.as_str())
    }

    pub fn set_value(&mut self, name: &str, value: impl Into<String>) {
        if let Some(field) = self.fields.iter_mut().find(|f| f.name == name) {
            field.value = value.into();
        }
    }

    pub fn validate(&mut self) -> bool {
        let mut all_valid = true;
        for field in &mut self.fields {
            if !field.validate() {
                all_valid = false;
            }
        }
        all_valid
    }

    pub fn get_values(&self) -> Vec<(String, String)> {
        self.fields
            .iter()
            .map(|f| (f.name.clone(), f.value.clone()))
            .collect()
    }

    pub fn clear_errors(&mut self) {
        for field in &mut self.fields {
            field.clear_error();
        }
    }

    fn render_field(
        &self,
        field: &FormField,
        y: u16,
        area: Rect,
        buf: &mut Buffer,
        is_focused: bool,
    ) {
        let label_text = if field.required {
            format!("{}*:", field.label)
        } else {
            format!("{}:", field.label)
        };

        for (i, ch) in label_text.chars().take(self.label_width).enumerate() {
            buf.modify_cell(area.x + i as u16, y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(field.label_style);
            });
        }

        let input_x = area.x + self.label_width as u16 + 1;
        let input_width = area.width.saturating_sub(self.label_width as u16 + 1);

        let bracket_style = if is_focused {
            Style::default()
        } else {
            self.style
        };

        buf.modify_cell(input_x, y, |cell| {
            cell.symbol = '['.to_string();
            cell.set_style(bracket_style);
        });

        let display_value = match field.field_type {
            FieldType::Password => "*".repeat(field.value.chars().count()),
            _ => field.value.clone(),
        };

        for (i, ch) in display_value
            .chars()
            .take(input_width.saturating_sub(2) as usize)
            .enumerate()
        {
            buf.modify_cell(input_x + 1 + i as u16, y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(field.style);
            });
        }

        buf.modify_cell(input_x + input_width.saturating_sub(1), y, |cell| {
            cell.symbol = ']'.to_string();
            cell.set_style(bracket_style);
        });

        if let Some(ref error) = field.error {
            if area.height > 1 {
                let error_y = y + 1;
                for (i, ch) in error.chars().take(area.width as usize).enumerate() {
                    buf.modify_cell(area.x + i as u16, error_y, |cell| { cell.symbol = ch.to_string(); });
                }
            }
        }
    }

    fn render_buttons(&self, area: Rect, buf: &mut Buffer) {
        let button_row = area.y + area.height.saturating_sub(1);
        let submit_text = format!("[ {} ]", self.submit_label);
        let cancel_text = format!("[ {} ]", self.cancel_label);

        for (i, ch) in submit_text.chars().enumerate() {
            buf.modify_cell(area.x + i as u16, button_row, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(self.style);
            });
        }

        let cancel_start = submit_text.len() + 2;
        for (i, ch) in cancel_text.chars().enumerate() {
            buf.modify_cell(area.x + cancel_start as u16 + i as u16, button_row, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(self.style);
            });
        }
    }
}

pub struct FormProps {
    pub fields: Vec<FormField>,
    pub submit_label: String,
    pub cancel_label: String,
    pub show_buttons: bool,
    pub style: Style,
    pub label_width: usize,
}

impl FormProps {
    pub fn new(fields: Vec<FormField>) -> Self {
        Self {
            fields,
            submit_label: "Submit".to_string(),
            cancel_label: "Cancel".to_string(),
            show_buttons: true,
            style: Style::default(),
            label_width: 10,
        }
    }

    pub fn submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit_label = label.into();
        self
    }

    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn show_buttons(mut self, show: bool) -> Self {
        self.show_buttons = show;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn label_width(mut self, width: usize) -> Self {
        self.label_width = width;
        self
    }
}

impl Component for Form {
    type Props = FormProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            fields: props.fields,
            focused_index: Some(0),
            submit_label: props.submit_label,
            cancel_label: props.cancel_label,
            show_buttons: props.show_buttons,
            style: props.style,
            label_width: props.label_width,
            field_spacing: 1,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() || self.fields.is_empty() {
            return;
        }

        let mut current_y = area.y;
        let fields_end = if self.show_buttons {
            area.height.saturating_sub(2)
        } else {
            area.height
        };

        for (i, field) in self.fields.iter().enumerate() {
            if current_y >= area.y + fields_end {
                break;
            }

            let is_focused = self.focused_index == Some(i);
            let field_height = if field.error.is_some() && current_y + 1 < area.y + fields_end {
                2
            } else {
                1
            };

            self.render_field(field, current_y, area, buf, is_focused);
            current_y += field_height + self.field_spacing;
        }

        if self.show_buttons {
            self.render_buttons(area, buf);
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctui_core::style::Color;
    use ctui_core::Buffer;
    use insta::assert_snapshot;

    fn render_to_string(form: &Form, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        form.render(Rect::new(0, 0, width, height), &mut buf);

        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = buf.get(x, y) { output.push_str(&cell.symbol); }
            }
            if y < height - 1 {
                output.push('\n');
            }
        }
        output
    }

    #[test]
    fn snapshot_form_basic() {
        let form = Form::new()
            .fields(vec![
                FormField::new("username", "Username"),
                FormField::new("password", "Password").field_type(FieldType::Password),
            ])
            .show_buttons(true);
        let result = render_to_string(&form, 30, 6);
        assert_snapshot!("form_basic", result);
    }

    #[test]
    fn snapshot_form_with_values() {
        let form = Form::new()
            .fields(vec![
                FormField::new("name", "Name").value("John Doe"),
                FormField::new("email", "Email").value("john@example.com"),
            ])
            .show_buttons(false);
        let result = render_to_string(&form, 35, 4);
        assert_snapshot!("form_with_values", result);
    }

    #[test]
    fn snapshot_form_with_errors() {
        let form = Form::new()
            .fields(vec![
                FormField::new("email", "Email").error("Invalid email format"),
                FormField::new("age", "Age"),
            ])
            .show_buttons(false);
        let result = render_to_string(&form, 40, 5);
        assert_snapshot!("form_with_errors", result);
    }

    #[test]
    fn test_form_field_new() {
        let field = FormField::new("test", "Test Field");
        assert_eq!(field.name, "test");
        assert_eq!(field.label, "Test Field");
        assert_eq!(field.value, "");
        assert!(!field.required);
    }

    #[test]
    fn test_form_field_required() {
        let mut field = FormField::new("test", "Test").required(true);
        assert!(field.required);
        assert!(!field.validate());
        assert!(field.error.is_some());

        field.set_value("value");
        assert!(field.validate());
        assert!(field.error.is_none());
    }

    #[test]
    fn test_form_field_email_validation() {
        let mut field = FormField::new("email", "Email")
            .field_type(FieldType::Email)
            .value("invalid");

        assert!(!field.validate());
        assert!(field.error.is_some());

        field.set_value("valid@email.com");
        assert!(field.validate());
        assert!(field.error.is_none());
    }

    #[test]
    fn test_form_field_number_validation() {
        let mut field = FormField::new("age", "Age")
            .field_type(FieldType::Number)
            .value("abc");

        assert!(!field.validate());
        assert!(field.error.is_some());

        field.set_value("42");
        assert!(field.validate());
        assert!(field.error.is_none());
    }

    #[test]
    fn test_form_field_password_display() {
        let field = FormField::new("pwd", "Password")
            .field_type(FieldType::Password)
            .value("secret");
        assert_eq!(field.value, "secret");
    }

    #[test]
    fn test_form_focus() {
        let mut form = Form::new().fields(vec![
            FormField::new("a", "A"),
            FormField::new("b", "B"),
            FormField::new("c", "C"),
        ]);

        form.focus(1);
        assert_eq!(form.focused(), Some(1));

        form.focus_next();
        assert_eq!(form.focused(), Some(2));

        form.focus_next();
        assert_eq!(form.focused(), Some(0));

        form.focus_prev();
        assert_eq!(form.focused(), Some(2));
    }

    #[test]
    fn test_form_get_set_value() {
        let mut form = Form::new().fields(vec![FormField::new("name", "Name").value("initial")]);

        assert_eq!(form.get_value("name"), Some("initial"));

        form.set_value("name", "updated");
        assert_eq!(form.get_value("name"), Some("updated"));

        assert_eq!(form.get_value("nonexistent"), None);
    }

    #[test]
    fn test_form_validate() {
        let mut form = Form::new().fields(vec![
            FormField::new("name", "Name").required(true),
            FormField::new("email", "Email").field_type(FieldType::Email),
        ]);

        assert!(!form.validate());

        form.set_value("name", "John");
        form.set_value("email", "invalid");
        assert!(!form.validate());

        form.set_value("email", "john@example.com");
        assert!(form.validate());
    }

    #[test]
    fn test_form_get_values() {
        let form = Form::new().fields(vec![
            FormField::new("a", "A").value("1"),
            FormField::new("b", "B").value("2"),
        ]);

        let values = form.get_values();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], ("a".to_string(), "1".to_string()));
        assert_eq!(values[1], ("b".to_string(), "2".to_string()));
    }

    #[test]
    fn test_form_props() {
        let props = FormProps::new(vec![FormField::new("test", "Test")])
            .submit_label("OK")
            .cancel_label("Abort")
            .label_width(15);

        let form = Form::create(props);
        assert_eq!(form.submit_label, "OK");
        assert_eq!(form.cancel_label, "Abort");
        assert_eq!(form.label_width, 15);
    }

    #[test]
    fn test_render_empty_form() {
        let form = Form::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 5));
        form.render(Rect::new(0, 0, 20, 5), &mut buf);
    }

    #[test]
    fn test_render_zero_area() {
        let form = Form::new().fields(vec![FormField::new("test", "Test")]);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        form.render(Rect::new(0, 0, 0, 0), &mut buf);
    }
}
