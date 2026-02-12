use crate::config::manager;
use crate::config::types::ComponentId;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn render_colors(f: &mut Frame, area: Rect, selection: usize) {
    let schemes = crate::presets::color_schemes::all();
    let user_themes = manager::list_themes().unwrap_or_default();

    let mut items: Vec<ListItem> = Vec::new();

    for (i, scheme) in schemes.iter().enumerate() {
        let style = item_style(i, selection);
        let cursor = if i == selection { "> " } else { "  " };
        items.push(ListItem::new(Line::from(Span::styled(
            format!("{}{} - {}", cursor, scheme.name, scheme.description),
            style,
        ))));
    }

    if !user_themes.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "\u{2500}\u{2500}\u{2500} User Themes \u{2500}\u{2500}\u{2500}",
            Style::default().fg(Color::DarkGray),
        ))));
    }

    for (i, (name, _)) in user_themes.iter().enumerate() {
        let idx = schemes.len() + i;
        let style = item_style(idx, selection);
        let cursor = if idx == selection { "> " } else { "  " };
        items.push(ListItem::new(Line::from(Span::styled(
            format!("{}{}", cursor, name),
            style,
        ))));
    }

    let height = (items.len() as u16 + 2).min(area.height.saturating_sub(4));
    let popup = centered_rect(50, height, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(" Import Color Scheme ");

    let list = List::new(items).block(block);
    f.render_widget(list, popup);
}

pub fn render_icons(f: &mut Frame, area: Rect, selection: usize) {
    let icon_sets = crate::presets::icon_sets::all();
    let user_themes = manager::list_themes().unwrap_or_default();

    // Compute max name display width for alignment
    let max_name_width = icon_sets
        .iter()
        .map(|s| UnicodeWidthStr::width(s.name))
        .chain(user_themes.iter().map(|(n, _)| UnicodeWidthStr::width(n.as_str())))
        .max()
        .unwrap_or(0);

    // Collect per-set component icons and separator (plain mode)
    let icon_rows: Vec<(Vec<&str>, &str)> = icon_sets
        .iter()
        .map(|set| {
            let icons: Vec<&str> = set
                .icons_iter()
                .filter(|(id, _)| *id != ComponentId::Separator)
                .map(|(_, ic)| ic.plain)
                .collect();
            let sep = set
                .get(ComponentId::Separator)
                .map(|ic| if ic.plain.is_empty() { ic.nerd_font } else { ic.plain })
                .unwrap_or(" ");
            (icons, sep)
        })
        .collect();

    // Per-icon-column max display width
    let num_cols = icon_rows.iter().map(|(r, _)| r.len()).max().unwrap_or(0);
    let col_widths: Vec<usize> = (0..num_cols)
        .map(|col| {
            icon_rows
                .iter()
                .filter_map(|(row, _)| row.get(col))
                .map(|icon| UnicodeWidthStr::width(*icon))
                .max()
                .unwrap_or(1)
        })
        .collect();

    // Max separator display width across all sets
    let max_sep_width = icon_rows
        .iter()
        .map(|(_, sep)| UnicodeWidthStr::width(*sep).max(1))
        .max()
        .unwrap_or(1);

    let mut items: Vec<ListItem> = Vec::new();

    for (i, (set, (icons, sep))) in icon_sets.iter().zip(icon_rows.iter()).enumerate() {
        let style = item_style(i, selection);
        let cursor = if i == selection { "> " } else { "  " };
        let name_pad = max_name_width - UnicodeWidthStr::width(set.name);
        let mut line = format!("{}{}{} ", cursor, set.name, " ".repeat(name_pad));
        for (col, icon) in icons.iter().enumerate() {
            if col > 0 {
                let sep_w = UnicodeWidthStr::width(*sep).max(1);
                let sep_pad = max_sep_width - sep_w;
                line.push_str(sep);
                line.push_str(&" ".repeat(sep_pad));
            }
            let w = UnicodeWidthStr::width(*icon);
            let pad = col_widths.get(col).unwrap_or(&1) - w;
            line.push_str(icon);
            line.push_str(&" ".repeat(pad));
        }
        items.push(ListItem::new(Line::from(Span::styled(
            line.trim_end().to_string(),
            style,
        ))));
    }

    if !user_themes.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "\u{2500}\u{2500}\u{2500} User Themes \u{2500}\u{2500}\u{2500}",
            Style::default().fg(Color::DarkGray),
        ))));
    }

    for (i, (name, _)) in user_themes.iter().enumerate() {
        let idx = icon_sets.len() + i;
        let style = item_style(idx, selection);
        let cursor = if idx == selection { "> " } else { "  " };
        let name_pad = max_name_width - UnicodeWidthStr::width(name.as_str());
        items.push(ListItem::new(Line::from(Span::styled(
            format!("{}{}{} (theme)", cursor, name, " ".repeat(name_pad)),
            style,
        ))));
    }

    let height = (items.len() as u16 + 2).min(area.height.saturating_sub(4));
    let popup = centered_rect(50, height, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(" Import Icon Set ");

    let list = List::new(items).block(block);
    f.render_widget(list, popup);
}

fn item_style(index: usize, selection: usize) -> Style {
    if index == selection {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [vert] = vertical.areas(area);
    let [rect] = horizontal.areas(vert);
    rect
}
