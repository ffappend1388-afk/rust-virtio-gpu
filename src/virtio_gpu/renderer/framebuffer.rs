#[derive(Debug)]
pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub dirty: bool,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;

        Self {
            width,
            height,
            data: vec![0; size],
            dirty: true,
        }
    }

    pub fn update(&mut self, src: &[u8]) {
        let len = self.data.len().min(src.len());

        self.data[..len].copy_from_slice(&src[..len]);

        self.dirty = true;
    }

    pub fn fill_test_pattern(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = ((y * self.width + x) * 4) as usize;

                self.data[index] = (x % 256) as u8; // B
                self.data[index + 1] = (y % 256) as u8; // G
                self.data[index + 2] = 120; // R
                self.data[index + 3] = 255; // A
            }
        }
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
