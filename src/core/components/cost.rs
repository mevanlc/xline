use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct CostComponent;

impl CostComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for CostComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let cost_data = input.cost.as_ref()?;
        let cost = cost_data.total_cost_usd?;

        let primary = if cost == 0.0 || cost < 0.01 {
            "$0".into()
        } else {
            format!("${:.2}", cost)
        };

        let mut metadata = HashMap::new();
        metadata.insert("cost".into(), cost.to_string());

        Some(ComponentData {
            primary,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Cost
    }
}
