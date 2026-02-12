use crate::config::theme::UserTheme;
use crate::config::types::{AnsiColor, ComponentConfig, ComponentId, StyleMode};
use crate::core::components::ComponentData;

pub struct StatusLineGenerator<'a> {
    theme: &'a UserTheme,
}

impl<'a> StatusLineGenerator<'a> {
    pub fn new(theme: &'a UserTheme) -> Self {
        Self { theme }
    }

    pub fn generate(&self, components: Vec<(ComponentConfig, ComponentData)>) -> String {
        let enabled: Vec<_> = components
            .into_iter()
            .filter(|(cfg, _)| cfg.enabled)
            .collect();

        let mut rendered = Vec::new();
        for (cfg, data) in &enabled {
            let s = self.render_component(cfg, data);
            if !s.is_empty() {
                rendered.push((s, cfg));
            }
        }

        if rendered.is_empty() {
            return String::new();
        }

        let sep_glyph = self.theme.separator_glyph();

        // Check if powerline mode with arrow separator
        if sep_glyph.contains('\u{e0b0}') || sep_glyph.is_empty() {
            self.join_powerline(&rendered)
        } else {
            self.join_with_separator(&rendered, sep_glyph)
        }
    }

    fn render_component(&self, cfg: &ComponentConfig, data: &ComponentData) -> String {
        let icon = if let Some(dynamic) = data.metadata.get("dynamic_icon") {
            dynamic.clone()
        } else {
            self.get_icon(cfg)
        };

        if let Some(bg) = &cfg.colors.background {
            let bg_code = bg_ansi(bg);

            let icon_colored = if let Some(ic) = &cfg.colors.icon {
                fg_ansi_no_reset(ic, &icon)
            } else {
                icon.clone()
            };

            let text_styled = style_no_reset(&data.primary, cfg.colors.text.as_ref(), cfg.styles.text_bold);

            let mut content = format!(" {} {} ", icon_colored, text_styled);

            if !data.secondary.is_empty() {
                let sec = style_no_reset(&data.secondary, cfg.colors.text.as_ref(), cfg.styles.text_bold);
                content.push_str(&format!("{} ", sec));
            }

            format!("{}{}\x1b[49m", bg_code, content)
        } else {
            let icon_colored = apply_color(&icon, cfg.colors.icon.as_ref());
            let text_styled = apply_style(&data.primary, cfg.colors.text.as_ref(), cfg.styles.text_bold);

            let mut segment = format!("{} {}", icon_colored, text_styled);

            if !data.secondary.is_empty() {
                segment.push_str(&format!(
                    " {}",
                    apply_style(&data.secondary, cfg.colors.text.as_ref(), cfg.styles.text_bold)
                ));
            }

            segment
        }
    }

    fn get_icon(&self, cfg: &ComponentConfig) -> String {
        match self.theme.style.mode {
            StyleMode::Plain => cfg.icon.plain.clone(),
            StyleMode::NerdFont | StyleMode::Powerline => cfg.icon.nerd_font.clone(),
        }
    }

    fn join_with_separator(&self, rendered: &[(String, &ComponentConfig)], sep_glyph: &str) -> String {
        let sep_cfg = self.theme.separator();
        let sep_color = sep_cfg.and_then(|s| s.colors.icon.as_ref());
        let colored_sep = apply_color(sep_glyph, sep_color);

        let parts: Vec<&str> = rendered.iter().map(|(s, _)| s.as_str()).collect();
        parts.join(&colored_sep)
    }

    fn join_powerline(&self, rendered: &[(String, &ComponentConfig)]) -> String {
        if rendered.len() == 1 {
            return rendered[0].0.clone();
        }

        let mut result = rendered[0].0.clone();

        for i in 1..rendered.len() {
            let prev_bg = rendered[i - 1].1.colors.background.as_ref();
            let curr_bg = rendered[i].1.colors.background.as_ref();
            result.push_str(&powerline_arrow(prev_bg, curr_bg));
            result.push_str(&rendered[i].0);
        }

        result.push_str("\x1b[0m");
        result
    }
}

pub fn collect_all_components(
    theme: &UserTheme,
    input: &crate::core::input::InputData,
) -> Vec<(ComponentConfig, ComponentData)> {
    use crate::core::components::*;

    let mut results = Vec::new();

    for comp_cfg in &theme.components {
        if !comp_cfg.enabled || comp_cfg.id == ComponentId::Separator {
            continue;
        }

        let data = match comp_cfg.id {
            ComponentId::Model => ModelComponent::new().collect(input),
            ComponentId::Directory => DirectoryComponent::new().collect(input),
            ComponentId::Git => {
                let show_sha = comp_cfg
                    .options
                    .get("show_sha")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                GitComponent::new().with_sha(show_sha).collect(input)
            }
            ComponentId::ContextWindow => ContextWindowComponent::new().collect(input),
            ComponentId::Usage => UsageComponent::new().collect(input),
            ComponentId::Cost => CostComponent::new().collect(input),
            ComponentId::Session => SessionComponent::new().collect(input),
            ComponentId::OutputStyle => OutputStyleComponent::new().collect(input),
            ComponentId::Separator => unreachable!(),
        };

        if let Some(data) = data {
            results.push((comp_cfg.clone(), data));
        }
    }

    results
}

