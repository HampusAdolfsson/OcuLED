use super::bitmap::Bitmap;

/**
 * A bitmap, but we can also draw text to it
 */
#[derive(Clone)]
pub struct Canvas {
    pub bitmap: Bitmap,
    pub font: fontdue::Font,
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
    /** The y coordinate is the middle point between the baseline and the topmost pixel */
    CenterBase,
    /** The y coordinate is where the topmost pixel should be drawn */
    Top,
    /** The y coordinate is where the bottommost pixel should be drawn */
    Bottom,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        return Canvas{
            bitmap: Bitmap::new(width, height),
            font: fontdue::Font::from_bytes(include_bytes!("../../resources/fonts/Roboto-Bold.ttf").as_slice(), fontdue::FontSettings::default()).unwrap(),
        }
    }

    pub fn clear(&mut self) {
        self.bitmap.clear();
    }

    pub fn set_font(&mut self, font: fontdue::Font) {
        self.font = font;
    }
    // This is expensive, so don't call this every frame
    pub fn set_font_from_bytes(&mut self, font: &[u8]) {
        self.font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
    }

    pub fn get_font(&self) -> &fontdue::Font {
        &self.font
    }

    /**
     * Draws the given text at the given position.
     */
    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, font_size: f32, hor_alignment: HorizontalAlignment, ver_alignment: VerticalAlignment) {
        let text_metrics = super::measure_text(text, &self.font, font_size);
        let mut next_x = match hor_alignment {
            HorizontalAlignment::Left => x as f32,
            HorizontalAlignment::Center => x as f32 - text_metrics.width as f32 / 2.0,
            HorizontalAlignment::Right => x as f32 - text_metrics.width as f32,
        };
        let baseline = match ver_alignment {
            VerticalAlignment::Baseline => y,
            VerticalAlignment::CenterBase => y + text_metrics.base_height as i32 / 2,
            VerticalAlignment::Top => y + text_metrics.base_height as i32,
            VerticalAlignment::Bottom => y - (text_metrics.height as i32 - text_metrics.base_height as i32),
        };

        for character in text.chars() {
            let (metrics, buffer) = self.font.rasterize(character, font_size);

            let padding = metrics.advance_width - metrics.width as f32;
            let top = baseline - metrics.height as i32 - metrics.ymin;
            let char_bmp = Bitmap{ width: metrics.width, height: metrics.height, buffer: buffer};
            self.bitmap.draw_bitmap((next_x + padding / 2.0) as i32, top, &char_bmp);

            next_x += metrics.advance_width;
        }
    }

    /**
     * Measures the size of some text without rendering it. The metrics return
     * describe the size the text would have if rendered.
     */
    pub fn measure_text(&self, text: &str, font_size: f32) -> TextMetrics {
        let mut x = 0f32;
        let mut base_height = 0i32;
        let mut bottom = 0i32;

        for character in text.chars() {
            let metrics = self.font.metrics(character, font_size);
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
