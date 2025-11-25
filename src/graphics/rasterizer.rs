use std::error::Error;

use super::image::{Framebuffer, Image};

pub struct DepthTesting {
    pub test: bool,
    pub write: bool,
}

pub enum WindingOrder {
    Clockwise,
    CounterClockwise,
}

pub enum Topology {
    TriangleList,
    TriangleStrip,
}

pub struct Pipeline {
    pub depth: DepthTesting,

    pub cull_back: bool,
    pub winding_order: WindingOrder,
    pub topology: Topology,
}

pub struct Rasterizer {
    // todo: multithreading worker
}

pub struct ClearValue {
    pub color: u32,
    pub depth: f32,
}

impl Rasterizer {
    fn clear_attachment<T: Sized + Copy>(&self, attachment: &mut Image<T>, value: T) {
        let (width, height) = attachment.size();

        for y in 0..height {
            for x in 0..width {
                attachment.exchange(x, y, value);
            }
        }
    }

    pub fn clear_framebuffer(
        &self,
        fb: &mut Framebuffer,
        value: &ClearValue,
    ) -> Result<(), Box<dyn Error>> {
        fb.validate()?;

        for attachment in &mut fb.color {
            self.clear_attachment(attachment, value.color);
        }

        match &mut fb.depth {
            Some(depth) => self.clear_attachment(depth, value.depth),
            None => (),
        };

        Ok(())
    }
}
