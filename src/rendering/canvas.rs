
/**
 * A bitmap, but we can also draw text to it
 */
pub struct Canvas {
    pub bitmap: super::bitmap::Bitmap,
    pub font: &'static [u8],
}

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
 * Specifices how to interpret the x coordinate of a text draw call
 */
pub enum HorizontalAlignment {
    /** The x coordinate is where the leftmost pixels should be drawn */
    Left,
    /** The x coordinate is where the rightmost pixels should be drawn */
    Right,
    /** The x coordinate is the middle point between the leftmost and the rightmost pixels */
    Center,
}
/**
 * Specifices how to interpret the y coordinate of a text draw call
 */
pub enum VerticalAlignment {
    /** The y coordinate is where the baseline should be drawn */
    Baseline,
    /** The y coordinate is the middle point between the baseline and the top of the drawn text */
    CenterBase,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        return Canvas{
            bitmap: super::bitmap::Bitmap::new(width, height),
            font: &[],
        }
    }

    pub fn clear(&mut self) {
        self.bitmap.clear();
    }

    pub fn set_font(&mut self, font: &'static [u8]) {
        self.font = font;
    }

    /**
     * Draws the given text at the given position.
     */
    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, font_size: f32, hor_alignment: HorizontalAlignment, ver_alignment: VerticalAlignment) {
        assert_ne!(self.font.len(), 0);

        let font = fontdue::Font::from_bytes(self.font, fontdue::FontSettings::default()).unwrap();

        let text_metrics = self.measure_text(text, font_size);
        let mut next_x = match hor_alignment {
            HorizontalAlignment::Left => x as f32,
            HorizontalAlignment::Center => x as f32 - text_metrics.width as f32 / 2.0,
            HorizontalAlignment::Right => x as f32 - text_metrics.width as f32,
        };
        let baseline = match ver_alignment {
            VerticalAlignment::Baseline => y,
            VerticalAlignment::CenterBase => y + text_metrics.base_height as i32 / 2,
        };

        for character in text.chars() {
            let (metrics, bitmap) = font.rasterize(character, font_size);
            let padding = metrics.advance_width - metrics.width as f32;
            let top = baseline - metrics.height as i32 - metrics.ymin;
            self.bitmap.draw_bitmap((next_x + padding / 2.0) as i32, top, metrics.width, metrics.height, &bitmap);

            next_x += metrics.advance_width;
        }
    }

    /**
     * Measures the size of some text without rendering it. The metrics return
     * describe the size the text would have if rendered.
     */
    pub fn measure_text(&self, text: &str, font_size: f32) -> TextMetrics {
        let font = fontdue::Font::from_bytes(self.font, fontdue::FontSettings::default()).unwrap();

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
}
