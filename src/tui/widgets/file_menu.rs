use crate::tui::app::FILE_MENU_ITEMS;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, selection: usize) {
    let popup = centered_rect(36, FILE_MENU_ITEMS.len() as u16 + 2, area);
    f.render_widget(Clear, popup);

    let items: Vec<ListItem> = FILE_MENU_ITEMS
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let style = if i == selection {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let cursor = if i == selection { "> " } else { "  " };
            ListItem::new(Line::from(Span::styled(
                format!("{}{}", cursor, label),
                style,
            )))
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(" Save / Activate ");

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
