use crate::config::manager;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem},
    Frame,
};

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

    let mut items: Vec<ListItem> = Vec::new();

    for (i, set) in icon_sets.iter().enumerate() {
        let style = item_style(i, selection);
        let cursor = if i == selection { "> " } else { "  " };
        items.push(ListItem::new(Line::from(Span::styled(
            format!("{}{} - {}", cursor, set.name, set.description),
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
