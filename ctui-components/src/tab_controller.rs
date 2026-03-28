pub mod messages {
    use ctui_core::{KeyCode, KeyModifiers, Msg};

    pub struct TabKeyEvent {
        pub key: KeyCode,
        pub modifiers: KeyModifiers,
    }
    impl Msg for TabKeyEvent {}

    pub struct TabSelected {
        pub index: usize,
        pub id: String,
    }
    impl Msg for TabSelected {}

    pub struct TabActivated {
        pub index: usize,
    }
    impl Msg for TabActivated {}

    pub struct TabDeactivated {
        pub index: usize,
    }
    impl Msg for TabDeactivated {}
}

use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabLifecycle {
    Initialized,
    Active,
    Inactive,
    Hidden,
}

pub trait TabContent: Component {
    fn lifecycle(&self) -> TabLifecycle {
        TabLifecycle::Initialized
    }

    fn on_show(&mut self) {}
    fn on_hide(&mut self) {}
    fn on_activate(&mut self) {}
    fn on_deactivate(&mut self) {}
}

pub struct TabEntry<C: Component> {
    id: String,
    label: String,
    component: C,
    lifecycle: TabLifecycle,
    style: Style,
}

impl<C: Component> TabEntry<C> {
    pub fn new(id: impl Into<String>, label: impl Into<String>, component: C) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            component,
            lifecycle: TabLifecycle::Initialized,
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn component(&self) -> &C {
        &self.component
    }

    pub fn component_mut(&mut self) -> &mut C {
        &mut self.component
    }

