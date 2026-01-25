//! Event handling.

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEventKind};

use crate::app::App;

/// Handle keyboard event.
pub fn handle_key(app: &mut App, key: KeyEvent, content_height: u16) {
    match (key.modifiers, key.code) {
        // Quit.
        (KeyModifiers::CONTROL, KeyCode::Char('c' | 'd'))
        | (KeyModifiers::NONE, KeyCode::Esc) => app.quit(),

        // Navigation.
        (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Left) => app.move_left(),
        (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Right) => app.move_right(),
        (KeyModifiers::CONTROL, KeyCode::Char('a')) => app.move_start(),
        (KeyModifiers::CONTROL, KeyCode::Char('e')) => app.move_end(),

        // Editing.
        (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Backspace) => app.delete_char(),
        (KeyModifiers::NONE, KeyCode::Delete) => app.delete_char_forward(),
        (KeyModifiers::CONTROL, KeyCode::Char('u')) => app.delete_to_start(),
        (KeyModifiers::CONTROL, KeyCode::Char('k')) => app.delete_to_end(),
        (KeyModifiers::CONTROL, KeyCode::Char('w')) => app.delete_word(),
        (KeyModifiers::CONTROL, KeyCode::Char('l')) => app.clear_input(),

        // Submit.
        (KeyModifiers::NONE, KeyCode::Enter) => app.submit(),

        // Scrolling.
        (KeyModifiers::NONE, KeyCode::Up) => app.scroll_up(1),
        (KeyModifiers::NONE, KeyCode::Down) => app.scroll_down(1, content_height),
        (KeyModifiers::NONE, KeyCode::PageUp) => app.scroll_up(10),
        (KeyModifiers::NONE, KeyCode::PageDown) => app.scroll_down(10, content_height),
        (KeyModifiers::NONE, KeyCode::Home) => app.scroll_offset = 0,
        (KeyModifiers::NONE, KeyCode::End) => app.scroll_to_bottom(),

        // Character input.
        (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => app.insert_char(c),

        _ => {}
    }
}

/// Handle mouse event.
pub fn handle_mouse(app: &mut App, event: crossterm::event::MouseEvent, content_height: u16) {
    match event.kind {
        MouseEventKind::ScrollUp => app.scroll_up(3),
        MouseEventKind::ScrollDown => app.scroll_down(3, content_height),
        _ => {}
    }
}

/// Handle terminal event.
pub fn handle_event(app: &mut App, event: &Event, content_height: u16) {
    match event {
        Event::Key(key) => handle_key(app, *key, content_height),
        Event::Mouse(mouse) => handle_mouse(app, *mouse, content_height),
        _ => {}
    }
}
