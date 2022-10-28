use image::{AnimationDecoder, EncodableLayout, ImageResult, RgbImage, RgbaImage};

use super::Bitmap;


pub struct Video {
    frames: Vec<super::Bitmap>,
    pos: usize,
}

impl Video {
    pub fn from_gif(bytes: &[u8], invert: bool) -> ImageResult<Self> {
        let cursor = std::io::Cursor::new(bytes);
        let decoder = image::codecs::gif::GifDecoder::new(cursor).unwrap();
        let frames = decoder.into_frames().collect_frames()?;
        let bitmaps = frames.into_iter().map(|f| {
            let mut gray = image::imageops::grayscale(f.buffer());
            if invert {
                image::imageops::invert(&mut gray);
            }
            gray = image::imageops::contrast(&gray, 40.0);
            Bitmap::from_raw_bytes(gray.as_bytes(), gray.width() as usize)
        }).collect::<Vec<Bitmap>>();
        assert!(bitmaps.len() > 0);
        Ok(Self {
            frames: bitmaps,
            pos: 0,
        })
    }
    pub fn from_images(images: &[RgbaImage]) -> Self {
        let frames = images.iter().map(|img| {
            let gray = image::imageops::grayscale(img);
            Bitmap::from_raw_bytes(gray.as_bytes(), gray.width() as usize)
        });
        Self {
            frames: frames.collect(),
            pos: 0
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        self.pos = self.pos % self.frames.len();
    }
    pub fn reset(&mut self) {
        self.pos = 0;
    }

    pub fn current_frame(&self) -> &Bitmap {
        &self.frames[self.pos]
    }
}
