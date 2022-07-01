use crate::matrix4::Matrix4;

#[derive(Copy, Clone, Debug)]
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

    pub fn mul(&self, mat: &Matrix4) -> Self {
        let mut elems = [0.0 as f32;4];
        for el in 0..=3 {
            let mut new_el: f32 = 0.0;
            for i in 0..=3 {
                new_el += mat.elements[i][el] * self.elems[i];
            }
            elems[el] = new_el;
        }
        Self{elems}
    }

    pub fn normalize(&self) -> Self{
        let l = (self.elems[0]*self.elems[0] + self.elems[1]*self.elems[1] + self.elems[2]*self.elems[2]).sqrt();
        Self{elems: [self.elems[0]/l, self.elems[1]/l, self.elems[2]/l, self.elems[3]]}
    }

    pub fn neg(&self) -> Self {
        let mut elems = self.elems.clone();
        for i in 0..=2 {
            elems[i] *= -1.0;
        }
        Self{elems}
    }
}

fn dot_product(u: &[f32;4], v: &[f32;4]) -> f32 {
    let mut result = 0.0;
    for i in 0..=3 {
        result += u[i]*v[i];
    }
    result
}

impl PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {
        self.elems == other.elems
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[test]
fn test_vec_mat_mul() {
    let v = Vec4::new_point(420.0,69.0,21.0);
    let vt = v.mul(&Matrix4::new_identity());
    assert_eq!(v, vt);
}