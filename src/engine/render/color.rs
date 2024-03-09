use sdl2::pixels::Color;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct RGBA {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}

impl RGBA {
  pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
    Self { red, green, blue, alpha }
  }
  pub fn destructure(self) -> (u8, u8, u8, u8) {
    (self.red, self.green, self.blue, self.alpha)
  }
}

impl From<&RGBA> for Color {
  fn from(value: &RGBA) -> Self {
    Self { r: value.red, g: value.green, b: value.blue, a: value.alpha }
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

  pub const ROSEWATER: RGBA = RGBA::new(245, 224, 220, OPAQUE);
  pub const FLAMINGO: RGBA = RGBA::new(242, 205, 205, OPAQUE);
  pub const PINK: RGBA = RGBA::new(245, 194, 231, OPAQUE);
  pub const MAUVE: RGBA = RGBA::new(203, 166, 247, OPAQUE);
  pub const RED: RGBA = RGBA::new(243, 139, 168, OPAQUE);
  pub const MAROON: RGBA = RGBA::new(235, 160, 172, OPAQUE);
  pub const PEACH: RGBA = RGBA::new(250, 179, 135, OPAQUE);
  pub const YELLOW: RGBA = RGBA::new(249, 226, 175, OPAQUE);
  pub const GREEN: RGBA = RGBA::new(166, 227, 161, OPAQUE);
  pub const TEAL: RGBA = RGBA::new(148, 226, 213, OPAQUE);
  pub const SKY: RGBA = RGBA::new(137, 220, 235, OPAQUE);
  pub const SAPPHIRE: RGBA = RGBA::new(116, 199, 236, OPAQUE);
  pub const BLUE: RGBA = RGBA::new(137, 180, 250, OPAQUE);
  pub const LAVENDER: RGBA = RGBA::new(180, 190, 254, OPAQUE);

  pub const TEXT: RGBA = RGBA::new(205, 214, 244, OPAQUE);
  pub const SURFACE_0: RGBA = RGBA::new(49, 50, 68, OPAQUE);
  pub const MANTLE: RGBA = RGBA::new(24, 24, 37, OPAQUE);

  // todo: finish this list
}