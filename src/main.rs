mod vec4;

use std::cmp::max;
use std::f32::consts::PI;
use vec4::Vec4;

mod figure;
use figure::Figure;

mod matrix4;
mod vec2;

use bmp::*;
use crate::matrix4::{Matrix4, PolarCoord};
use crate::vec2::Vec2;

fn main() {
    println!("Hello, world!");

    let mut image = Image::new(100,100);

    for (x,y) in image.coordinates() {
        image.put_pixel(x, y, px!(255, 0, 255));
    }

    image.put_pixel(0,0, Pixel{r:255,g:255,b:255});
    image.put_pixel(0,image.get_height()-1, Pixel{r:0,g:255,b:255});

    image.save("siccimage.bmp").expect("writing to file failed");
}

#[test]
fn test_rendering_stuff() {
    let aspect_ratio = 16.0/9.0; // width / height
    let image_width = 1024;
    let image_height = (image_width as f32 * 1.0/aspect_ratio) as u32;

    let d_near = 1.0;
    let d_far = 100.0;
    let hfov_rad = PI/6.0;
    let left = -f32::tan(hfov_rad/2.0) * d_near;
    let right = -left;
    let top = right * 1.0/aspect_ratio;
    let bottom = -top;

    let mut image = Image::new(image_width, image_height);
    let viewport_scaling = image_width as f32/(right-left) * 0.9;
    let viewport_offset = Vec2::new(-left, -bottom);

    let eye_pos = Vec4::new_point(20.0, 10.0, 15.0);
    //let eye_point_transform = Matrix4::new_eye_point_transform_looking_at_origin(&eye_pos);
    let eye_point_transform = Matrix4::new_eye_point_transform(&eye_pos, &eye_pos.neg());

    let mut fig = Figure::new_tetrahedron();
    fig.transform(&eye_point_transform);

    for face in fig.faces {
        if face.indexes.len() != 3 {
            eprintln!("face must be a triangle");
        }
        let a = &fig.vertices[face.indexes[0]];
        let b = &fig.vertices[face.indexes[1]];
        let c = &fig.vertices[face.indexes[2]];
        draw_triangle(a, b, c, viewport_scaling, &viewport_offset, &mut image);
    }

    image.save("siccimage.bmp").expect("writing to file failed");
}

fn draw_triangle(a: &Vec4, b: &Vec4, c: &Vec4, viewport_scaling: f32, viewport_offset: &Vec2, image: &mut Image) {
    // project a, b and c to screen space
    let proj_a = project_point(a, viewport_scaling, viewport_offset);
    let proj_b = project_point(b, viewport_scaling, viewport_offset);
    let proj_c = project_point(c, viewport_scaling, viewport_offset);

    // find min and max y values of the projected triangle
    let proj_y_values = vec![proj_a.y(), proj_b.y(), proj_c.y()];
    let min_y = proj_y_values.iter().fold(f32::INFINITY, |a, &b| a.min(b)) as u32;
    let max_y = proj_y_values.iter().fold(-f32::INFINITY, |a, &b| a.max(b)) as u32;

    for y_i in min_y+1..max_y {
        // determine where to start drawing the horizontal "scanline" and where to end
        let (x_l, x_r) = calculate_scanline(y_i, &proj_a, &proj_b, &proj_c);
        for x_i in x_l..x_r {
            image.put_pixel(x_i, y_i, px!(255,0,255));
        }
    }

    fn calculate_scanline(y: u32, proj_a: &Vec2, proj_b: &Vec2, proj_c: &Vec2) -> (u32, u32) {
        let mut x_l_ab = f32::MAX;
        let mut x_l_ac = f32::MAX;
        let mut x_l_bc = f32::MAX;
        let mut x_r_ab = f32::MIN;
        let mut x_r_ac = f32::MIN;
        let mut x_r_bc = f32::MIN;
        update_x_l_and_x_r(&mut x_l_ab, &mut x_r_ab, y, proj_a, proj_b);
        update_x_l_and_x_r(&mut x_l_ac, &mut x_r_ac, y, proj_a, proj_c);
        update_x_l_and_x_r(&mut x_l_bc, &mut x_r_bc, y, proj_b, proj_c);

        let x_l_candidates = vec![x_l_ab, x_l_ac, x_l_bc];
        let x_r_candidates = vec![x_r_ab, x_r_ac, x_r_bc];

        let x_l = (x_l_candidates.iter().fold(f32::INFINITY, |a, &b| a.min(b)) + 0.5) as u32;
        let x_r = (x_r_candidates.iter().fold(-f32::INFINITY, |a, &b| a.max(b)) - 0.5) as u32;

        (x_l,x_r)
    }

    fn update_x_l_and_x_r(x_l_pq: &mut f32, x_r_pq: &mut f32, y: u32, proj_p: &Vec2, proj_q: &Vec2) {
        let y = y as f32;
        if (y - proj_p.y()) * (y-proj_q.y()) > 0.0 || proj_p.y() == proj_q.y()  {
            return;
        }
        let intersection_x = proj_q.x() + (proj_p.x() - proj_q.x()) * (y - proj_q.y()) / (proj_p.y() - proj_q.y());
        *x_l_pq = intersection_x;
        *x_r_pq = intersection_x;
    }

    fn project_point(p: &Vec4, img_scaling: f32, img_offset: &Vec2) -> Vec2 {
        let x = (-p.x()/p.z() + img_offset.x()) * img_scaling;
        let y = (-p.y()/p.z() + img_offset.y()) * img_scaling;

        Vec2::new(x, y)
    }
}

// Is there a cleaner alternative to achieve this?
trait PutPixelWhereOriginIsBottomLeft {
    fn put_pixel(&mut self, x: u32, y:u32, p: Pixel);
}

impl PutPixelWhereOriginIsBottomLeft for Image {
    fn put_pixel(&mut self, x: u32, y: u32, p: Pixel) {
        self.set_pixel(x, self.get_height()-1-y, p);
    }
}