use image::ImageDecoder;

/**
 * A monochrome buffer we can draw to. Pixels are 1 byte each.
 */
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

    /** Decodes bytes in monochrome png format (e.g. read from a png file) into a bitmap */
    pub fn from_png(bytes: &[u8]) -> Self {
        let cursor = std::io::Cursor::new(bytes);
        let decoder = image::codecs::png::PngDecoder::new(cursor).unwrap();

        if decoder.color_type() != image::ColorType::L8 {
            panic!("Attempted to load non-grayscale image (color type was {:?}).", decoder.color_type());
        }

        let mut buf = vec![0u8; decoder.total_bytes() as usize];
        let dimensions = decoder.dimensions();
        decoder.read_image(&mut buf[..]).unwrap();

        Bitmap {
            width: dimensions.0 as usize,
            height: dimensions.1 as usize,
            buffer: buf,
        }
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

                self.buffer[(actual_y * self.width as i32 + actual_x) as usize] = bitmap.buffer[bmp_x + bmp_y * bitmap.width];
            }
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: usize, height: usize) {
        let y_start = std::cmp::max(y, 0) as usize;
        let y_end = std::cmp::min((y + height as i32) as usize, self.height);
        let x_start = std::cmp::max(x, 0) as usize;
        let x_end = std::cmp::min((x + width as i32) as usize, self.width);

        for y_draw in y_start..y_end {
            for x_draw in x_start..x_end {
                self.buffer[y_draw * self.width + x_draw] = 0xff;
            }
        }
    }
}
