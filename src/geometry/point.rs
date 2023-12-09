/*
 * This is the based point for obsidian. It is a simple struct that holds two
 * u32 values, x and y.
 */

use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let x = (self.x - other.x).powi(2);
        let y = (self.y - other.y).powi(2);
        let z = (self.z - other.z).powi(2);
        (x + y + z).sqrt()
    }

    pub fn to_coords(&self) -> Point {
        let video_mode = glfw::Monitor::from_primary().get_video_mode().unwrap();

        Point::new(
            self.x / video_mode.width as f32,
            self.y / video_mode.height as f32,
            self.z / 1.0,
        )
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl From<(f32, f32, f32)> for Point {
    fn from(value: (f32, f32, f32)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}
