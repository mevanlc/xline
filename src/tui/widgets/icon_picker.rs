use crate::data::icon_catalog::{IconCatalogData, IconPickerTab, IconSection};
use crate::tui::app::IconPickerState;
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

/// Items in the flattened icon list — either a section header or a selectable icon.
pub enum FlatItem {
    Header(String),
    Icon { icon: String, name: String },
}

/// Flatten sections into a list of headers and icon entries.
pub fn flatten_sections(sections: &[IconSection]) -> Vec<FlatItem> {
    let mut items = Vec::new();
    for section in sections {
        items.push(FlatItem::Header(section.title.clone()));
        for entry in &section.entries {
            items.push(FlatItem::Icon {
                icon: entry.icon.clone(),
                name: entry.name.clone(),
            });
        }
    }
    items
}

/// Count the number of selectable (non-header) items in the flat list.
pub fn selectable_count(flat: &[FlatItem]) -> usize {
    flat.iter()
        .filter(|item| matches!(item, FlatItem::Icon { .. }))
        .count()
}

/// Map a selectable index (0-based among Icon items only) to a flat-list index.
fn selectable_to_flat(flat: &[FlatItem], sel: usize) -> usize {
    let mut count = 0;
    for (i, item) in flat.iter().enumerate() {
        if matches!(item, FlatItem::Icon { .. }) {
            if count == sel {
                return i;
            }
            count += 1;
        }
    }
    0
}

pub fn render(f: &mut Frame, area: Rect, state: &IconPickerState, catalog: &IconCatalogData) {
    let popup = centered_rect(56, 26, area);
    f.render_widget(Clear, popup);

    let tab = *state.tab.current();
    let is_custom = tab == IconPickerTab::Custom;

    // Overall layout: outer block, then vertical sections inside
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" Icon Picker ");

    let inner = outer_block.inner(popup);
    f.render_widget(outer_block, popup);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Icon Set tabs
            Constraint::Length(3), // Search / Input
            Constraint::Min(3),   // Icons list (or empty for Custom)
            Constraint::Length(3), // Keymap
        ])
        .split(inner);

    // --- Icon Set tabs ---
    render_tabs(f, layout[0], state);

    // --- Search / Input ---
    if is_custom {
        render_custom_input(f, layout[1], state);
    } else {
        render_search(f, layout[1], state);
    }

    // --- Icons list ---
    if is_custom {
        // Empty area for custom tab
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Indexed(236)));
        f.render_widget(block, layout[2]);
    } else {
        render_icon_list(f, layout[2], state, catalog);
    }

    // --- Keymap ---
    render_keymap(f, layout[3], &[
        ("\u{2190}\u{2191}\u{2193}\u{2192}", "Navigate"),
        ("Enter", "Select"),
        ("Esc", "Cancel"),
    ]);
}

fn render_tabs(f: &mut Frame, area: Rect, state: &IconPickerState) {
    let tabs = [
        ("Emoji", IconPickerTab::Emoji),
        ("Nerd Font", IconPickerTab::NerdFont),
        ("Unicode", IconPickerTab::Unicode),
        ("Custom", IconPickerTab::Custom),
    ];

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::raw(" "));
    for (i, (label, tab)) in tabs.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", Style::default().fg(Color::DarkGray)));
        }
        let selected = state.tab == *tab;
        let indicator = if selected { "\u{2022}" } else { " " };
        let style = if selected {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Indexed(245))
        };
        spans.push(Span::styled(
            format!("[{}] {}", indicator, label),
            style,
        ));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(236)))
        .title(" Icon Set ");

    let line = Line::from(spans);
    let para = Paragraph::new(line).block(block);
    f.render_widget(para, area);
}

