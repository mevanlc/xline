use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;

#[derive(Default)]
pub struct UsageComponent;

impl UsageComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for UsageComponent {
    fn collect(&self, _input: &InputData) -> Option<ComponentData> {
        // TODO: Implement API usage fetching (requires HTTP client + OAuth)
        None
    }

    fn id(&self) -> ComponentId {
        ComponentId::Usage
    }
}
