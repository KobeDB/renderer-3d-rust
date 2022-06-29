pub struct Vec2 {
    elems: [f32;2]
}

impl Vec2 {
    pub fn new(x:f32, y:f32) -> Self {
        Self{elems: [x,y]}
    }

    pub fn x(&self) -> f32 { return self.elems[0] }
    pub fn y(&self) -> f32 { return self.elems[1] }
}