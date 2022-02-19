mod bitmap;
mod canvas;
mod binary_bitmap;

pub use canvas::Canvas;
pub use canvas::HorizontalAlignment;
pub use canvas::VerticalAlignment;
pub use bitmap::Bitmap;
pub use binary_bitmap::BinaryBitmap;

/**
 * Describes the size of a string of text for some font and font size
 */
pub struct TextMetrics {
    pub width: usize,
    pub height: usize,
    /* The distance from the top of the text to the baseline */
    pub base_height: usize,
}

/**
 * Measures the size of some text without rendering it. The metrics return
 * describe the size the text would have if rendered.
 */
pub fn measure_text(text: &str, font: &fontdue::Font, font_size: f32) -> TextMetrics {
    let mut x = 0f32;
    let mut base_height = 0i32;
    let mut bottom = 0i32;

    for character in text.chars() {
        let metrics = font.metrics(character, font_size);
        let top = -(metrics.height as i32) - metrics.ymin;
        if top < base_height {
            base_height = top;
        }
        if -metrics.ymin > bottom {
            bottom = -metrics.ymin;
        }

        x += metrics.advance_width;
    }
    return TextMetrics {
        width: x as usize,
        height: (bottom - base_height) as usize,
        base_height: (-base_height) as usize,
    };
}