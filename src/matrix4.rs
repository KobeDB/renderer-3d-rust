/// we use row vectors
pub struct Matrix4 {
    pub elements: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn new_identity() -> Self {
        let mut elements = [[0.0 as f32; 4]; 4];

        for i in 0..=3 {
            elements[i][i] = 1.0;
        }

        Self{elements}
    }

    /// Clock-wise rotation ( because this will later be used for a very specific transformation)
    pub fn new_rotation_z(angle_rad: f32) -> Self {
        let mut result = Self::new_identity();

        result.elements[0][0] = f32::cos(angle_rad);
        result.elements[0][1] = -f32::sin(angle_rad);
        result.elements[1][0] = f32::sin(angle_rad);
        result.elements[1][1] = f32::cos(angle_rad);

        result
    }

    /// Clock-wise rotation ( because this will later be used for a very specific transformation)
    pub fn new_rotation_x(angle_rad: f32) -> Self {
        let mut result = Self::new_identity();

        result.elements[1][1] = f32::cos(angle_rad);
        result.elements[1][2] = -f32::sin(angle_rad);
        result.elements[2][1] = f32::sin(angle_rad);
        result.elements[2][2] = f32::cos(angle_rad);

        result
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut elements = [[0.0 as f32; 4]; 4];

        for col in 0..=3 {
            for row in 0..=3 {
                let mut dot_prod: f32 = 0.0;
                for i in 0..=3 {
                    dot_prod += self.elements[row][i] * other.elements[i][col];
                }
                elements[row][col] = dot_prod;
            }
        }

        Self{elements}
    }
}