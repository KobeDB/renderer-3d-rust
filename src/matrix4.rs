use std::intrinsics::cosf32;

/// we use row vectors
pub struct Matrix4 {
    elements: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn new_identity() -> Self {
        let mut elements = [[0.0 as f32; 4]; 4];

        for i in 0..3 {
            elements[i][i] = 1.0;
        }

        Self{elements}
    }

    /// Clock-wise rotation ( because this will later be used for a very specific transformation)
    pub fn new_rotation_z(angle_rad: f32) -> Self {
        let mut result = Self::new_identity();

        elements[0][0] = f32::cos(angle_rad);
        elements[0][1] = -f32::sin(angle_rad);
        elements[1][0] = f32::sin(angle_rad);
        elements[1][1] = f32::cos(angle_rad);

        result
    }

    /// Clock-wise rotation ( because this will later be used for a very specific transformation)
    pub fn new_rotation_x(angle_rad: f32) -> Self {
        let mut result = Self::new_identity();

        elements[0][0] = f32::cos(angle_rad);
        elements[0][1] = -f32::sin(angle_rad);
        elements[1][0] = f32::sin(angle_rad);
        elements[1][1] = f32::cos(angle_rad);

        result
    }

    pub fn mul_mut(&mut self, other: &Self) {

    }
}