//! Text component - Text rendering with styles

use crate::core::{Element, ElementType, Style, Color, TextWrap};

/// Text component builder
#[derive(Debug, Clone)]
pub struct Text {
    content: String,
    style: Style,
    key: Option<String>,
}

impl Text {
    /// Create a new Text with content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: Style::new(),
            key: None,
        }
    }

    /// Set key for reconciliation
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    // === Text styles ===

    /// Set text color
    pub fn color(mut self, color: Color) -> Self {
        self.style.color = Some(color);
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.style.background_color = Some(color);
        self
    }

    /// Alias for background
    pub fn bg(self, color: Color) -> Self {
        self.background(color)
    }

    /// Set bold
    pub fn bold(mut self) -> Self {
        self.style.bold = true;
        self
    }

    /// Set italic
    pub fn italic(mut self) -> Self {
        self.style.italic = true;
        self
    }

    /// Set underline
    pub fn underline(mut self) -> Self {
        self.style.underline = true;
        self
    }

    /// Set strikethrough
    pub fn strikethrough(mut self) -> Self {
        self.style.strikethrough = true;
        self
    }

    /// Set dim (less bright)
    pub fn dim(mut self) -> Self {
        self.style.dim = true;
        self
    }

    /// Set inverse (swap foreground and background)
    pub fn inverse(mut self) -> Self {
        self.style.inverse = true;
        self
    }

    /// Set text wrap behavior
    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.style.text_wrap = wrap;
        self
    }

    // === Convenience methods ===

    /// Apply error style (red color)
    pub fn error(self) -> Self {
        self.color(Color::Red)
    }

    /// Apply success style (green color)
    pub fn success(self) -> Self {
        self.color(Color::Green)
    }

    /// Apply warning style (yellow color)
    pub fn warning(self) -> Self {
        self.color(Color::Yellow)
    }

    /// Apply info style (blue color)
    pub fn info(self) -> Self {
        self.color(Color::Blue)
    }

    /// Apply muted style (dim)
    pub fn muted(self) -> Self {
        self.dim()
    }

    /// Convert to Element
    pub fn into_element(self) -> Element {
        let mut element = Element::new(ElementType::Text);
        element.style = self.style;
        element.text_content = Some(self.content);
        element.key = self.key;
        element
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_creation() {
        let element = Text::new("Hello").into_element();
        assert_eq!(element.get_text(), Some("Hello"));
    }

    #[test]
    fn test_text_styles() {
        let element = Text::new("Styled")
            .color(Color::Green)
            .bold()
            .underline()
            .into_element();

        assert_eq!(element.style.color, Some(Color::Green));
        assert!(element.style.bold);
        assert!(element.style.underline);
    }

    #[test]
    fn test_text_convenience_methods() {
        let error = Text::new("Error").error().into_element();
        assert_eq!(error.style.color, Some(Color::Red));

        let success = Text::new("Success").success().into_element();
        assert_eq!(success.style.color, Some(Color::Green));
    }
}
