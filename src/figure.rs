use std::f32::consts::PI;
use crate::{Color, vec4};
use vec4::Vec4;
use crate::matrix4::Matrix4;

pub struct Figure {
    pub mesh: Mesh,
    pub ambient_reflection: Color,
    pub diffuse_reflection: Color,
    pub specular_reflection: Color,
}

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

pub struct Mesh {
    pub vertices: Vec<Vec4>,
    pub faces: Vec<Face>,
}

impl Mesh {
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

    /// radius is distance from torus center to center of a ring
    pub fn new_torus(radius: f32, ring_radius: f32, rings_amt: u32, ring_points_amt: u32) -> Self {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for ring_i in 0..rings_amt {
            let ring_angle = ring_i as f32 * (2.0*PI / rings_amt as f32); // all angles in radians

            for ring_segment_i in 0..ring_points_amt {
                let ring_segment_angle = ring_segment_i as f32 * (2.0*PI / ring_points_amt as f32);
                let z = ring_segment_angle.sin() * ring_radius;
                let x = ring_angle.cos() * (radius+ring_segment_angle.cos());
                let y = ring_angle.sin() * (radius+ring_segment_angle.cos());
                vertices.push(Vec4::new_point(x,y,z));
            }
        }

        fn coord_to_index(i: u32, j: u32, ring_points_amt: u32) -> usize {
            (i * ring_points_amt + j).try_into().unwrap()
        }

        for i in 0..ring_points_amt {
            for j in 0..rings_amt {
                // the vertices in the face are listed counter clock wise
                let face = Face::new(
                    vec![
                        coord_to_index(i,j, ring_points_amt),
                        coord_to_index((i+1)%rings_amt, j, ring_points_amt),
                        coord_to_index((i+1)%rings_amt, (j+1)% ring_points_amt, ring_points_amt),
                        coord_to_index(i,(j+1)% ring_points_amt, ring_points_amt),
                    ]);
                faces.push(face);
            }
        }

        Self{vertices, faces}
    }

    pub fn triangulate(&mut self) {
        let mut new_faces = Vec::new();

        for face in self.faces.iter() {
            if face.indexes.len() <= 3 { continue; }
            for i in 2..=face.indexes.len()-1 {
                new_faces.push(Face::new(vec![
                    face.indexes[0],
                    face.indexes[i-1],
                    face.indexes[i],
                ]));
            }
        }

        self.faces = new_faces;
    }

    pub fn transform(&mut self, t: &Matrix4) {
        for vertex in self.vertices.iter_mut() {
            *vertex = vertex.mul(t);
        }
    }
}