use image::GenericImageView;

/// A monochrome buffer we can draw to. Pixels are 1 byte each.
#[derive(Clone)]
pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Self {
        return Bitmap{
            width,
            height,
            buffer: vec![0u8; width * height],
        }
    }

    pub fn from_raw_bytes(bytes: &[u8], width: usize) -> Self {
        return Bitmap {
            width,
            height: bytes.len() / width,
            buffer: bytes.to_vec(),
        }
    }

    /// Decodes bytes in png format (e.g. read from a png file) into a bitmap
    pub fn from_png(bytes: &[u8]) -> Self {
        Self::from_png_with_scale(bytes, 1.0)
    }
    /// Decodes bytes in png format (e.g. read from a png file) into a bitmap
    pub fn from_png_with_scale(bytes: &[u8], scale: f32) -> Self {
        let scaled_img = {
            let cursor = std::io::Cursor::new(bytes);
            let img = image::io::Reader::new(cursor).with_guessed_format().unwrap().decode().unwrap();
            let gray = image::imageops::grayscale(&img);
            let dimensions = img.dimensions();
            let (w, h) = (dimensions.0 as f32 * scale, dimensions.1 as f32 * scale);
            image::imageops::resize(&gray, w as u32, h as u32, image::imageops::FilterType::Nearest)
        };
        let dimensions = scaled_img.dimensions();
        let buf = scaled_img.into_vec();

        Bitmap {
            width: dimensions.0 as usize,
            height: dimensions.1 as usize,
            buffer: buf,
        }
    }

    pub fn from_text(text: &str, font_size: f32, font: &fontdue::Font) -> Self {
        let text_metrics = measure_text(text, font, font_size);
        let baseline = text_metrics.base_height as i32;
        let mut next_x = 0.0;

        let mut bmp = Bitmap::new(text_metrics.width, text_metrics.height);
        for character in text.chars() {
            let (metrics, buffer) = font.rasterize(character, font_size);

            let padding = metrics.advance_width - metrics.width as f32;
            let top = baseline - metrics.height as i32 - metrics.ymin;
            let char_bmp = Bitmap{ width: metrics.width, height: metrics.height, buffer: buffer};
            bmp.draw_bitmap((next_x + padding / 2.0) as i32, top, &char_bmp);

            next_x += metrics.advance_width;
        }
        bmp
   }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn draw_bitmap(&mut self, x: i32, y: i32, bitmap: &Bitmap) {
        // could use memcpy or vectorization for better performance
        for bmp_y in 0..bitmap.height {
            let actual_y = y + bmp_y as i32;
            if actual_y < 0 { continue; }
            if actual_y as usize >= self.height { return; }
            for bmp_x in 0..bitmap.width {
                let actual_x = x + bmp_x as i32;
                if actual_x < 0 { continue; }
                if actual_x as usize >= self.width { break; }

                self.buffer[(actual_y * self.width as i32 + actual_x) as usize] |= bitmap.buffer[bmp_x + bmp_y * bitmap.width];
            }
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: usize, height: usize) {
        let x_start = x.max(0).min(self.width as i32) as usize;
        let x_end = (x + width as i32).max(0).min(self.width as i32) as usize;
        let y_start = y.max(0).min(self.height as i32) as usize;
        let y_end = (y + height as i32).max(0).min(self.height as i32) as usize;

        for y_draw in y_start..y_end {
            for x_draw in x_start..x_end {
                self.buffer[y_draw * self.width + x_draw] = 0xff;
            }
        }
    }
    pub fn draw_rect_with_slits(&mut self, x: i32, y: i32, width: usize, height: usize, slit_interval: usize) {
        let x_start = x.max(0).min(self.width as i32) as usize;
        let x_end = (x + width as i32).max(0).min(self.width as i32) as usize;
        let y_start = y.max(0).min(self.height as i32) as usize;
        let y_end = (y + height as i32).max(0).min(self.height as i32) as usize;

        for y_draw in y_start..y_end {
            for x_draw in x_start..x_end {
                if y_draw % slit_interval == x_draw % slit_interval { continue; }
                self.buffer[y_draw * self.width + x_draw] = 0xff;
            }
        }
    }
}

/// Describes the size of a string of text for some font and font size
struct TextMetrics {
    pub width: usize,
    pub height: usize,
    /* The distance from the top of the text to the baseline */
    pub base_height: usize,
}

/// Measures the size of some text without rendering it. The metrics return
/// describe the size the text would have if rendered.
fn measure_text(text: &str, font: &fontdue::Font, font_size: f32) -> TextMetrics {
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