use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::key_hints;

pub struct HelpBarWidget;

impl HelpBarWidget {
    pub fn render(f: &mut Frame, area: Rect) {
        let grid = key_hints::render_grid(&[
            &[
                ("\u{2190}\u{2192}", "Panel"),
                ("\u{2191}\u{2193}", "Nav"),
                ("Space", "Edit/Toggle"),
                ("^S", "Menu"),
                ("A/D", "Theme"),
                ("S+\u{2191}\u{2193}", "Reorder"),
                ("C", "Colors"),
                ("I", "Icons"),
            ],
        ]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Keymap ");

        let keymap = Paragraph::new(grid).block(block);
        f.render_widget(keymap, area);
    }
}
