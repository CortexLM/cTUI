use ctui_macros::component;

// =============================================================================
// BASIC COMPONENT (existing tests preserved)
// =============================================================================

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

// =============================================================================
// EDGE CASE TESTS
// =============================================================================

/// Component for testing Debug/Clone derives on Props
#[component]
struct DebugTest {
    value: i32,
}

#[test]
fn test_props_has_debug_clone() {
    let props = DebugTestProps { value: 42 };
    let props_clone = props.clone();
    assert_eq!(props_clone.value, 42);
    let debug_str = format!("{:?}", props_clone);
    assert!(debug_str.contains("DebugTestProps"));
}

/// Component with multiple fields of various types
#[component]
struct MultiField {
    name: String,
    count: usize,
    active: bool,
}

#[test]
fn test_multiple_fields_type_correctness() {
    let props = MultiFieldProps {
        name: "test".to_string(),
        count: 100,
        active: true,
    };

    let component: MultiField = ctui_core::Component::create(props);
    assert_eq!(component.name, "test");
    assert_eq!(component.count, 100);
    assert!(component.active);
}

#[test]
fn test_state_is_unit() {
    fn assert_state_is_unit<T: ctui_core::Component<State = ()>>() {}
    assert_state_is_unit::<Button>();
}

/// Component with various integer types
#[component]
struct NumericFields {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i32,
}

#[test]
fn test_numeric_types_preserved() {
    let props = NumericFieldsProps {
        a: 255,
        b: 65000,
        c: 3000000000,
        d: 18000000000000000000,
        e: -42,
    };

    let comp: NumericFields = ctui_core::Component::create(props);
    assert_eq!(comp.a, 255);
    assert_eq!(comp.b, 65000);
    assert_eq!(comp.c, 3000000000);
    assert_eq!(comp.d, 18000000000000000000);
    assert_eq!(comp.e, -42);
}

#[test]
fn test_render_is_callable() {
    use ctui_core::{Buffer, Rect};

    let props = ButtonProps {
        label: "Render".to_string(),
        disabled: false,
    };
    let button: Button = ctui_core::Component::create(props);

    let area = Rect::new(0, 0, 10, 1);
    let mut buf = Buffer::empty(area);
    ctui_core::Component::render(&button, area, &mut buf);
}

#[test]
fn test_props_builder_pattern() {
    // Props should allow direct construction
    let props = ButtonProps {
        label: "Test".to_string(),
        disabled: false,
    };

    // Verify we can create component from props
    let _button: Button = ctui_core::Component::create(props);
}

/// Component with single field (edge case)
#[component]
struct SingleField {
    x: i32,
}

#[test]
fn test_single_field_component() {
    let props = SingleFieldProps { x: 123 };
    let comp: SingleField = ctui_core::Component::create(props);
    assert_eq!(comp.x, 123);
}

/// Component with longer name to test name formatting
#[component]
struct MyVeryLongComponentNameForTesting {
    value: String,
}

#[test]
fn test_long_component_name() {
    // Check that Props name follows pattern
    let props = MyVeryLongComponentNameForTestingProps {
        value: "test".to_string(),
    };
    let comp: MyVeryLongComponentNameForTesting = ctui_core::Component::create(props);
    assert_eq!(comp.value, "test");
}
