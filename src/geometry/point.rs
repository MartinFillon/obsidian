/*
 * This is the based point for obsidian. It is a simple struct that holds two
 * u32 values, x and y.
 */

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let x = (self.x - other.x).powi(2);
        let y = (self.y - other.y).powi(2);
        (x + y).sqrt()
    }
}