// --- ANSI helpers ---

fn apply_color(text: &str, color: Option<&AnsiColor>) -> String {
    match color {
        Some(AnsiColor::Color16 { c16 }) => {
            let code = if *c16 < 8 { 30 + c16 } else { 90 + (c16 - 8) };
            format!("\x1b[{}m{}\x1b[0m", code, text)
        }
        Some(AnsiColor::Color256 { c256 }) => {
            format!("\x1b[38;5;{}m{}\x1b[0m", c256, text)
        }
        Some(AnsiColor::Rgb { r, g, b }) => {
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        }
        None => text.to_string(),
    }
}

fn apply_style(text: &str, color: Option<&AnsiColor>, bold: bool) -> String {
    let mut codes = Vec::new();
    if bold {
        codes.push("1".into());
    }
    match color {
        Some(AnsiColor::Color16 { c16 }) => {
            codes.push(if *c16 < 8 { (30 + c16).to_string() } else { (90 + (c16 - 8)).to_string() });
        }
        Some(AnsiColor::Color256 { c256 }) => {
            codes.extend(["38".into(), "5".into(), c256.to_string()]);
        }
        Some(AnsiColor::Rgb { r, g, b }) => {
            codes.extend(["38".into(), "2".into(), r.to_string(), g.to_string(), b.to_string()]);
        }
        None => {}
    }
    if codes.is_empty() {
        text.to_string()
    } else {
        format!("\x1b[{}m{}\x1b[0m", codes.join(";"), text)
    }
}

fn fg_ansi_no_reset(color: &AnsiColor, text: &str) -> String {
    match color {
        AnsiColor::Color16 { c16 } => {
            let code = if *c16 < 8 { 30 + c16 } else { 90 + (c16 - 8) };
            format!("\x1b[{}m{}", code, text)
        }
        AnsiColor::Color256 { c256 } => format!("\x1b[38;5;{}m{}", c256, text),
        AnsiColor::Rgb { r, g, b } => format!("\x1b[38;2;{};{};{}m{}", r, g, b, text),
    }
}

fn style_no_reset(text: &str, color: Option<&AnsiColor>, bold: bool) -> String {
    let mut codes = Vec::new();
    if bold {
        codes.push("1".into());
    }
    match color {
        Some(AnsiColor::Color16 { c16 }) => {
            codes.push(if *c16 < 8 { (30 + c16).to_string() } else { (90 + (c16 - 8)).to_string() });
        }
        Some(AnsiColor::Color256 { c256 }) => {
            codes.extend(["38".into(), "5".into(), c256.to_string()]);
        }
        Some(AnsiColor::Rgb { r, g, b }) => {
            codes.extend(["38".into(), "2".into(), r.to_string(), g.to_string(), b.to_string()]);
        }
        None => {}
    }
    if codes.is_empty() {
        text.to_string()
    } else {
        format!("\x1b[{}m{}", codes.join(";"), text)
    }
}

fn bg_ansi(color: &AnsiColor) -> String {
    match color {
        AnsiColor::Color16 { c16 } => {
            let code = if *c16 < 8 { 40 + c16 } else { 100 + (c16 - 8) };
            format!("\x1b[{}m", code)
        }
        AnsiColor::Color256 { c256 } => format!("\x1b[48;5;{}m", c256),
        AnsiColor::Rgb { r, g, b } => format!("\x1b[48;2;{};{};{}m", r, g, b),
    }
}

fn powerline_arrow(prev_bg: Option<&AnsiColor>, curr_bg: Option<&AnsiColor>) -> String {
    let arrow = "\u{e0b0}";
    match (prev_bg, curr_bg) {
        (Some(prev), Some(curr)) => {
            let fg = fg_code(prev);
            let bg = bg_ansi(curr);
            format!("{}{}{}\x1b[0m", bg, fg, arrow)
        }
        (Some(prev), None) => {
            let fg = fg_code(prev);
            format!("{}{}\x1b[0m", fg, arrow)
        }
        (None, Some(curr)) => {
            let bg = bg_ansi(curr);
            format!("{}{}\x1b[0m", bg, arrow)
        }
        (None, None) => arrow.to_string(),
    }
}

fn fg_code(color: &AnsiColor) -> String {
    match color {
        AnsiColor::Color16 { c16 } => {
            let code = if *c16 < 8 { 30 + c16 } else { 90 + (c16 - 8) };
            format!("\x1b[{}m", code)
        }
        AnsiColor::Color256 { c256 } => format!("\x1b[38;5;{}m", c256),
        AnsiColor::Rgb { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
    }
}
