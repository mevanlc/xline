use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem},
};
use std::path::PathBuf;

pub fn render(f: &mut Frame, area: Rect, themes: &[(String, PathBuf)], selection: usize) {
    let mut items: Vec<ListItem> = Vec::new();

    for (i, (name, _)) in themes.iter().enumerate() {
        let style = if i == selection {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let cursor = if i == selection { "> " } else { "  " };
        items.push(ListItem::new(Line::from(Span::styled(
            format!("{}{}", cursor, name),
            style,
        ))));
    }

    let height = (items.len() as u16 + 2).min(area.height.saturating_sub(4));
    let popup = centered_rect(40, height, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(" Open Theme ");

    let list = List::new(items).block(block);
    f.render_widget(list, popup);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [vert] = vertical.areas(area);
    let [rect] = horizontal.areas(vert);
    rect
}
