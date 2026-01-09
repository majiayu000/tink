//! Text measurement utilities

use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;

/// Measure the display width of text (accounting for Unicode)
pub fn measure_text_width(text: &str) -> usize {
    text.width()
}

/// Measure text dimensions (width, height)
pub fn measure_text(text: &str) -> (usize, usize) {
    let lines: Vec<&str> = text.lines().collect();
    let height = lines.len().max(1);
    let width = lines.iter().map(|line| line.width()).max().unwrap_or(0);
    (width, height)
}

/// Wrap text to fit within a maximum width
pub fn wrap_text(text: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }

    let mut result = String::new();
    let mut current_width = 0;

    for ch in text.chars() {
        let char_width = ch.width().unwrap_or(0);

        if ch == '\n' {
            result.push(ch);
            current_width = 0;
        } else if current_width + char_width > max_width {
            result.push('\n');
            result.push(ch);
            current_width = char_width;
        } else {
            result.push(ch);
            current_width += char_width;
        }
    }

    result
}

/// Truncate text to fit within a maximum width
pub fn truncate_text(text: &str, max_width: usize, ellipsis: &str) -> String {
    let text_width = text.width();

    if text_width <= max_width {
        return text.to_string();
    }

    let ellipsis_width = ellipsis.width();
    if max_width <= ellipsis_width {
        return ellipsis.chars().take(max_width).collect();
    }

    let target_width = max_width - ellipsis_width;
    let mut result = String::new();
    let mut current_width = 0;

    for ch in text.chars() {
        let char_width = ch.width().unwrap_or(0);
        if current_width + char_width > target_width {
            break;
        }
        result.push(ch);
        current_width += char_width;
    }

    result.push_str(ellipsis);
    result
}

/// Truncate text from the start
pub fn truncate_start(text: &str, max_width: usize, ellipsis: &str) -> String {
    let text_width = text.width();

    if text_width <= max_width {
        return text.to_string();
    }

    let ellipsis_width = ellipsis.width();
    if max_width <= ellipsis_width {
        return ellipsis.chars().take(max_width).collect();
    }

    let target_width = max_width - ellipsis_width;
    let mut result = String::new();
    let mut chars: Vec<char> = text.chars().collect();
    chars.reverse();

    let mut current_width = 0;
    let mut end_chars = Vec::new();

    for ch in chars {
        let char_width = ch.width().unwrap_or(0);
        if current_width + char_width > target_width {
            break;
        }
        end_chars.push(ch);
        current_width += char_width;
    }

    end_chars.reverse();
    result.push_str(ellipsis);
    result.extend(end_chars);
    result
}

/// Truncate text from the middle
pub fn truncate_middle(text: &str, max_width: usize, ellipsis: &str) -> String {
    let text_width = text.width();

    if text_width <= max_width {
        return text.to_string();
    }

    let ellipsis_width = ellipsis.width();
    if max_width <= ellipsis_width {
        return ellipsis.chars().take(max_width).collect();
    }

    let available = max_width - ellipsis_width;
    let left_width = available / 2;
    let right_width = available - left_width;

    let mut left = String::new();
    let mut current_width = 0;
    for ch in text.chars() {
        let char_width = ch.width().unwrap_or(0);
        if current_width + char_width > left_width {
            break;
        }
        left.push(ch);
        current_width += char_width;
    }

    let mut right = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut right_chars = Vec::new();
    current_width = 0;

    for ch in chars.iter().rev() {
        let char_width = ch.width().unwrap_or(0);
        if current_width + char_width > right_width {
            break;
        }
        right_chars.push(*ch);
        current_width += char_width;
    }

    right_chars.reverse();
    right.extend(right_chars);

    format!("{}{}{}", left, ellipsis, right)
}

/// Pad text to a specific width
pub fn pad_text(text: &str, width: usize, align: TextAlign) -> String {
    let text_width = text.width();

    if text_width >= width {
        return text.to_string();
    }

    let padding = width - text_width;

    match align {
        TextAlign::Left => format!("{}{}", text, " ".repeat(padding)),
        TextAlign::Right => format!("{}{}", " ".repeat(padding), text),
        TextAlign::Center => {
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
        }
    }
}

/// Text alignment
#[derive(Debug, Clone, Copy, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Right,
    Center,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_ascii() {
        assert_eq!(measure_text_width("hello"), 5);
        assert_eq!(measure_text_width("hello world"), 11);
    }

    #[test]
    fn test_measure_unicode() {
        // Chinese characters are typically 2 cells wide
        assert_eq!(measure_text_width("你好"), 4);
        assert_eq!(measure_text_width("Hello 世界"), 10);
    }

    #[test]
    fn test_measure_text_dimensions() {
        let (w, h) = measure_text("hello\nworld");
        assert_eq!(w, 5);
        assert_eq!(h, 2);
    }

    #[test]
    fn test_wrap_text() {
        let wrapped = wrap_text("hello world", 6);
        assert!(wrapped.contains('\n'));
    }

    #[test]
    fn test_truncate_text() {
        let truncated = truncate_text("hello world", 8, "...");
        assert_eq!(truncated, "hello...");
    }

    #[test]
    fn test_truncate_start() {
        let truncated = truncate_start("hello world", 8, "...");
        assert_eq!(truncated, "...world");
    }

    #[test]
    fn test_truncate_middle() {
        let truncated = truncate_middle("hello world", 9, "...");
        assert_eq!(truncated, "hel...rld");
    }

    #[test]
    fn test_pad_text() {
        assert_eq!(pad_text("hi", 5, TextAlign::Left), "hi   ");
        assert_eq!(pad_text("hi", 5, TextAlign::Right), "   hi");
        assert_eq!(pad_text("hi", 5, TextAlign::Center), " hi  ");
    }
}
