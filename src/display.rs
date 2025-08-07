use nannou::image::{ Pixel, Rgba };

pub struct ColorBuffer {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl ColorBuffer {
    pub fn new(width: u32, height:u32) -> Self {
        let pixel_count = (width * height) as usize;
        let data = vec![0; pixel_count * 4];
        Self { data, width, height }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgba<u8>) {
        let idx = ((y * self.width + x) * 4) as usize;

        if idx < self.data.len() {
            for (channel, value) in color.channels().iter().enumerate() {
                self.data[idx + channel] = *value;
            }
        }
    }

    pub fn clear(&mut self, color: Rgba<u8>) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
    
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

