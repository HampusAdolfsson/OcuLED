use crate::components::Drawable;
use crate::rendering;
use super::TextWidget;
use super::simple_text::SimpleTextWidget;
use super::super::{ Widget, Bounds, Size };

const SCROLL_SPEED: f32 = 40.0; // text scrolling speed in pixels per second

/// Text that will periodically scroll to show all of its contents, if it does not fit within its bounds.
pub struct ScrollingTextWidget<'a> {
    state: ScrollingState,
    text: SimpleTextWidget<'a>,
    default_alignment: Alignment,
    wait_start: std::time::Duration,
    wait_end: std::time::Duration,
}

pub enum Alignment {
    Left,
    Center,
    Right,
}

impl<'a> ScrollingTextWidget<'a> {
    /// Creates a new scrolling text widget.
    ///
    /// `text` - The initial text to display
    /// `font` - The font to render
    /// `font_size` - The font size to render
    /// `default_alignment` - The alignment to use when the bounds are larger than the text
    /// `wait_start` - The time to after resetting until the text starts scrolling again
    /// `wait_end` - The time to after finishing scrolling until the text resets
    pub fn new(
        text: String,
        font: &'a fontdue::Font,
        font_size: f32,
        default_alignment: Alignment,
        wait_start: std::time::Duration,
        wait_end: std::time::Duration,
    ) -> Self {
        Self {
            state: ScrollingState::WaitingStart(wait_start),
            text: SimpleTextWidget::new(text, font, font_size),
            default_alignment,
            wait_start,
            wait_end,
        }
    }

    fn update(&mut self, elapsed: &std::time::Duration, widget_width: u32) {
        match self.state {
            ScrollingState::WaitingStart(remaining) => {
                let new_remaining = remaining.saturating_sub(*elapsed);
                if new_remaining > std::time::Duration::ZERO {
                    self.state = ScrollingState::WaitingStart(new_remaining);
                } else {
                    // technically we should care about the carry-over time here, but...
                    self.state = ScrollingState::Moving(0.0);
                }
            },
            ScrollingState::Moving(mut x_pos) => {
                x_pos -= SCROLL_SPEED * (elapsed.as_millis() as f32 / 1000.0);
                if (x_pos as i32 + self.text.size().width as i32) < widget_width as i32 {
                    self.state = ScrollingState::WaitingEnd(self.wait_end);
                } else {
                    self.state = ScrollingState::Moving(x_pos);
                }
            },
            ScrollingState::WaitingEnd(remaining) => {
                let new_remaining = remaining.saturating_sub(*elapsed);
                if new_remaining > std::time::Duration::ZERO {
                    self.state = ScrollingState::WaitingEnd(new_remaining);
                } else {
                    let carry_over = elapsed.saturating_sub(remaining);
                    self.state = ScrollingState::WaitingStart(self.wait_start - carry_over);
                }
            },
        }
    }
}

enum ScrollingState {
    WaitingStart(std::time::Duration),
    Moving(f32),
    WaitingEnd(std::time::Duration),
}

impl<'a> TextWidget<(), u32> for ScrollingTextWidget<'a> {
    fn set_text(&mut self, text: &str) -> bool {
        if self.text.set_text(text) {
            self.state = ScrollingState::WaitingStart(self.wait_start);
            return true;
        }
        false
    }
}

impl<'a> Drawable for ScrollingTextWidget<'a> {
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        self.update(elapsed, bounds.size.width);

        let text_size = self.text.size();
        let x: i32 = if bounds.size.width >= text_size.width {
            match self.default_alignment {
                Alignment::Left => bounds.pos.x,
                Alignment::Center => bounds.pos.x + (bounds.size.width - text_size.width) as i32 / 2,
                Alignment::Right => bounds.right() - text_size.width as i32
            }
        } else {
            match self.state {
                ScrollingState::WaitingStart(_) => {
                    bounds.pos.x
                },
                ScrollingState::Moving(x_delta) => {
                    bounds.pos.x + x_delta as i32
                },
                ScrollingState::WaitingEnd(_) => {
                    bounds.right() - text_size.width as i32
                },
            }
        };
        self.text.draw(canvas, bounds.with_x(x), elapsed);
    }
}

impl<'a> Widget<(), u32> for ScrollingTextWidget<'a> {
    fn size(&self) -> Size<(), u32> {
        Size {
            width: (),
            height: self.text.size().height,
        }
    }
}