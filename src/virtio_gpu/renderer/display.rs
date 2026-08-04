use minifb::{Key, Window, WindowOptions};

use super::framebuffer::FrameBuffer;

pub struct Display {
    window: Window,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Self {
        let window =
            Window::new("rust-virtio-gpu", width, height, WindowOptions::default()).unwrap();

        Self { window }
    }

    pub fn update(&mut self, framebuffer: &mut FrameBuffer) {
        let pixels: Vec<u32> = framebuffer
            .data
            .chunks_exact(4)
            .map(|p| {
                let b = p[0] as u32;
                let g = p[1] as u32;
                let r = p[2] as u32;

                (r << 16) | (g << 8) | b
            })
            .collect();

        self.window
            .update_with_buffer(
                &pixels,
                framebuffer.width as usize,
                framebuffer.height as usize,
            )
            .unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }
}
