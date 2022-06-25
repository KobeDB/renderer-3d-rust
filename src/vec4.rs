pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {
    /// Creates a 4D point, used to represent 3D positions.
    /// The 4th component is the homogeneous coordinate used for translations.
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Vec4 { x, y, z, w: 1.0 }
    }

    /// Creates a 4D vector.
    /// Contrary to point, translations do not affect this vector.
    /// Thus vector is an ideal way of representing directions.
    pub fn new_vec4(x: f32, y: f32, z: f32) -> Self {
        Vec4 { x, y, z, w: 0.0 }
    }
}