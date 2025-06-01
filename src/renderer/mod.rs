pub mod terminal;
pub mod fb;

use crate::lin::{Vec2, Triangle2};
use libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self{
        Self { r, g, b }
    }

    pub fn from_u32(packed: u32) -> Self {
        let r = (packed >> 16) as u8;
        let g = ((packed >> 8) | 0xFF) as u8;
        let b = (packed | 0xFF) as u8;

        Self { r, g, b }
    }

    pub fn as_u32(&self) -> u32 {
        ((self.r as u32) << 16) + ((self.g as u32) << 8) + self.b as u32
    }

    // https://stackoverflow.com/a/596243
    pub fn percieved_luminance(&self) -> f32 {
        (0.299 * (self.r as f32 / 255.0)) + (0.587 * (self.g as f32 / 255.0)) + (0.114 * (self.b as f32 / 255.0))
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();

        Self {
            r: rng.random::<u8>(),
            g: rng.random::<u8>(),
            b: rng.random::<u8>()
        }
    }
}

pub trait Renderer {
    fn set_pixel(&mut self, x: u32, y: u32, color: Color);
    fn clear(&mut self);
    fn size(&self) -> (u32, u32);
    fn commit(&mut self) -> anyhow::Result<()>;

    fn draw_triangle(&mut self, tri: Triangle2, color: Color) {
        // calculate bounding box
        let min_x = f32::min(f32::min(tri.a.x, tri.b.x), tri.c.x).floor() as u32;
        let max_x = f32::max(f32::max(tri.a.x, tri.b.x), tri.c.x).ceil() as u32;
        let min_y = f32::min(f32::min(tri.a.y, tri.b.y), tri.c.y).floor() as u32;
        let max_y = f32::max(f32::max(tri.a.y, tri.b.y), tri.c.y).ceil() as u32;

        for x in min_x..max_x {
            for y in min_y..max_y {
                if tri.point_inside(Vec2 { x: x as f32, y: y as f32 }) {
                    self.set_pixel(x, y, color);
                }
            }
        }
    }
}
