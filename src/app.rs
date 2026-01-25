//! Application state.

/// Application state.
#[derive(Debug)]
pub struct App {
    /// User input buffer.
    pub input: String,
    /// Cursor position (byte offset).
    pub cursor: usize,
    /// Output buffer.
    pub output: Vec<String>,
    /// Scroll offset (line-based).
    pub scroll_offset: u16,
    /// Auto-scroll enabled.
    pub auto_scroll: bool,
    /// Application running.
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            output: vec!["Welcome to {{project-name}}!".to_string(), String::new()],
            scroll_offset: 0,
            auto_scroll: true,
            running: true,
        }
    }
}

impl App {
    /// Insert a character at cursor position.
    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    /// Delete character before cursor.
    pub fn delete_char(&mut self) {
        if self.cursor > 0 {
            let prev = self.input[..self.cursor]
                .char_indices()
                .next_back()
                .map_or(0, |(i, _)| i);
            self.input.drain(prev..self.cursor);
            self.cursor = prev;
        }
    }

    /// Delete character at cursor.
    pub fn delete_char_forward(&mut self) {
        if self.cursor < self.input.len() {
            let next = self.input[self.cursor..]
                .char_indices()
                .nth(1)
                .map_or(self.input.len(), |(i, _)| self.cursor + i);
            self.input.drain(self.cursor..next);
        }
    }

    /// Move cursor left.
    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.input[..self.cursor]
                .char_indices()
                .next_back()
                .map_or(0, |(i, _)| i);
        }
    }

    /// Move cursor right.
    pub fn move_right(&mut self) {
        if self.cursor < self.input.len() {
            self.cursor = self.input[self.cursor..]
                .char_indices()
                .nth(1)
                .map_or(self.input.len(), |(i, _)| self.cursor + i);
        }
    }

    /// Move cursor to start.
    pub const fn move_start(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end.
    pub fn move_end(&mut self) {
        self.cursor = self.input.len();
    }

    /// Delete from cursor to start.
    pub fn delete_to_start(&mut self) {
        self.input.drain(..self.cursor);
        self.cursor = 0;
    }

    /// Delete from cursor to end.
    pub fn delete_to_end(&mut self) {
        self.input.drain(self.cursor..);
    }

    /// Delete word before cursor.
    pub fn delete_word(&mut self) {
        if self.cursor == 0 {
            return;
        }

        // Skip trailing whitespace.
        let mut end = self.cursor;
        while end > 0 {
            let prev = self.input[..end].char_indices().next_back();
            if let Some((i, c)) = prev {
                if c.is_whitespace() {
                    end = i;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Delete word.
        let mut start = end;
        while start > 0 {
            let prev = self.input[..start].char_indices().next_back();
            if let Some((i, c)) = prev {
                if c.is_whitespace() {
                    break;
                }
                start = i;
            } else {
                break;
            }
        }

        self.input.drain(start..self.cursor);
        self.cursor = start;
    }

    /// Clear input.
    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor = 0;
    }

    /// Submit input.
    pub fn submit(&mut self) {
        let input = std::mem::take(&mut self.input);
        self.cursor = 0;

        if input.is_empty() {
            return;
        }

        self.output.push(format!("> {input}"));
        self.output.push(format!("You typed: {input}"));
        self.output.push(String::new());

        if self.auto_scroll {
            self.scroll_to_bottom();
        }
    }

    /// Scroll up.
    pub const fn scroll_up(&mut self, amount: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
        self.auto_scroll = false;
    }

    /// Scroll down.
    pub fn scroll_down(&mut self, amount: u16, max: u16) {
        self.scroll_offset = self.scroll_offset.saturating_add(amount).min(max);
        if self.scroll_offset >= max {
            self.auto_scroll = true;
        }
    }

    /// Scroll to bottom.
    pub const fn scroll_to_bottom(&mut self) {
        self.scroll_offset = u16::MAX;
        self.auto_scroll = true;
    }

    /// Quit application.
    pub const fn quit(&mut self) {
        self.running = false;
    }
}
