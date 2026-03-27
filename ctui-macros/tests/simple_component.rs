use ctui_macros::component;

#[component]
struct Button {
    label: String,
    disabled: bool,
}

#[test]
fn test_component_macro_generates_props() {
    let props = ButtonProps {
        label: "Click me".to_string(),
        disabled: false,
    };

    let button: Button = ctui_core::Component::create(props);
    assert_eq!(button.label, "Click me");
    assert!(!button.disabled);
}

#[test]
fn test_component_macro_generates_impl() {
    let props = ButtonProps {
        label: "Test".to_string(),
        disabled: true,
    };

    let mut button: Button = ctui_core::Component::create(props);

    let cmd = ctui_core::Component::update(&mut button, Box::new(()));
    assert_eq!(cmd, ctui_core::Cmd::Noop);
}
