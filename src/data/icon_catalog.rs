use super::nerd_fonts;

/// A single icon entry: the icon string and a display name for search.
#[derive(Clone)]
pub struct IconEntry {
    pub icon: String,
    pub name: String,
}

/// A named section of icons (e.g., "Common Emoji", "All Emoji").
pub struct IconSection {
    pub title: String,
    pub entries: Vec<IconEntry>,
}

/// Pre-built icon catalog data, held on App for the lifetime of the process.
pub struct IconCatalogData {
    emoji_common: Vec<IconEntry>,
    emoji_all: Vec<IconEntry>,
    nerd_common: Vec<IconEntry>,
    nerd_all: Vec<IconEntry>,
    unicode_common: Vec<IconEntry>,
    unicode_all: Vec<IconEntry>,
}

/// Common emoji for the "Common" section — curated for statusline use.
const COMMON_EMOJI: &[&str] = &[
    "🌱", "🔧", "⚡", "⭐", "✨", "🔥", "💎", "🤖", "🎯", "🚀",
    "📁", "🌿", "📊", "💰", "⏱\u{fe0f}", "🎨", "💡", "🔒",
];

/// Common nerd font glyph name prefixes to match.
const COMMON_NERD_NAMES: &[&str] = &[
    "cod hubot",         // robot
    "md folder",         // folder
    "md git",            // git
    "oct zap",           // zap/lightning
    "md chart bar",      // chart
    "cod credit card",   // cost
    "md timer",          // timer
    "md target",         // target
    "md rocket launch",  // rocket
    "seti code",         // code
];

/// Common Unicode symbols for statusline use.
const COMMON_UNICODE: &[(&str, &str)] = &[
    ("●", "Black Circle"),
    ("◆", "Black Diamond"),
    ("★", "Black Star"),
    ("→", "Rightwards Arrow"),
    ("│", "Box Drawings Light Vertical"),
    ("■", "Black Square"),
    ("▲", "Black Up-Pointing Triangle"),
    ("○", "White Circle"),
    ("✦", "Black Four Pointed Star"),
    ("⟩", "Mathematical Right Angle Bracket"),
    ("·", "Middle Dot"),
    ("»", "Right-Pointing Double Angle Quotation Mark"),
    ("✓", "Check Mark"),
    ("✗", "Ballot X"),
];

impl IconCatalogData {
    /// Build the full catalog. Called once at startup.
    pub fn load() -> Self {
        let emoji_common = build_emoji_common();
        let emoji_all = build_emoji_all();
        let nerd_all_raw = nerd_fonts::load();
        let (nerd_common, nerd_all) = build_nerd_sections(&nerd_all_raw);
        let unicode_common = build_unicode_common();
        let unicode_all = build_unicode_all();

        Self {
            emoji_common,
            emoji_all,
            nerd_common,
            nerd_all,
            unicode_common,
            unicode_all,
        }
    }

