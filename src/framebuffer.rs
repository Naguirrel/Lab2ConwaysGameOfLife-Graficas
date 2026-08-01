pub const DEAD: u32 = 0x00070A18;

const ALIVE_COLORS: [u32; 7] = [
    0x00F7D046, // yellow
    0x00FF4D8D, // pink
    0x0000D9FF, // cyan
    0x007CF56B, // green
    0x00B86BFF, // violet
    0x00FF8A3D, // orange
    0x00FFFFFF, // white
];

pub fn is_alive(color: u32) -> bool {
    color != DEAD
}

pub fn alive_color(x: usize, y: usize, neighbors: u32) -> u32 {
    let index = (x * 3 + y * 5 + neighbors as usize) % ALIVE_COLORS.len();
    ALIVE_COLORS[index]
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![DEAD; width * height],
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x]
        } else {
            DEAD
        }
    }
}
