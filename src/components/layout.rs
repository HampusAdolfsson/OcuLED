use crate::rendering::Bitmap;


#[derive(Clone, Copy)]
pub struct Size<W: Copy, H: Copy> {
    pub width: W,
    pub height: H,
}

#[derive(Clone, Copy)]
pub struct Point<X: Copy, Y: Copy> {
    pub x: X,
    pub y: Y,
}

#[derive(Clone, Copy)]
pub struct Rect<X: Copy, Y: Copy, W: Copy, H: Copy> {
    pub pos: Point<X, Y>,
    pub size: Size<W, H>,
}

pub type Bounds = Rect<i32, i32, u32, u32>;
pub type EmptyBounds = Rect<(), (), (), ()>;

impl EmptyBounds {
    pub fn new() -> Self {
        EmptyBounds { pos: Point{ x: (), y: () }, size: Size { width: (), height: () }}
    }
}

impl Bounds {
    pub fn cover_bitmap(bitmap: &Bitmap) -> Self {
        Bounds {
            pos: Point {
                x: 0,
                y: 0,
            },
            size: Size {
                width: bitmap.width.try_into().unwrap(),
                height: bitmap.height.try_into().unwrap(),
            },
        }
    }
}

// From x
impl<Y: Copy, W: Copy, H: Copy> Rect<i32, Y, W, H> {
    pub fn move_x(&self, by: i32) -> Rect<i32, Y, W, H> {
        Rect {
            pos: Point {
                x: self.pos.x + by,
                y: self.pos.y,
            },
            size: self.size,
        }
    }
}
// From y
impl<X: Copy, W: Copy, H: Copy> Rect<X, i32, W, H> {
    pub fn move_y(&self, by: i32) -> Rect<X, i32, W, H> {
        Rect {
            pos: Point {
                x: self.pos.x,
                y: self.pos.y + by,
            },
            size: self.size,
        }
    }
}

// From width
impl<X: Copy, Y: Copy, H: Copy> Rect<X, Y, u32, H> {
    /// Aligns the right edge of `self` with the left edge of `other`.
    pub fn left_of<Y2: Copy, W2: Copy, H2: Copy>(&self, other: &Rect<i32, Y2, W2, H2>) -> Rect<i32, Y, u32, H> {
        Rect {
            pos: Point {
                x: other.pos.x - self.size.width as i32,
                y: self.pos.y,
            },
            size: self.size,
        }
    }
    /// Aligns the left edge of `self` with the right edge of `other`.
    pub fn right_of<Y2: Copy, W2: Copy, H2: Copy>(&self, other: &Rect<i32, Y2, W2, H2>) -> Rect<i32, Y, u32, H> {
        Rect {
            pos: Point {
                x: other.pos.x - self.size.width as i32,
                y: self.pos.y,
            },
            size: self.size,
        }
    }
    /// Aligns the left edge of `self` with the left edge of `other`.
    pub fn align_right<Y2: Copy, H2: Copy>(&self, other: &Rect<i32, Y2, u32, H2>) -> Rect<i32, Y, u32, H> {
        Rect {
            pos: Point {
                x: other.right() - self.size.width as i32,
                y: self.pos.y,
            },
            size: self.size,
        }
    }
    /// Positions `self` in the horizontal center of `other`.
    pub fn center_hor_in<Y2: Copy, H2: Copy>(&self, other: &Rect<i32, Y2, u32, H2>) -> Rect<i32, Y, u32, H> {
        Rect {
            pos: Point {
                x: other.pos.x + (other.size.width - self.size.width) as i32 / 2,
                y: self.pos.y,
            },
            size: self.size,
        }
    }
}