    /// Return filtered sections for a given tab and search query.
    pub fn sections(&self, tab: IconPickerTab, query: &str) -> Vec<IconSection> {
        let query_lower = query.to_lowercase();

        match tab {
            IconPickerTab::Emoji => filter_two_sections(
                "Common Emoji",
                &self.emoji_common,
                "All Emoji",
                &self.emoji_all,
                &query_lower,
            ),
            IconPickerTab::NerdFont => filter_two_sections(
                "Common",
                &self.nerd_common,
                "All Nerd Font",
                &self.nerd_all,
                &query_lower,
            ),
            IconPickerTab::Unicode => filter_two_sections(
                "Common",
                &self.unicode_common,
                "All Unicode",
                &self.unicode_all,
                &query_lower,
            ),
            IconPickerTab::Custom => Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconPickerTab {
    Emoji,
    NerdFont,
    Unicode,
    Custom,
}

// --- builders ---

fn build_emoji_common() -> Vec<IconEntry> {
    COMMON_EMOJI
        .iter()
        .filter_map(|s| {
            let emoji = emojis::get(s)?;
            Some(IconEntry {
                icon: emoji.as_str().to_string(),
                name: emoji.name().to_string(),
            })
        })
        .collect()
}

fn build_emoji_all() -> Vec<IconEntry> {
    emojis::iter()
        .map(|emoji| IconEntry {
            icon: emoji.as_str().to_string(),
            name: emoji.name().to_string(),
        })
        .collect()
}

fn build_nerd_sections(all: &[nerd_fonts::NerdFontGlyph]) -> (Vec<IconEntry>, Vec<IconEntry>) {
    let common: Vec<IconEntry> = COMMON_NERD_NAMES
        .iter()
        .filter_map(|prefix| {
            all.iter().find(|g| g.name == *prefix).map(|g| IconEntry {
                icon: g.icon.clone(),
                name: g.name.clone(),
            })
        })
        .collect();

    let all_entries: Vec<IconEntry> = all
        .iter()
        .map(|g| IconEntry {
            icon: g.icon.clone(),
            name: g.name.clone(),
        })
        .collect();

    (common, all_entries)
}

fn build_unicode_common() -> Vec<IconEntry> {
    COMMON_UNICODE
        .iter()
        .map(|(icon, name)| IconEntry {
            icon: icon.to_string(),
            name: name.to_string(),
        })
        .collect()
}

fn build_unicode_all() -> Vec<IconEntry> {
    let mut entries = Vec::new();
    for code in 0u32..=0x10FFFF {
        // Skip surrogates
        if (0xD800..=0xDFFF).contains(&code) {
            continue;
        }
        if let Some(ch) = char::from_u32(code) {
            if let Some(name) = unicode_names2::name(ch) {
                let name_str = name.to_string();
                // Skip control characters and uninteresting blocks
                if name_str.starts_with('<') {
                    continue;
                }
                entries.push(IconEntry {
                    icon: ch.to_string(),
                    name: name_str,
                });
            }
        }
    }
    entries
}

/// Filter two sections by query, returning only non-empty sections.
fn filter_two_sections(
    common_title: &str,
    common: &[IconEntry],
    all_title: &str,
    all: &[IconEntry],
    query: &str,
) -> Vec<IconSection> {
    let mut sections = Vec::new();

    if query.is_empty() {
        // No filter — return both sections as-is
        if !common.is_empty() {
            sections.push(IconSection {
                title: common_title.to_string(),
                entries: common.to_vec(),
            });
        }
        if !all.is_empty() {
            sections.push(IconSection {
                title: all_title.to_string(),
                entries: all.to_vec(),
            });
        }
    } else {
        // Filter both sections
        let filtered_common: Vec<IconEntry> = common
            .iter()
            .filter(|e| e.name.to_lowercase().contains(query))
            .cloned()
            .collect();
        let filtered_all: Vec<IconEntry> = all
            .iter()
            .filter(|e| e.name.to_lowercase().contains(query))
            .cloned()
            .collect();

        if !filtered_common.is_empty() {
            sections.push(IconSection {
                title: common_title.to_string(),
                entries: filtered_common,
            });
        }
        if !filtered_all.is_empty() {
            sections.push(IconSection {
                title: all_title.to_string(),
                entries: filtered_all,
            });
        }
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_loads_with_expected_counts() {
        let catalog = IconCatalogData::load();

        let emoji = catalog.sections(IconPickerTab::Emoji, "");
        let emoji_total: usize = emoji.iter().map(|s| s.entries.len()).sum();
        assert!(emoji_total > 1000, "expected >1000 emoji, got {}", emoji_total);

        let nerd = catalog.sections(IconPickerTab::NerdFont, "");
        let nerd_total: usize = nerd.iter().map(|s| s.entries.len()).sum();
        assert!(nerd_total > 5000, "expected >5000 nerd font glyphs, got {}", nerd_total);

        let uni = catalog.sections(IconPickerTab::Unicode, "");
        let uni_total: usize = uni.iter().map(|s| s.entries.len()).sum();
        assert!(uni_total > 10000, "expected >10000 unicode chars, got {}", uni_total);
    }

    #[test]
    fn search_filters_results() {
        let catalog = IconCatalogData::load();
        let sections = catalog.sections(IconPickerTab::Emoji, "rocket");
        let total: usize = sections.iter().map(|s| s.entries.len()).sum();
        assert!(total >= 1, "expected at least 1 result for 'rocket'");
        assert!(total < 50, "expected <50 results for 'rocket', got {}", total);
    }
}
