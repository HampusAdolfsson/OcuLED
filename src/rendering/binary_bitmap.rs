/**
 * A bitmap with one bit per pixel.
 */
#[derive(Clone)]
pub struct BinaryBitmap {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
}

impl From<&super::bitmap::Bitmap> for BinaryBitmap {
    fn from(bitmap: &super::bitmap::Bitmap) -> Self {
        assert!(bitmap.buffer.len() % 8 == 0);

        let mut buffer = vec![0u8; bitmap.width*bitmap.height/8];
        for i in 0..buffer.len() {
            let mut byte = 0u8;
            for bit in 0..8 {
                if bitmap.buffer[8*i + bit] > 0x7f {
                    byte |= 1 << (7 - bit);
                }
            }
            buffer[i] = byte;
        }

        BinaryBitmap{
            width: bitmap.width,
            height: bitmap.height,
            buffer: buffer,
        }
    }
}
