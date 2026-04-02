mod cometix;
mod default;
mod gruvbox;
mod minimal;
mod nord;
mod powerline_dark;
mod powerline_light;
mod rose_pine;
mod tokyo_night;

use crate::config::types::{AnsiColor, ComponentId, TextStyleConfig};

/// Colors and text style for a single component within a color scheme.
#[derive(Debug, Clone)]
pub struct ComponentColors {
    pub icon: Option<AnsiColor>,
    pub text: Option<AnsiColor>,
    pub background: Option<AnsiColor>,
    pub text_bold: bool,
}

/// A built-in color scheme preset. Contains colors only — no icons.
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub name: &'static str,
    pub description: &'static str,
    entries: Vec<(ComponentId, ComponentColors)>,
}

impl ColorScheme {
    pub fn new(
        name: &'static str,
        description: &'static str,
        entries: Vec<(ComponentId, ComponentColors)>,
    ) -> Self {
        Self {
            name,
            description,
            entries,
        }
    }

    /// Get colors for a specific component.
    pub fn get(&self, id: ComponentId) -> Option<&ComponentColors> {
        self.entries
            .iter()
            .find(|(cid, _)| *cid == id)
            .map(|(_, c)| c)
    }

    /// Whether any single theme's components supply all of this scheme's colors.
    pub fn is_supplied_by(
        &self,
        theme_components: &[crate::config::types::ComponentConfig],
    ) -> bool {
        self.entries.iter().all(|(id, colors)| {
            theme_components
                .iter()
                .find(|c| c.id == *id)
                .is_some_and(|comp| {
                    comp.colors.icon == colors.icon
                        && comp.colors.text == colors.text
                        && comp.colors.background == colors.background
                        && comp.styles.text_bold == colors.text_bold
                })
        })
    }

    /// Whether this color scheme is designed for powerline mode.
    pub fn is_powerline(&self) -> bool {
        self.name.to_ascii_lowercase().contains("powerline")
    }

    /// Apply this color scheme to a theme's components (mutates in place).
    pub fn apply_to(&self, components: &mut [crate::config::types::ComponentConfig]) {
        for comp in components.iter_mut() {
            if let Some(colors) = self.get(comp.id) {
                comp.colors.icon = colors.icon.clone();
                comp.colors.text = colors.text.clone();
                comp.colors.background = colors.background.clone();
                comp.styles = TextStyleConfig {
                    text_bold: colors.text_bold,
                };
            }
        }
    }
}

/// Return all built-in color schemes.
pub fn all() -> Vec<ColorScheme> {
    vec![
        default::scheme(),
        cometix::scheme(),
        gruvbox::scheme(),
        nord::scheme(),
        minimal::scheme(),
        powerline_dark::scheme(),
        powerline_light::scheme(),
        rose_pine::scheme(),
        tokyo_night::scheme(),
    ]
}

/// Find a color scheme by name (case-insensitive).
pub fn find(name: &str) -> Option<ColorScheme> {
    all()
        .into_iter()
        .find(|s| s.name.eq_ignore_ascii_case(name))
}