fn render_search(f: &mut Frame, area: Rect, state: &IconPickerState) {
    let cursor = "\u{2588}";
    let text = Line::from(vec![
        Span::raw(" "),
        Span::styled(&state.search_query, Style::default().fg(Color::White)),
        Span::styled(cursor, Style::default().fg(Color::Blue)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(Span::styled(
            " Search ",
            Style::default().fg(Color::Yellow),
        ));

    let para = Paragraph::new(text).block(block);
    f.render_widget(para, area);
}

fn render_custom_input(f: &mut Frame, area: Rect, state: &IconPickerState) {
    let cursor = "\u{2588}";
    let text = Line::from(vec![
        Span::raw(" "),
        Span::styled(&state.custom_buffer, Style::default().fg(Color::White)),
        Span::styled(cursor, Style::default().fg(Color::Blue)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(Span::styled(
            " Input ",
            Style::default().fg(Color::Yellow),
        ));

    let para = Paragraph::new(text).block(block);
    f.render_widget(para, area);
}

fn render_icon_list(
    f: &mut Frame,
    area: Rect,
    state: &IconPickerState,
    catalog: &IconCatalogData,
) {
    let tab = *state.tab.current();
    let sections = catalog.sections(tab, &state.search_query);
    let flat = flatten_sections(&sections);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(236)))
        .title(" Icons ");

    let inner = block.inner(area);
    f.render_widget(block, area);

    let visible_height = inner.height as usize;
    if visible_height == 0 {
        return;
    }

    // Map selected_index to flat position for highlight
    let selected_flat = selectable_to_flat(&flat, state.selected_index);

    // Compute scroll to keep selected visible
    let scroll = state.scroll_offset;

    let mut lines: Vec<Line> = Vec::new();
    for (i, item) in flat.iter().enumerate().skip(scroll).take(visible_height) {
        match item {
            FlatItem::Header(title) => {
                let dashes = "\u{2500}".repeat(3);
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{}\u{2500}{} ", dashes, dashes),
                        Style::default().fg(Color::Indexed(240)),
                    ),
                    Span::styled(
                        title.clone(),
                        Style::default().fg(Color::Indexed(245)),
                    ),
                    Span::styled(
                        format!(" {}", dashes),
                        Style::default().fg(Color::Indexed(240)),
                    ),
                ]));
            }
            FlatItem::Icon { icon, name } => {
                let is_selected = i == selected_flat;
                if is_selected {
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!(" {} ", icon),
                            Style::default().fg(Color::White),
                        ),
                        Span::styled(
                            name.clone(),
                            Style::default()
                                .fg(Color::White)
                                .bg(Color::Indexed(238))
                                .add_modifier(Modifier::BOLD),
                        ),
                        // Fill remaining width with selection background
                        Span::styled(
                            " ".repeat(inner.width.saturating_sub(
                                (icon.chars().count() + name.len() + 3) as u16,
                            )
                                as usize),
                            Style::default().bg(Color::Indexed(238)),
                        ),
                    ]));
                } else {
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!(" {} ", icon),
                            Style::default().fg(Color::White),
                        ),
                        Span::styled(name.clone(), Style::default().fg(Color::Indexed(250))),
                    ]));
                }
            }
        }
    }

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);

    // Page counter in bottom-right of the Icons block border
    if visible_height > 0 && !flat.is_empty() {
        let total_pages = (flat.len() + visible_height - 1) / visible_height;
        let current_page = scroll / visible_height + 1;
        let counter = format!(" Page {}/{} ", current_page, total_pages);
        let counter_width = counter.len() as u16;
        let counter_area = Rect {
            x: area.x + area.width.saturating_sub(counter_width + 1),
            y: area.y + area.height - 1,
            width: counter_width,
            height: 1,
        };
        f.render_widget(
            Paragraph::new(Span::styled(
                counter,
                Style::default().fg(Color::Indexed(240)),
            )),
            counter_area,
        );
    }
}

fn render_keymap(f: &mut Frame, area: Rect, hints: &[(&str, &str)]) {
    let line = super::key_hints::render(hints);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(236)))
        .title(" Keymap ");

    let para = Paragraph::new(line).block(block);
    f.render_widget(para, area);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [vert] = vertical.areas(area);
    let [rect] = horizontal.areas(vert);
    rect
}
