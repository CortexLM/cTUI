# Form Component

Form with fields and validation.

## Variants

### Basic Form

**Code:**

```rust
let form = Form::new()
    .field(FormField::new("name").label("Name"))
    .field(FormField::new("email").label("Email"))
    .field(FormField::new("message").label("Message"));
```

**Render:**

```
Name:    [________________]
Email:   [________________]
Message: [________________]

        [Submit] [Cancel]
```

### With Validation

**Code:**

```rust
let form = Form::new()
    .field(FormField::new("email")
        .label("Email")
        .required(true)
        .validator(|v| v.contains('@')))
    .on_submit(|values| {
        println!("{:?}", values);
    });
```

**Render:**

```
Email:   [test@example.com]

        [Submit]
```

### Field Types

```rust
pub enum FieldType {
    Text,
    Password,
    Email,
    Number,
    Textarea,
    Select(Vec<String>),
    Checkbox,
    Date,
}
```

## FormField

```rust
let field = FormField::new("username")
    .label("Username")
    .field_type(FieldType::Text)
    .required(true)
    .placeholder("Enter username")
    .validator(|v| v.len() >= 3)
    .error_message("Username must be 3+ characters");
```

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `fields` | `Vec<FormField>` | [] | Form fields |
| `submit_label` | `String` | "Submit" | Submit button label |
| `cancel_label` | `String` | "Cancel" | Cancel button label |
| `validate_on_change` | `bool` | true | Validate on input |
| `validate_on_submit` | `bool` | true | Validate on submit |

## FormField Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `name` | `String` | "" | Field identifier |
| `label` | `String` | "" | Field label |
| `field_type` | `FieldType` | Text | Input type |
| `required` | `bool` | false | Required field |
| `placeholder` | `String` | "" | Placeholder text |
| `value` | `String` | "" | Current value |
| `validator` | `Option<fn>` | None | Validation function |
| `error_message` | `String` | "" | Error text |

## Validation

```rust
// Built-in validation
.field(FormField::new("email")
    .field_type(FieldType::Email)
    .required(true))

// Custom validation
.field(FormField::new("password")
    .field_type(FieldType::Password)
    .validator(|v| v.len() >= 8)
    .error_message("Password must be 8+ characters"))

// Multiple validation rules
.field(FormField::new("age")
    .field_type(FieldType::Number)
    .min_value(0)
    .max_value(150))
```

## Example

```rust
use ctui_components::{Form, FormField, FieldType};

struct ContactForm {
    form: Form,
}

impl Component for ContactForm {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.form.render(area, buf);
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if let Some(values) = self.form.handle_submit(&msg) {
            // Process form values
            let name = values.get("name").unwrap();
            let email = values.get("email").unwrap();
            // ...
        }
        Cmd::Render
    }
}

fn create_form() -> Form {
    Form::new()
        .field(FormField::new("name")
            .label("Full Name")
            .required(true)
            .placeholder("John Doe"))
        .field(FormField::new("email")
            .label("Email Address")
            .field_type(FieldType::Email)
            .required(true))
        .field(FormField::new("message")
            .label("Message")
            .field_type(FieldType::Textarea)
            .required(true))
        .submit_label("Send Message")
}
```

## See Also

- [Input](input.md) - Text input
- [Checkbox](checkbox.md) - Checkbox input
- [Modal](modal.md) - Modal dialogs
