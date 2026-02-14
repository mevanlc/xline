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

    /// Get icons for a specific component.
    pub fn get(&self, id: ComponentId) -> Option<&ComponentIcons> {
        self.entries.iter().find(|(cid, _)| *cid == id).map(|(_, i)| i)
    }

    /// Whether any single theme's components supply all of this set's icons.
    pub fn is_supplied_by(&self, theme_components: &[crate::config::types::ComponentConfig]) -> bool {
        self.entries.iter().all(|(id, icons)| {
            theme_components
                .iter()
                .find(|c| c.id == *id)
                .is_some_and(|comp| {
                    comp.icon.plain == icons.plain && comp.icon.nerd_font == icons.nerd_font
                })
        })
    }

    /// Whether this icon set uses powerline arrows as separators.
    pub fn is_powerline(&self) -> bool {
        self.get(ComponentId::Separator)
            .map_or(false, |ic| ic.nerd_font.contains('\u{e0b0}'))
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
