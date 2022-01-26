
/**
 * A buffer we can draw to. Pixels are 1 byte each.
 */
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        return Canvas{
            width,
            height,
            buffer: vec![0u8; width * height],
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
                    byte |= 1 << bit;
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