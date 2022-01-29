
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

    pub fn clear(&mut self) {
        self.buffer.fill(0);
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
