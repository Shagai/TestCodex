//! Utilities for working with the `ratatui` crate.
//!
//! The library is intentionally tiny and only exposes one helper
//! [`build_paragraph`] which is used by the example program. It shows how to
//! create a `Paragraph` widget with a title and border.

// Re-export widgets we'll use from Ratatui.  Keeping the list small makes the
// example code easy to read and understand.
use ratatui::widgets::{Block, Borders, Paragraph};

//------------------------------------------------------------------------------
// Public API
//------------------------------------------------------------------------------

/// Build a [`Paragraph`] widget showing the provided text surrounded by a
/// titled border.
///
/// # Examples
///
/// ```
/// use testcodex::build_paragraph;
/// let _ = build_paragraph("Hello");
/// ```
pub fn build_paragraph<'a>(text: &'a str) -> Paragraph<'a> {
    Paragraph::new(text).block(Block::default().title("Example").borders(Borders::ALL))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::widgets::Paragraph;

    #[test]
    fn paragraph_type_check() {
        // The widget returned by `build_paragraph` should be exactly a
        // `Paragraph`.  This assignment will fail to compile if that ever
        // changes, providing a simple type-level check.
        let widget = build_paragraph("sample");
        let _: Paragraph = widget;
    }
}
