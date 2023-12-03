/*
 * This is the base vectors for obsidian.
*/

use super::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: Point,
    pub y: Point,
}

impl Vector2D {
    pub fn new(x: Point, y: Point) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x.x.powi(2) + self.y.y.powi(2)).sqrt()
    }
}
