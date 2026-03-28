use super::{Component, ComponentData};
use crate::config::types::{ComponentId, PerModelIcons};
use crate::core::input::InputData;
use std::collections::HashMap;

pub struct ModelComponent {
    per_model: Option<PerModelIcons>,
}

impl Default for ModelComponent {
    fn default() -> Self {
        Self { per_model: None }
    }
}

impl ModelComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_per_model(mut self, per_model: Option<PerModelIcons>) -> Self {
        self.per_model = per_model;
        self
    }
}

impl Component for ModelComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let mut metadata = HashMap::new();
        metadata.insert("model_id".into(), input.model.id.clone());
        metadata.insert("display_name".into(), input.model.display_name.clone());

        if let Some(pm) = &self.per_model {
            if pm.enabled {
                let model_id = input.model.id.to_ascii_lowercase();
                let icon = if model_id.contains("opus") {
                    &pm.opus
                } else if model_id.contains("haiku") {
                    &pm.haiku
                } else {
                    &pm.sonnet
                };
                if !icon.is_empty() {
                    metadata.insert("dynamic_icon".into(), icon.clone());
                }
            }
        }

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
