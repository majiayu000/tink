//! Output buffer for terminal rendering

use crate::core::{Color, Style};
use std::fmt::Write as FmtWrite;

/// A styled character in the output grid
#[derive(Debug, Clone, Default)]
pub struct StyledChar {
    pub ch: char,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub dim: bool,
    pub inverse: bool,
}

impl StyledChar {
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            ..Default::default()
        }
    }

    pub fn with_style(ch: char, style: &Style) -> Self {
        Self {
            ch,
            fg: style.color,
            bg: style.background_color,
            bold: style.bold,
            italic: style.italic,
            underline: style.underline,
            strikethrough: style.strikethrough,
            dim: style.dim,
            inverse: style.inverse,
        }
    }

    /// Check if this char has any styling
    pub fn has_style(&self) -> bool {
        self.fg.is_some()
            || self.bg.is_some()
            || self.bold
            || self.italic
            || self.underline
            || self.strikethrough
            || self.dim
            || self.inverse
    }

    /// Check if two styled chars have the same style
    pub fn same_style(&self, other: &Self) -> bool {
        self.fg == other.fg
            && self.bg == other.bg
            && self.bold == other.bold
            && self.italic == other.italic
            && self.underline == other.underline
            && self.strikethrough == other.strikethrough
            && self.dim == other.dim
            && self.inverse == other.inverse
    }
}

/// Clip region for overflow handling
#[derive(Debug, Clone)]
pub struct ClipRegion {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
}

impl ClipRegion {
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x1 && x < self.x2 && y >= self.y1 && y < self.y2
    }
}

/// Output buffer that collects rendered content
pub struct Output {
    pub width: u16,
    pub height: u16,
    grid: Vec<Vec<StyledChar>>,
    clip_stack: Vec<ClipRegion>,
}

impl Output {
    /// Create a new output buffer
    pub fn new(width: u16, height: u16) -> Self {
        let grid = vec![vec![StyledChar::new(' '); width as usize]; height as usize];
        Self {
            width,
            height,
            grid,
            clip_stack: Vec::new(),
        }
    }

    /// Write text at position with style
    pub fn write(&mut self, x: u16, y: u16, text: &str, style: &Style) {
        let mut col = x as usize;
        let row = y as usize;

        if row >= self.grid.len() {
            return;
        }

        for ch in text.chars() {
            if ch == '\n' {
                break;
            }

            if col >= self.grid[row].len() {
                break;
            }

            // Check clip region
            if let Some(clip) = self.clip_stack.last() {
                if !clip.contains(col as u16, row as u16) {
                    col += 1;
                    continue;
                }
            }

            self.grid[row][col] = StyledChar::with_style(ch, style);
            col += 1;
        }
    }

    /// Write a single character at position
    pub fn write_char(&mut self, x: u16, y: u16, ch: char, style: &Style) {
        let col = x as usize;
        let row = y as usize;

        if row >= self.grid.len() || col >= self.grid[row].len() {
            return;
        }

        // Check clip region
        if let Some(clip) = self.clip_stack.last() {
            if !clip.contains(x, y) {
                return;
            }
        }

        self.grid[row][col] = StyledChar::with_style(ch, style);
    }

    /// Fill a rectangle with a character
    pub fn fill_rect(&mut self, x: u16, y: u16, width: u16, height: u16, ch: char, style: &Style) {
        for row in y..(y + height).min(self.height) {
            for col in x..(x + width).min(self.width) {
                self.write_char(col, row, ch, style);
            }
        }
    }

    /// Push a clip region
    pub fn clip(&mut self, region: ClipRegion) {
        self.clip_stack.push(region);
    }

    /// Pop the current clip region
    pub fn unclip(&mut self) {
        self.clip_stack.pop();
    }

    /// Convert the buffer to a string with ANSI codes
    pub fn render(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        for row in self.grid.iter() {
            let mut line = String::new();
            let mut current_style: Option<StyledChar> = None;

            for cell in row {
                // Check if we need to change style
                let need_style_change = match &current_style {
                    None => cell.has_style(),
                    Some(prev) => !cell.same_style(prev),
                };

                if need_style_change {
                    // Reset and apply new style
                    line.push_str("\x1b[0m");
                    self.apply_style(&mut line, cell);
                    current_style = Some(cell.clone());
                }

                line.push(cell.ch);
            }

            // Reset at end of line
            if current_style.is_some() {
                line.push_str("\x1b[0m");
            }

            // Trim trailing whitespace (like ink does)
            let trimmed = line.trim_end();
            lines.push(trimmed.to_string());
        }

        // Remove trailing empty lines
        while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
            lines.pop();
        }

        lines.join("\n")
    }

    fn apply_style(&self, result: &mut String, cell: &StyledChar) {
        let mut codes: Vec<u8> = Vec::new();

        if cell.bold {
            codes.push(1);
        }
        if cell.dim {
            codes.push(2);
        }
        if cell.italic {
            codes.push(3);
        }
        if cell.underline {
            codes.push(4);
        }
        if cell.inverse {
            codes.push(7);
        }
        if cell.strikethrough {
            codes.push(9);
        }

        if let Some(fg) = cell.fg {
            self.color_to_ansi(fg, false, &mut codes);
        }

        if let Some(bg) = cell.bg {
            self.color_to_ansi(bg, true, &mut codes);
        }

        if !codes.is_empty() {
            result.push_str("\x1b[");
            for (i, code) in codes.iter().enumerate() {
                if i > 0 {
                    result.push(';');
                }
                let _ = write!(result, "{}", code);
            }
            result.push('m');
        }
    }

    fn color_to_ansi(&self, color: Color, background: bool, codes: &mut Vec<u8>) {
        let base = if background { 40 } else { 30 };

        match color {
            Color::Reset => {}
            Color::Black => codes.push(base),
            Color::Red => codes.push(base + 1),
            Color::Green => codes.push(base + 2),
            Color::Yellow => codes.push(base + 3),
            Color::Blue => codes.push(base + 4),
            Color::Magenta => codes.push(base + 5),
            Color::Cyan => codes.push(base + 6),
            Color::White => codes.push(base + 7),
            Color::BrightBlack => codes.push(base + 60),
            Color::BrightRed => codes.push(base + 61),
            Color::BrightGreen => codes.push(base + 62),
            Color::BrightYellow => codes.push(base + 63),
            Color::BrightBlue => codes.push(base + 64),
            Color::BrightMagenta => codes.push(base + 65),
            Color::BrightCyan => codes.push(base + 66),
            Color::BrightWhite => codes.push(base + 67),
            Color::Ansi256(n) => {
                codes.push(if background { 48 } else { 38 });
                codes.push(5);
                codes.push(n);
            }
            Color::Rgb(r, g, b) => {
                codes.push(if background { 48 } else { 38 });
                codes.push(2);
                codes.push(r);
                codes.push(g);
                codes.push(b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_creation() {
        let output = Output::new(80, 24);
        assert_eq!(output.width, 80);
        assert_eq!(output.height, 24);
    }

    #[test]
    fn test_write_text() {
        let mut output = Output::new(80, 24);
        output.write(0, 0, "Hello", &Style::default());

        assert_eq!(output.grid[0][0].ch, 'H');
        assert_eq!(output.grid[0][4].ch, 'o');
    }

    #[test]
    fn test_styled_output() {
        let mut output = Output::new(80, 24);
        let mut style = Style::default();
        style.color = Some(Color::Green);
        style.bold = true;

        output.write(0, 0, "Test", &style);

        let rendered = output.render();
        assert!(rendered.contains("\x1b["));
    }
}