    pub fn lifecycle(&self) -> TabLifecycle {
        self.lifecycle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TabControllerConfig {
    pub wrap_around: bool,
    pub remember_state: bool,
    pub lazy_init: bool,
}

impl TabControllerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn wrap_around(mut self, wrap: bool) -> Self {
        self.wrap_around = wrap;
        self
    }

    pub fn remember_state(mut self, remember: bool) -> Self {
        self.remember_state = remember;
        self
    }

    pub fn lazy_init(mut self, lazy: bool) -> Self {
        self.lazy_init = lazy;
        self
    }
}

pub struct TabController<C: Component> {
    tabs: Vec<TabEntry<C>>,
    active_index: usize,
    tab_bar_style: Style,
    active_tab_style: Style,
    inactive_tab_style: Style,
    config: TabControllerConfig,
}

impl<C: Component> TabController<C> {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_index: 0,
            tab_bar_style: Style::default(),
            active_tab_style: Style::default(),
            inactive_tab_style: Style::default(),
            config: TabControllerConfig::default(),
        }
    }

    pub fn with_config(mut self, config: TabControllerConfig) -> Self {
        self.config = config;
        self
    }

    pub fn add_tab(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        component: C,
    ) -> Self {
        self.tabs.push(TabEntry::new(id, label, component));
        self
    }

    pub fn tab_bar_style(mut self, style: Style) -> Self {
        self.tab_bar_style = style;
        self
    }

    pub fn active_tab_style(mut self, style: Style) -> Self {
        self.active_tab_style = style;
        self
    }

    pub fn inactive_tab_style(mut self, style: Style) -> Self {
        self.inactive_tab_style = style;
        self
    }

    pub fn tabs(&self) -> &[TabEntry<C>] {
        &self.tabs
    }

    pub fn tabs_mut(&mut self) -> &mut [TabEntry<C>] {
        &mut self.tabs
    }

    pub fn active_index(&self) -> usize {
        self.active_index
    }

    pub fn active_tab(&self) -> Option<&TabEntry<C>> {
        self.tabs.get(self.active_index)
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut TabEntry<C>> {
        self.tabs.get_mut(self.active_index)
    }

    pub fn active_component(&self) -> Option<&C> {
        self.tabs.get(self.active_index).map(|t| &t.component)
    }

    pub fn active_component_mut(&mut self) -> Option<&mut C> {
        self.tabs
            .get_mut(self.active_index)
            .map(|t| &mut t.component)
    }

    pub fn select(&mut self, index: usize) -> bool {
        if index >= self.tabs.len() || self.tabs.is_empty() {
            return false;
        }

        if index == self.active_index {
            return true;
        }

        if let Some(old_tab) = self.tabs.get_mut(self.active_index) {
            old_tab.lifecycle = TabLifecycle::Inactive;
        }

        if let Some(new_tab) = self.tabs.get_mut(index) {
            new_tab.lifecycle = TabLifecycle::Active;
            self.active_index = index;
            true
        } else {
            false
        }
    }

    pub fn select_by_id(&mut self, id: &str) -> bool {
        if let Some(index) = self.tabs.iter().position(|t| t.id == id) {
            self.select(index)
        } else {
            false
        }
    }

    pub fn select_next(&mut self) -> bool {
        if self.tabs.is_empty() {
            return false;
        }

        let next = if self.config.wrap_around {
            (self.active_index + 1) % self.tabs.len()
        } else {
            (self.active_index + 1).min(self.tabs.len() - 1)
        };

        self.select(next)
    }

    pub fn select_prev(&mut self) -> bool {
        if self.tabs.is_empty() {
            return false;
        }

        let prev = if self.config.wrap_around {
            if self.active_index == 0 {
                self.tabs.len() - 1
            } else {
                self.active_index - 1
            }
        } else {
            self.active_index.saturating_sub(1)
        };

        self.select(prev)
    }

    pub fn select_first(&mut self) -> bool {
        self.select(0)
    }

    pub fn select_last(&mut self) -> bool {
        if self.tabs.is_empty() {
            false
        } else {
            self.select(self.tabs.len() - 1)
        }
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    pub fn remove_tab(&mut self, index: usize) -> Option<TabEntry<C>> {
        if index >= self.tabs.len() {
            return None;
        }

        let removed = self.tabs.remove(index);

        if self.active_index >= self.tabs.len() && !self.tabs.is_empty() {
            self.active_index = self.tabs.len() - 1;
        }

        Some(removed)
    }

    pub fn clear(&mut self) {
        self.tabs.clear();
        self.active_index = 0;
    }

    pub fn handle_key(
        &mut self,
        key: ctui_core::KeyCode,
        modifiers: ctui_core::KeyModifiers,
    ) -> Cmd {
        use ctui_core::KeyCode;

        if modifiers.shift {
            match key {
                KeyCode::Tab => {
                    self.select_prev();
                    return Cmd::Render;
                }
                _ => {}
            }
        } else {
            match key {
                KeyCode::Tab => {
                    self.select_next();
                    return Cmd::Render;
                }
                KeyCode::Char('[') => {
                    self.select_prev();
                    return Cmd::Render;
                }
                KeyCode::Char(']') => {
                    self.select_next();
                    return Cmd::Render;
                }
                _ => {}
            }
        }

        if let Some(tab) = self.tabs.get_mut(self.active_index) {
            tab.component
                .update(Box::new(messages::TabKeyEvent { key, modifiers }))
        } else {
            Cmd::Noop
        }
    }

    pub fn init_all(&mut self) {
        for tab in &mut self.tabs {
            tab.lifecycle = TabLifecycle::Initialized;
        }
        if let Some(first_tab) = self.tabs.first_mut() {
            first_tab.lifecycle = TabLifecycle::Active;
        }
    }

    pub fn render_tab_bar(&self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || self.tabs.is_empty() {
            return;
        }

        let mut current_x = area.x;

        for (i, tab) in self.tabs.iter().enumerate() {
            let is_active = i == self.active_index;
            let style = if is_active {
                self.active_tab_style
            } else {
                self.inactive_tab_style
            };

            let prefix = " ";
            let suffix = " ";

            if current_x < area.x + area.width {
                buf.modify_cell(current_x, area.y, |cell| {
                    cell.symbol = prefix.chars().next().unwrap_or(' ').to_string();
                    cell.set_style(style);
                });
            }

            let mut char_pos = 0;
            for ch in tab.label.chars() {
                if current_x + 1 + char_pos >= area.x + area.width {
                    break;
                }
                buf.modify_cell(current_x + 1 + char_pos, area.y, |cell| {
                    cell.symbol = ch.to_string();
                    cell.set_style(style);
                });
                char_pos += 1;
            }

            let label_len = tab.label.chars().count() as u16;
            if current_x + 1 + label_len < area.x + area.width {
                buf.modify_cell(current_x + 1 + label_len, area.y, |cell| {
                    cell.symbol = suffix.chars().next().unwrap_or(' ').to_string();
                    cell.set_style(style);
                });
            }

            current_x += label_len + 2;
        }
    }

    pub fn render_content(&mut self, area: Rect, buf: &mut Buffer) {
        if let Some(tab) = self.tabs.get_mut(self.active_index) {
            tab.component.render(area, buf);
        }
    }
}

impl<C: Component> Default for TabController<C> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctui_core::Buffer;

    struct MockComponent {
        value: i32,
    }

    struct MockProps;

    impl Component for MockComponent {
        type Props = MockProps;
        type State = ();

        fn create(_props: Self::Props) -> Self {
            Self { value: 0 }
        }

        fn render(&self, area: Rect, buf: &mut Buffer) {
            buf.modify_cell(area.x, area.y, |cell| { cell.symbol = self.value.to_string(); });
        }

        fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
            Cmd::Noop
        }
    }

    #[test]
    fn test_tab_controller_new() {
        let controller: TabController<MockComponent> = TabController::new();
        assert!(controller.is_empty());
        assert_eq!(controller.len(), 0);
    }

    #[test]
    fn test_tab_controller_add_tabs() {
        let controller = TabController::new()
            .add_tab("tab1", "First", MockComponent::create(MockProps))
            .add_tab("tab2", "Second", MockComponent::create(MockProps));

        assert_eq!(controller.len(), 2);
        assert!(!controller.is_empty());
        assert_eq!(controller.active_index(), 0);
    }

    #[test]
    fn test_tab_controller_select() {
        let mut controller = TabController::new()
            .add_tab("tab1", "First", MockComponent::create(MockProps))
            .add_tab("tab2", "Second", MockComponent::create(MockProps))
            .add_tab("tab3", "Third", MockComponent::create(MockProps));

        assert!(controller.select(2));
        assert_eq!(controller.active_index(), 2);

        assert!(controller.select(0));
        assert_eq!(controller.active_index(), 0);

        assert!(!controller.select(10));
        assert_eq!(controller.active_index(), 0);
    }

    #[test]
    fn test_tab_controller_select_by_id() {
        let mut controller = TabController::new()
            .add_tab("tab1", "First", MockComponent::create(MockProps))
            .add_tab("tab2", "Second", MockComponent::create(MockProps));

        assert!(controller.select_by_id("tab2"));
        assert_eq!(controller.active_index(), 1);

        assert!(!controller.select_by_id("nonexistent"));
        assert_eq!(controller.active_index(), 1);
    }

    #[test]
    fn test_tab_controller_navigation() {
        let mut controller = TabController::new()
            .add_tab("tab1", "First", MockComponent::create(MockProps))
            .add_tab("tab2", "Second", MockComponent::create(MockProps))
            .add_tab("tab3", "Third", MockComponent::create(MockProps));

        controller.select_next();
        assert_eq!(controller.active_index(), 1);

        controller.select_next();
        assert_eq!(controller.active_index(), 2);

        controller.select_prev();
        assert_eq!(controller.active_index(), 1);

        controller.select_first();
        assert_eq!(controller.active_index(), 0);

        controller.select_last();
        assert_eq!(controller.active_index(), 2);
    }

    #[test]
    fn test_tab_controller_wrap_around() {
        let config = TabControllerConfig::new().wrap_around(true);
        let mut controller = TabController::new()
            .with_config(config)
            .add_tab("tab1", "First", MockComponent::create(MockProps))
            .add_tab("tab2", "Second", MockComponent::create(MockProps));

        controller.select_last();
        assert_eq!(controller.active_index(), 1);

        controller.select_next();
        assert_eq!(controller.active_index(), 0);

        controller.select_prev();
        assert_eq!(controller.active_index(), 1);
    }

    #[test]
    fn test_tab_controller_remove_tab() {
        let mut controller = TabController::new()
            .add_tab("tab1", "First", MockComponent::create(MockProps))
            .add_tab("tab2", "Second", MockComponent::create(MockProps))
            .add_tab("tab3", "Third", MockComponent::create(MockProps));

        controller.select(2);
        assert_eq!(controller.active_index(), 2);

        let removed = controller.remove_tab(2);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id, "tab3");
        assert_eq!(controller.len(), 2);
        assert_eq!(controller.active_index(), 1);
    }

    #[test]
    fn test_tab_controller_lifecycle() {
        let mut controller = TabController::new()
            .add_tab("tab1", "First", MockComponent::create(MockProps))
            .add_tab("tab2", "Second", MockComponent::create(MockProps));

        controller.init_all();

        assert_eq!(controller.tabs()[0].lifecycle(), TabLifecycle::Active);
        assert_eq!(controller.tabs()[1].lifecycle(), TabLifecycle::Initialized);

        controller.select(1);

        assert_eq!(controller.tabs()[0].lifecycle(), TabLifecycle::Inactive);
        assert_eq!(controller.tabs()[1].lifecycle(), TabLifecycle::Active);
    }

    #[test]
    fn test_tab_controller_active_tab() {
        let mut controller =
            TabController::new().add_tab("tab1", "First", MockComponent::create(MockProps));

        controller.tabs_mut()[0].component.value = 42;

        let tab = controller.active_tab().unwrap();
        assert_eq!(tab.label(), "First");
        assert_eq!(tab.component().value, 42);
    }

    #[test]
    fn test_tab_config_default() {
        let config = TabControllerConfig::default();
        assert!(!config.wrap_around);
        assert!(!config.remember_state);
        assert!(!config.lazy_init);
    }

    #[test]
    fn test_tab_config_builder() {
        let config = TabControllerConfig::new()
            .wrap_around(true)
            .remember_state(true)
            .lazy_init(true);

        assert!(config.wrap_around);
        assert!(config.remember_state);
        assert!(config.lazy_init);
    }
}
