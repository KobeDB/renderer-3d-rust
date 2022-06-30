use std::collections::HashMap;
use bmp::Pixel;
use bmp::px;

#[derive(Copy, Clone)]
pub struct Color {
    values: [f32;3],
}

impl Color {
    pub fn new(r:f32,g:f32,b:f32) -> Self {
        Self{values: [r.clamp(0.0,1.0),g.clamp(0.0,1.0),b.clamp(0.0,1.0)]}
    }

    pub fn r(&self) -> f32 {self.values[0]}
    pub fn g(&self) -> f32 {self.values[1]}
    pub fn b(&self) -> f32 {self.values[2]}

    pub fn to_pixel(&self) -> bmp::Pixel {
        px!(
                self.r() as u8 *255,
                self.g() as u8 *255,
                self.b() as u8 *255
            )
    }
}