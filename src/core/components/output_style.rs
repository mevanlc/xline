use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct OutputStyleComponent;

impl OutputStyleComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for OutputStyleComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let style = input.output_style.as_ref()?;

        let mut metadata = HashMap::new();
        metadata.insert("style_name".into(), style.name.clone());

        Some(ComponentData {
            primary: style.name.clone(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::OutputStyle
    }
}
