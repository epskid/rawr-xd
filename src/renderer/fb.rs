use super::*;
use minifb::{Window, WindowOptions};

pub struct MinifbRenderer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    depth_buffer: Vec<f32>,
    window: Window,
}

impl MinifbRenderer {
    pub fn new(width: usize, height: usize) -> anyhow::Result<Self> {
        let window = Window::new("RAWR", width, height, WindowOptions::default())?;

        Ok(Self {
            width,
            height,
            buffer: vec![0; width * height],
            depth_buffer: vec![f32::INFINITY; width * height],
            window
        })
    }
}

impl Renderer for MinifbRenderer {
    fn depth_buffer(&mut self) -> &mut Vec<f32> {
        &mut self.depth_buffer
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let idx = (y as usize * self.width) + x as usize;
        if idx >= self.buffer.len() {
            return;
        }
        self.buffer[idx] = color.as_u32();
    }

    fn clear_pixels(&mut self) {
        self.buffer.fill(0);
    }

    fn size(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn commit(&mut self) -> anyhow::Result<()> {
        self.window.update_with_buffer(&self.buffer, self.width, self.height)?;
        Ok(())
    }
}
