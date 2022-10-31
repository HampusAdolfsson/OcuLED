mod simple_text;
mod scrolling_text;

pub use simple_text::*;
pub use scrolling_text::*;

use super::Widget;

pub trait TextWidget<W: Copy, H: Copy> : Widget<W, H> {
    /// Set the text to be displayed.
    ///
    /// The text will be drawn on the next [super::Widget::draw] call.
    /// Returns `true` if the new text was different from the previous one.
    fn set_text(&mut self, text: &str) -> bool;
}
