use sdl2::pixels::Color;

#[derive(Copy, Clone, Debug, Default)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl RGBA {
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
    pub fn destructure(self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }
}

impl From<&RGBA> for Color {
    fn from(value: &RGBA) -> Self {
        Self {
            r: value.red,
            g: value.green,
            b: value.blue,
            a: value.alpha,
        }
    }
}

impl From<RGBA> for Color {
    fn from(value: RGBA) -> Self {
        Self::from(&value)
    }
}

// Utility //

pub const U8MAX: u8 = 255;
pub const U8MIN: u8 = 255;
pub const TRANSPARENT: u8 = U8MIN;
pub const OPAQUE: u8 = U8MAX;

// common //

pub mod color {
    use super::{OPAQUE, RGBA};

    pub const RED: RGBA = RGBA::new(255, 0, 0, OPAQUE);
    pub const GREEN: RGBA = RGBA::new(0, 255, 0, OPAQUE);
    pub const BLUE: RGBA = RGBA::new(0, 0, 255, OPAQUE);

    pub const YELLOW: RGBA = RGBA::new(255, 255, 0, OPAQUE);
    pub const MAGENTA: RGBA = RGBA::new(255, 0, 255, OPAQUE);
    pub const CYAN: RGBA = RGBA::new(0, 255, 255, OPAQUE);

    pub const WHITE: RGBA = RGBA::new(255, 255, 255, OPAQUE);
    pub const BLACK: RGBA = RGBA::new(0, 0, 0, OPAQUE);
    pub const GRAY: RGBA = RGBA::new(128, 128, 128, OPAQUE);
}
