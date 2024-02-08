#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {r, g, b, a: 255 }
    }
    
    pub fn to_u32(&self) -> u32 {
        (self.r as u32) << (0 * 8) |
        (self.g as u32) << (1 * 8) |
        (self.b as u32) << (2 * 8) |
        (self.a as u32) << (3 * 8) 
    }

    pub fn darken(&self, f: f32) -> Self {
        if f < 0.0 {
            Self { r: 0, g: 0, b: 0, a: 255 }
        }
        else if f <= 1.0 {
            Self {
                r: ((self.r as f32) * f) as u8,
                g: ((self.g as f32) * f) as u8,
                b: ((self.b as f32) * f) as u8,
                a: 255
            }
        }
        else {
            Self { r: 255, g: 255, b: 255, a: 255 }
        }
    }
}
