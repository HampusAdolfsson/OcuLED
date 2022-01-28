
/**
 * A monochrome buffer we can draw to. Pixels are 1 byte each.
 */
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
    pub font: &'static [u8],
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        return Canvas{
            width,
            height,
            buffer: vec![0u8; width * height],
            font: &[],
        }
    }

    pub fn set_font(&mut self, font: &'static [u8]) {
        self.font = font;
    }

    pub fn draw_text(&mut self, x: i32, baseline: i32, text: &str, font_size: f32) {
        assert_ne!(self.font.len(), 0);

        let font = fontdue::Font::from_bytes(self.font, fontdue::FontSettings::default()).unwrap();

        let mut float_x = x as f32;

        for character in text.chars() {
            let (metrics, bitmap) = font.rasterize(character, font_size);
            let padding = metrics.advance_width - metrics.width as f32;
            self.draw_bitmap((float_x + padding / 2.0) as i32, baseline - metrics.height as i32 - metrics.ymin, metrics.width, metrics.height, &bitmap);

            float_x += metrics.advance_width;
        }
    }

    pub fn draw_bitmap(&mut self, x: i32, y: i32, width: usize, height: usize, bitmap: &[u8]) {
        // could use memcpy or vectorization for better performance
        for bmp_y in 0..height {
            let actual_y = y + bmp_y as i32;
            if actual_y < 0 { continue; }
            if actual_y as usize >= self.height { return; }
            for bmp_x in 0..width {
                let actual_x = x + bmp_x as i32;
                if actual_x < 0 { continue; }
                if actual_x as usize >= self.width { break; }

                self.buffer[(actual_y * self.width as i32 + actual_x) as usize] = bitmap[bmp_x + bmp_y * width];
            }
        }
    }
}

/**
 * A canvas with one bit per pixel.
 */
pub struct BinaryCanvas {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
}

impl From<Canvas> for BinaryCanvas {
    fn from(canvas: Canvas) -> Self {
        assert!(canvas.buffer.len() % 8 == 0);

        let mut buffer = vec![0u8; canvas.width*canvas.height/8];
        for i in 0..buffer.len() {
            let mut byte = 0u8;
            for bit in 0..8 {
                if canvas.buffer[8*i + bit] > 0x7f {
                    byte |= 1 << (7 - bit);
                }
            }
            buffer[i] = byte;
        }

        BinaryCanvas{
            width: canvas.width,
            height: canvas.height,
            buffer: buffer,
        }
    }
}