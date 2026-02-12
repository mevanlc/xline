use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::key_hints;

pub struct HelpBarWidget;

impl HelpBarWidget {
    pub fn render(
        f: &mut Frame,
        area: Rect,
        status: Option<&str>,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);

        // Keymap (left)
        let grid = key_hints::render_grid(&[
            &[("Tab", "Panel"), ("\u{2191}\u{2193}", "Nav"),    ("\u{23ce}", "Edit"),     ("I", "Colors")],
            &[("^S", "Menu"),   ("\u{2190}\u{2192}", "Theme"),  ("\u{23e1}", "Toggle"),  ("K", "Icons")],
        ]);

        let keymap_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Keymap ");

        let keymap = Paragraph::new(grid).block(keymap_block);
        f.render_widget(keymap, chunks[0]);

        // Status (right)
        let status_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Status ");

        let status_text = match status {
            Some(msg) => Span::styled(format!(" {}", msg), Style::default().fg(Color::Blue)),
            None => Span::raw(""),
        };

        let status_widget = Paragraph::new(status_text).block(status_block);
        f.render_widget(status_widget, chunks[1]);
    }
}
