pub mod terminal;
pub mod fb;

use crate::model::Model;
use crate::lin::{Vec2, Vec3, Triangle3, Transform};
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
    fn depth_buffer(&mut self) -> &mut Vec<f32>;

    fn get_depth(&mut self, x: u32, y: u32) -> f32 {
        let width = self.size().0 as usize;
        let db = self.depth_buffer();

        let idx = (y as usize * width) + x as usize;
        if idx >= db.len() {
            return f32::INFINITY;
        }

        db[idx]
    }

    fn set_depth(&mut self, x: u32, y: u32, depth: f32) {
        let width = self.size().0 as usize;
        let db = self.depth_buffer();

        let idx = (y as usize * width) + x as usize;
        if idx >= db.len() {
            return;
        }

        db[idx] = depth;
    }

    fn reset_depth_buffer(&mut self) {
        let db = self.depth_buffer();

        db.fill(f32::INFINITY)
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color);
    fn clear_pixels(&mut self);

    fn clear(&mut self) {
        self.clear_pixels();
        self.reset_depth_buffer();
    }

    fn size(&self) -> (u32, u32);
    fn commit(&mut self) -> anyhow::Result<()>;

    fn draw_triangle(&mut self, tri: Triangle3, color: Color) {
        // calculate bounding box
        let min_x = f32::min(f32::min(tri.a.x, tri.b.x), tri.c.x).floor() as u32;
        let max_x = f32::max(f32::max(tri.a.x, tri.b.x), tri.c.x).ceil() as u32;
        let min_y = f32::min(f32::min(tri.a.y, tri.b.y), tri.c.y).floor() as u32;
        let max_y = f32::max(f32::max(tri.a.y, tri.b.y), tri.c.y).ceil() as u32;

        for x in min_x..max_x {
            for y in min_y..max_y {
                if let Some(depth_weights) = tri.trunc().depth_at(Vec2 { x: x as f32, y: y as f32 }) {
                    let this_depth = Vec3::new(tri.a.z, tri.b.z, tri.c.z).recip().dot(depth_weights).recip();
                    if this_depth < self.get_depth(x, y) {
                        self.set_pixel(x, y, color);
                        self.set_depth(x, y, this_depth);
                    }
                }
            }
        }
    }

    fn draw_model(&mut self, model: &Model, transform: Transform) {
        let size = self.size();

        for (triangle, color) in model.as_projected_triangles(transform, Vec2::new(size.0 as f32, size.1 as f32)).into_iter().zip(model.colors.iter()) {
            self.draw_triangle(triangle, *color);
        }
    }
}
