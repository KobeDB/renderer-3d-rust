use std::f32::consts::PI;
use crate::{Vec4, vec4};

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

    pub fn new_eye_point_transformation(eye_pos: &PolarCoord) -> Self {
        let mut result = Self::new_identity();

        result = Self::mul(&result, &Self::new_rotation_z(eye_pos.theta_rad+PI/2.0));
        result = Self::mul(&result, &Self::new_rotation_x(eye_pos.phi_rad));

        result.elements[3][2] = -eye_pos.r;

        result
    }

    pub fn mul(a: &Self, b: &Self) -> Self {
        let mut elements = [[0.0 as f32; 4]; 4];

        for col in 0..=3 {
            for row in 0..=3 {
                let mut dot_prod: f32 = 0.0;
                for i in 0..=3 {
                    dot_prod += a.elements[row][i] * b.elements[i][col];
                }
                elements[row][col] = dot_prod;
            }
        }

        Self{elements}
    }
}

pub struct PolarCoord {
    pub theta_rad:  f32,
    pub phi_rad:    f32,
    pub r:          f32,
}

impl PolarCoord {
    pub fn new(theta_rad: f32, phi_rad: f32, r: f32) -> Self {
        Self{theta_rad,phi_rad,r}
    }

    pub fn new_from_cartesian(x:f32,y:f32,z:f32) -> Self {
        let r = f32::sqrt(x*x+y*y+z*z);
        let theta_rad = f32::atan2(y,x);
        let phi_rad = f32::acos(z/r);

        Self{theta_rad,phi_rad,r}
    }
}

#[test]
fn test_eyepoint() {
    let eye_pos = PolarCoord::new(PI/2.0, PI/2.0, 5.0);
    let eye_point_transform = Matrix4::new_eye_point_transformation(&eye_pos);

    let mut a = Vec4::new_point(1.0,1.0,1.0);
    a = a.mul(&eye_point_transform);

    let mut b = Vec4::new_point(-1.0,-1.0,0.0);
    b = b.mul(&eye_point_transform);

    let mut c = Vec4::new_point(0.0,5.0,0.0);
    c = c.mul(&eye_point_transform);

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("b: {:?}", c);

}