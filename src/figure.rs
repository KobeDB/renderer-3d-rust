use crate::vec4;
use vec4::Vec4;
use crate::matrix4::Matrix4;

pub struct Face {
    // indexes in points from Figure, stored counter clock wise if you
    // look at the face from the outside
    pub indexes: Vec<usize>
}

impl Face{
    fn new(indexes: Vec<usize>) -> Self {
        Self{indexes}
    }
}

pub struct Figure {
    pub vertices: Vec<Vec4>,
    pub faces: Vec<Face>,
}

impl Figure {
    pub fn new_tetrahedron() -> Self {
        let points: Vec<Vec4> = vec![
            Vec4::new_point(1.0,-1.0,-1.0),
            Vec4::new_point(-1.0,1.0,-1.0),
            Vec4::new_point(1.0,1.0,1.0),
            Vec4::new_point(-1.0,-1.0,1.0),
        ];

        let faces: Vec<Face> = vec![
            Face::new(vec![0,1,2]),
            Face::new(vec![0,2,3]),
            Face::new(vec![0,3,1]),
            Face::new(vec![1,3,2])
        ];

        Self{ vertices: points, faces }
    }

    pub fn transform(&mut self, t: &Matrix4) {
        for vertex in self.vertices.iter_mut() {
            *vertex = vertex.mul(t);
        }
    }
}