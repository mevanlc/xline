use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct ModelComponent;

impl ModelComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for ModelComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let mut metadata = HashMap::new();
        metadata.insert("model_id".into(), input.model.id.clone());
        metadata.insert("display_name".into(), input.model.display_name.clone());

        Some(ComponentData {
            primary: input.model.display_name.clone(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Model
    }
}
