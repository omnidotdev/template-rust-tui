//! UI rendering.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

/// Render the UI.
#[allow(clippy::cast_possible_truncation)]
pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Content
            Constraint::Length(3), // Input
        ])
        .split(frame.area());

    // Header.
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            " {{project-name}} ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("| "),
        Span::styled("Ctrl+C", Style::default().fg(Color::DarkGray)),
        Span::raw(" quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    // Content.
    let content_height = chunks[1].height.saturating_sub(2);
    let total_lines = app.output.len().min(usize::from(u16::MAX)) as u16;
    let max_scroll = total_lines.saturating_sub(content_height);

    // Clamp scroll offset.
    if app.auto_scroll || app.scroll_offset > max_scroll {
        app.scroll_offset = max_scroll;
    }

    let visible_lines: Vec<Line> = app
        .output
        .iter()
        .skip(app.scroll_offset as usize)
        .take(content_height as usize)
        .map(|s| Line::from(s.as_str()))
        .collect();

    let content = Paragraph::new(visible_lines)
        .block(Block::default().borders(Borders::ALL).title(" Output "))
        .wrap(Wrap { trim: false });
    frame.render_widget(content, chunks[1]);

    // Input.
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title(" Input "))
        .style(Style::default().fg(Color::White));
    frame.render_widget(input, chunks[2]);

    // Cursor position.
    let cursor_x = app.input[..app.cursor]
        .chars()
        .count()
        .min(usize::from(u16::MAX)) as u16;
    frame.set_cursor_position(Position::new(
        chunks[2].x + 1 + cursor_x,
        chunks[2].y + 1,
    ));
}
