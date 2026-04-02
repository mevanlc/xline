pub mod context_window;
pub mod cost;
pub mod directory;
pub mod git;
pub mod model;
pub mod output_style;
pub mod session;
pub mod usage;

use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

pub trait Component {
    fn collect(&self, input: &InputData) -> Option<ComponentData>;
    fn id(&self) -> ComponentId;
}

#[derive(Debug, Clone)]
pub struct ComponentData {
    pub primary: String,
    pub secondary: String,
    pub metadata: HashMap<String, String>,
}

pub use context_window::ContextWindowComponent;
pub use cost::CostComponent;
pub use directory::DirectoryComponent;
pub use git::GitComponent;
pub use model::ModelComponent;
pub use output_style::OutputStyleComponent;
pub use session::SessionComponent;
pub use usage::UsageComponent;