// From height
impl<X: Copy, Y: Copy, W: Copy> Rect<X, Y, W, u32> {
    /// Aligns the bottom edge of `self` with the top edge of `other`.
    pub fn above<X2: Copy, W2: Copy, H2: Copy>(&self, other: &Rect<X2, i32, W2, H2>) -> Rect<X, i32, W, u32> {
        Rect {
            pos: Point {
                x: self.pos.x,
                y: other.pos.y - self.size.height as i32,
            },
            size: self.size,
        }
    }
    /// Aligns the top edge of `self` with the bottom edge of `other`.
    pub fn below<X2: Copy, W2: Copy>(&self, other: &Rect<X2, i32, W2, u32>) -> Rect<X, i32, W, u32> {
        Rect {
            pos: Point {
                x: self.pos.x,
                y: other.bottom(),
            },
            size: self.size,
        }
    }
    /// Aligns the bottom edge of `self` with the bottom edge of `other`.
    pub fn align_bottom<X2: Copy, W2: Copy>(&self, other: &Rect<X2, i32, W2, u32>) -> Rect<X, i32, W, u32> {
        Rect {
            pos: Point {
                x: self.pos.x,
                y: other.bottom() - self.size.height as i32,
            },
            size: self.size,
        }
    }
    /// Positions `self` in the vertical center of `other`.
    pub fn center_ver_in<X2: Copy, W2: Copy>(&self, other: &Rect<X2, i32, W2, u32>) -> Rect<X, i32, W, u32> {
        Rect {
            pos: Point {
                x: self.pos.x,
                y: other.pos.y + (other.size.height - self.size.height) as i32 / 2,
            },
            size: self.size,
        }
    }
}

// From width & height
impl<X: Copy, Y: Copy> Rect<X, Y, u32, u32> {
    /// Positions `self` in the horizontal and vertical center of `other`.
    pub fn center_in(&self, other: &Bounds) -> Bounds {
        self.center_hor_in(&other).center_ver_in(&other)
    }
}

// From x & width
impl<Y: Copy, H: Copy> Rect<i32, Y, u32, H> {
    /// Returns the right edge of `self`.
    pub fn right(&self) -> i32 {
        self.pos.x + self.size.width as i32
    }
}
// From y & height
impl<X: Copy, W: Copy> Rect<X, i32, W, u32> {
    /// Returns the bottom edge of `self`.
    pub fn bottom(&self) -> i32 {
        self.pos.y + self.size.height as i32
    }
}


// From nothing
impl<X: Copy, Y: Copy, W: Copy, H: Copy> Rect<X, Y, W, H> {
    pub fn with_x(&self, value: i32) -> Rect<i32, Y, W, H> {
        Rect {
            pos: Point {
                x: value,
                y: self.pos.y,
            },
            size: self.size,
        }
    }
    pub fn with_y(&self, value: i32) -> Rect<X, i32, W, H> {
        Rect {
            pos: Point {
                x: self.pos.x,
                y: value,
            },
            size: self.size,
        }
    }
    pub fn with_width(&self, value: u32) -> Rect<X, Y, u32, H> {
        Rect {
            pos: self.pos,
            size: Size {
                width: value,
                height: self.size.height,
            }
        }
    }
    pub fn with_height(&self, value: u32) -> Rect<X, Y, W, u32> {
        Rect {
            pos: self.pos,
            size: Size {
                width: self.size.width,
                height: value,
            }
        }
    }
    pub fn with_size(&self, size: Size<u32, u32>) -> Rect<X, Y, u32, u32> {
        Rect {
            pos: self.pos,
            size,
        }
    }
    /// Sets the left edge of `self` to `x1` and the right edge of `self` to `x2`.
    pub fn between_hor(&self, x1: i32, x2: i32) -> Rect<i32, Y, u32, H> {
        Rect {
            pos: Point {
                x: x1,
                y: self.pos.y,
            },
            size: Size {
                width: (x2 - x1) as u32,
                height: self.size.height,
            }
        }
    }
    /// Sets the top edge of `self` to `y1` and the bottom edge of `self` to `y2`.
    pub fn between_ver(&self, y1: i32, y2: i32) -> Rect<X, i32, W, u32> {
        Rect {
            pos: Point {
                x: self.pos.x,
                y: y1,
            },
            size: Size {
                width: self.size.width,
                height: (y2 - y1) as u32,
            }
        }
    }
    /// Aligns the left edge of `self` with the left edge of `other`.
    pub fn align_left<Y2: Copy, W2: Copy, H2: Copy>(&self, other: &Rect<i32, Y2, W2, H2>) -> Rect<i32, Y, W, H> {
        Rect {
            pos: Point {
                x: other.pos.x,
                y: self.pos.y,
            },
            size: self.size,
        }
    }
    /// Aligns the top edge of `self` with the top edge of `other`.
    pub fn align_top<X2: Copy, W2: Copy, H2: Copy>(&self, other: &Rect<X2, i32, W2, H2>) -> Rect<X, i32, W, H> {
        Rect {
            pos: Point {
                x: self.pos.x,
                y: other.pos.y,
            },
            size: self.size,
        }
    }
}
