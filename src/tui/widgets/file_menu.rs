use crate::tui::app::FILE_MENU_ITEMS;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem},
};

pub fn render(f: &mut Frame, area: Rect, selection: usize) {
    let popup_outer = centered_rect(38, FILE_MENU_ITEMS.len() as u16 + 4, area);
    let popup = Rect::new(
        popup_outer.x.saturating_add(1),
        popup_outer.y.saturating_add(1),
        popup_outer.width.saturating_sub(2),
        popup_outer.height.saturating_sub(2),
    );
    f.render_widget(Clear, popup_outer);

    let items: Vec<ListItem> = FILE_MENU_ITEMS
        .iter()
        .enumerate()
        .map(|(i, (label, mnemonic))| {
            let style = if i == selection {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let cursor = if i == selection { "> " } else { "  " };
            let spans = mnemonic_line(cursor, label, *mnemonic, style, i == selection);
            ListItem::new(Line::from(spans))
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

fn mnemonic_line(
    cursor: &str,
    label: &str,
    mnemonic: char,
    style: Style,
    selected: bool,
) -> Vec<Span<'static>> {
    let mnemonic_lower = mnemonic.to_ascii_lowercase();
    let base_color = if selected {
        Color::Yellow
    } else {
        Color::White
    };
    let mnemonic_style = Style::default()
        .fg(if selected {
            Color::Cyan
        } else {
            Color::LightCyan
        })
        .add_modifier(Modifier::BOLD);
    let label_style = if selected {
        Style::default().fg(base_color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(base_color)
    };

    let mut spans = vec![Span::styled(cursor.to_string(), style)];

    let mut highlighted = false;
    for ch in label.chars() {
        if !highlighted && ch.to_ascii_lowercase() == mnemonic_lower {
            spans.push(Span::styled(ch.to_string(), mnemonic_style));
            highlighted = true;
        } else {
            spans.push(Span::styled(ch.to_string(), label_style));
        }
    }

    spans
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [vert] = vertical.areas(area);
    let [rect] = horizontal.areas(vert);
    rect
}
