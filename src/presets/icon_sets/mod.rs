mod emoji;
mod minimal;
mod nerd_font;
mod powerline;

use crate::config::types::ComponentId;

/// Icons for a single component within an icon set.
#[derive(Debug, Clone)]
pub struct ComponentIcons {
    pub plain: &'static str,
    pub nerd_font: &'static str,
}

/// A built-in icon set preset. Contains icons only — no colors.
/// Includes separator glyph as the Separator component.
#[derive(Debug, Clone)]
pub struct IconSet {
    pub name: &'static str,
    pub description: &'static str,
    entries: Vec<(ComponentId, ComponentIcons)>,
}

impl IconSet {
    pub fn new(
        name: &'static str,
        description: &'static str,
        entries: Vec<(ComponentId, ComponentIcons)>,
    ) -> Self {
        Self {
            name,
            description,
            entries,
        }
    }

    /// Iterate over all (ComponentId, ComponentIcons) entries.
    pub fn icons_iter(&self) -> impl Iterator<Item = &(ComponentId, ComponentIcons)> {
        self.entries.iter()
    }

    /// Get icons for a specific component.
    pub fn get(&self, id: ComponentId) -> Option<&ComponentIcons> {
        self.entries.iter().find(|(cid, _)| *cid == id).map(|(_, i)| i)
    }

    /// Get a preview of all icons (plain mode) as a string, excluding separator.
    pub fn preview_plain(&self) -> String {
        self.entries
            .iter()
            .filter(|(id, _)| *id != ComponentId::Separator)
            .map(|(_, icons)| icons.plain)
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Get a preview of all icons (nerd font mode) as a string, excluding separator.
    pub fn preview_nerd_font(&self) -> String {
        self.entries
            .iter()
            .filter(|(id, _)| *id != ComponentId::Separator)
            .map(|(_, icons)| icons.nerd_font)
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Apply this icon set to a theme's components (mutates in place).
    pub fn apply_to(
        &self,
        components: &mut [crate::config::types::ComponentConfig],
    ) {
        for comp in components.iter_mut() {
            if let Some(icons) = self.get(comp.id) {
                comp.icon.plain = icons.plain.to_string();
                comp.icon.nerd_font = icons.nerd_font.to_string();
            }
        }
    }
}

/// Return all built-in icon sets.
pub fn all() -> Vec<IconSet> {
    vec![
        emoji::icon_set(),
        nerd_font::icon_set(),
        minimal::icon_set(),
        powerline::icon_set(),
    ]
}

/// Find an icon set by name (case-insensitive).
pub fn find(name: &str) -> Option<IconSet> {
    all().into_iter().find(|s| s.name.eq_ignore_ascii_case(name))
}
