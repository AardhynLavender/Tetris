use std::marker::Copy;

use num::{Num, Unsigned};
use sdl2::rect::{Point, Rect};

// Traits //

/// Primitive type for geometric shapes
pub trait UnitPrimitive: Num + Copy {}

impl<T: Num + Copy> UnitPrimitive for T {}

/// Primitive type for geometric sizes
pub trait SizePrimitive: UnitPrimitive + Unsigned + Into<u32> {}

impl<T: UnitPrimitive + Unsigned + Into<u32>> SizePrimitive for T {}

/// Can the shape primitive be converted to an i32
///
/// SDL2 uses integers internally for rendering, supplied type `T` must conform to this constraint if used in rendering
pub trait IntConvertable: UnitPrimitive + Into<i32> {}

impl<T: UnitPrimitive + Into<i32>> IntConvertable for T {}

// Vector 2D //

/// A vector representation in 2D space of some numeric type `T`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2<T>
  where
    T: UnitPrimitive,
{
  pub x: T,
  pub y: T,
}

impl<T: UnitPrimitive> Vec2<T> {
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
  pub fn destructure(&self) -> (T, T) {
    (self.x, self.y)
  }
}

impl<T: IntConvertable> From<Vec2<T>> for Point {
  fn from(value: Vec2<T>) -> Self {
    Self::from(&value)
  }
}

impl<T: IntConvertable> From<&Vec2<T>> for Point {
  fn from(value: &Vec2<T>) -> Self {
    let (x, y) = value.destructure();
    Point::new(x.into(), y.into())
  }
}

// Rectangle 2D //

/// A Rectangle representation in 2D space of some numeric type `T`
#[derive(Clone, Copy, Debug, Default)]
pub struct Rec2<T: UnitPrimitive, U: SizePrimitive> {
  pub origin: Vec2<T>,
  pub size: Vec2<U>,
}

impl<T: UnitPrimitive, U: SizePrimitive> Rec2<T, U> {
  pub const fn new(origin: Vec2<T>, size: Vec2<U>) -> Self {
    Self { origin, size }
  }
  pub fn destructure(&self) -> ((T, T), (U, U)) {
    (self.origin.destructure(), self.size.destructure())
  }
}

impl<T: IntConvertable, U: SizePrimitive> From<Rec2<T, U>> for Rect {
  fn from(value: Rec2<T, U>) -> Self {
    let ((x, y), (w, h)) = value.destructure();
    Rect::new(x.into(), y.into(), w.into(), h.into())
  }
}

impl<T: UnitPrimitive, U: SizePrimitive> From<&Rec2<T, U>> for Rect {
  fn from(value: &Rec2<T, U>) -> Self {
    Self::from(value)
  }
}

/// A Line representation in 2D space of some numeric type `T`
#[derive(Clone, Copy, Debug, Default)]
pub struct Line2<T>
  where
    T: UnitPrimitive,
{
  pub start: Vec2<T>,
  pub end: Vec2<T>,
}

impl<T: UnitPrimitive> Line2<T> {
  pub const fn new(start: Vec2<T>, end: Vec2<T>) -> Self {
    Self { start, end }
  }
  pub fn destructure(&self) -> (Vec2<T>, Vec2<T>) {
    (self.start, self.end)
  }
}

/// A Polygon representation in 2D space of some numeric type `T`
#[derive(Debug, Default)]
pub struct Pol2<T>
  where
    T: UnitPrimitive,
{
  pub vertices: Vec<Vec2<T>>,
}

impl<T: UnitPrimitive> Pol2<T> {
  pub const fn new(vertices: Vec<Vec2<T>>) -> Self {
    Self { vertices }
  }
}

/// A Circle representation in 2D space of some numeric type `T`
#[derive(Clone, Copy, Debug, Default)]
pub struct Cir2<T>
  where
    T: UnitPrimitive,
{
  pub origin: Vec2<T>,
  pub radius: u32,
}

impl<T: UnitPrimitive> Cir2<T> {
  pub const fn new(origin: Vec2<T>, radius: u32) -> Self {
    Self { origin, radius }
  }
}

/// A Ray representation in 2D space of some numeric type
#[derive(Clone, Copy, Debug, Default)]
pub struct Ray2<T>
  where
    T: UnitPrimitive,
{
  pub origin: Vec2<T>,
  pub angle: f32,
}

impl<T: UnitPrimitive> Ray2<T> {
  pub const fn new(origin: Vec2<T>, angle: f32) -> Self {
    Self { origin, angle }
  }
}
