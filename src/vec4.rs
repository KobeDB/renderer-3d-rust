use crate::matrix4::Matrix4;

#[derive(Debug)]
pub struct Vec4 { elems: [f32;4] }

impl Vec4 {

    pub fn x(&self) -> f32 { self.elems[0] }
    pub fn y(&self) -> f32 { self.elems[1] }
    pub fn z(&self) -> f32 { self.elems[2] }
    pub fn w(&self) -> f32 { self.elems[3] }

    /// Creates a 4D point, used to represent 3D positions.
    /// The 4th component is the homogeneous coordinate used for translations.
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Self{elems: [ x, y, z, 1.0 ]}
    }

    /// Creates a 4D vector.
    /// Contrary to point, translations do not affect this vector.
    /// Thus vector is an ideal way of representing directions.
    pub fn new_vec4(x: f32, y: f32, z: f32) -> Self {
        Self{elems: [ x, y, z, 0.0 ]}
    }

    pub fn transform(&mut self, mat: &Matrix4) {
        for el in 0..=3 {
            let mut new_el: f32 = 0.0;
            for i in 0..=3 {
                new_el += mat.elements[i][el] * self.elems[i];
            }
            new_el *= self.elems[el];
            self.elems[el] = new_el;
        }
    }
}

fn dot_product(u: &[f32;4], v: &[f32;4]) -> f32 {
    let mut result = 0.0;
    for i in 0..=3 {
        result += u[i]*v[i];
    }
    result
}