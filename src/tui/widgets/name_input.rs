use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, title: &str, buffer: &str) {
    let popup = centered_rect(40, 4, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(format!(" {} ", title));

    let cursor = "\u{2588}"; // █
    let text = Line::from(vec![
        Span::styled(buffer, Style::default().fg(Color::White)),
        Span::styled(cursor, Style::default().fg(Color::Blue)),
    ]);

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, popup);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [vert] = vertical.areas(area);
    let [rect] = horizontal.areas(vert);
    rect
}
