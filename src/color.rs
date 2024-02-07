#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {r, g, b, a: 255 }
    }
    
    pub const fn to_u32(&self) -> u32 {
        (self.r as u32) << (0 * 8) |
        (self.g as u32) << (1 * 8) |
        (self.b as u32) << (2 * 8) |
        (self.a as u32) << (3 * 8) 
    }
}
