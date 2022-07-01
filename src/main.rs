extern crate core;

mod vec4;

use std::cmp::max;
use std::f32::consts::PI;
use vec4::Vec4;

mod figure;
use figure::Mesh;

mod matrix4;
mod vec2;
mod color;
mod ini_reader;

use color::Color;

use bmp::*;
use crate::figure::Figure;
use crate::ini_reader::IniConfiguration;
use crate::matrix4::{Matrix4, PolarCoord};
use crate::vec2::Vec2;

fn main() {
    println!("Hello, world!");
}

trait Light {
    fn calculate_reflected_light(&self);
}

struct Eye {
    pos: Vec4,
    looking_dir: Vec4,
    hfov_rad: f32,
    aspect_ratio: f32,  // w/h
    image_width: u32,   // the final image width in pixels
}

enum FigureType {
    Tetrahedron(),
    Torus(f32, f32, u32, u32), // radius, ring_radius, rings_amt, ring_points_amt
}

struct FigureDescription {
    figure_type: FigureType,
    ambient_reflection: Color,
    diffuse_reflection: Color,
    specular_reflection: Color,
    center: Vec4,
    scale:  f32,
    rotation_x_rad: f32,
    rotation_y_rad: f32,
    rotation_z_rad: f32,
}

struct SceneDescription {
    figures: Vec<FigureDescription>,
    lights:  Vec<Box<dyn Light>>,
    eye: Eye,
}

fn read_scene_description_from_ini_file(path_to_ini: &str) -> SceneDescription {

    let configuration = IniConfiguration::new(path_to_ini);

    let general = configuration.get_section("General").unwrap();

    // Reading eye info

    let aspect_ratio = general.as_f32_or_default("aspectRatio", 4.0/3.0);
    let eye_pos = general.as_tuple_or_default("eye", [20.0, 10.0, 15.0]);
    let eye_pos = Vec4::new_point(eye_pos[0], eye_pos[1],eye_pos[2]);
    let eye_looking_dir = general.as_tuple_or_default("viewDirection", [-eye_pos.x(), -eye_pos.y(), -eye_pos.z()]);
    let eye_looking_dir= Vec4::new_vec4(eye_looking_dir[0], eye_looking_dir[1], eye_looking_dir[2]);
    let hfov_rad = general.as_f32_or_default("hfov", 90.0).to_radians();
    let image_width = general.as_f32_or_default("size", 1024.0) as u32;

    let eye = Eye{ pos: eye_pos, looking_dir: eye_looking_dir, hfov_rad, aspect_ratio, image_width};

    // Reading figures

    let mut figures = Vec::new();
    let figures_amt = general.as_f32_or_default("nrFigures", 0.0) as u32;

    for i in 0..figures_amt {
        let figure_section = configuration.get_section(&format!("Figure{i}")).unwrap();

        let figure_type = figure_section.as_string_or_die("type");

        let mut figure_type = match figure_type.as_str() {
            "Tetrahedron" => { FigureType::Tetrahedron() },
            "Torus"       => {
                let radius = figure_section.as_f32_or_die("R");
                let ring_radius = figure_section.as_f32_or_die("r");
                let rings_amt = figure_section.as_f32_or_die("n") as u32;
                let ring_points_amt = figure_section.as_f32_or_die("m") as u32;
                FigureType::Torus(radius, ring_radius, rings_amt, ring_points_amt)
            }
            _ => {
                println!("Too bad. I don't have your requested shape. How about a torus instead?");
                FigureType::Torus(5.0, 1.0, 20, 20)
            }
        };

        let ambient_reflection = if figure_section.key_exists("color") {
            figure_section.as_tuple_or_die("color")
        }
        else {
            figure_section.as_tuple_or_die("ambientReflection")
        };
        let ambient_reflection = Color::new(ambient_reflection[0], ambient_reflection[1], ambient_reflection[2]);

        let diffuse_reflection = figure_section.as_tuple_or_default("diffuseReflection", [0.0;3]);
        let diffuse_reflection = Color::new(diffuse_reflection[0], diffuse_reflection[1], diffuse_reflection[2]);

        let specular_reflection = figure_section.as_tuple_or_default("specularReflection", [0.0;3]);
        let specular_reflection = Color::new(specular_reflection[0], specular_reflection[1], specular_reflection[2]);

        let center = figure_section.as_tuple_or_default("center", [0.0;3]);
        let center = Vec4::new_vec4(center[0], center[1], center[2]);

        let scale = figure_section.as_f32_or_default("scale", 1.0);

        let rotation_x_rad = figure_section.as_f32_or_default("rotateX", 0.0).to_radians();
        let rotation_y_rad = figure_section.as_f32_or_default("rotateY", 0.0).to_radians();
        let rotation_z_rad = figure_section.as_f32_or_default("rotateZ", 0.0).to_radians();

        figures.push(FigureDescription{
            figure_type,
            center,
            scale,
            rotation_x_rad, rotation_y_rad, rotation_z_rad,
            ambient_reflection, diffuse_reflection, specular_reflection,
        });
    }

    let mut lights = Vec::new();

    SceneDescription{figures, lights, eye}
}

fn render_scene(scene_desc: &SceneDescription, path_to_output_image: &str) {

    let aspect_ratio = scene_desc.eye.aspect_ratio; // width / height
    let image_width = scene_desc.eye.image_width;
    let image_height = (image_width as f32 * 1.0/aspect_ratio) as u32;

    let mut image = Image::new(image_width, image_height);

    let d_near = 1.0;
    let d_far = 100.0;
    let hfov_rad = PI/6.0;
    let left = -f32::tan(hfov_rad/2.0) * d_near;
    let right = -left;
    let top = right * 1.0/aspect_ratio;
    let bottom = -top;

    let viewport_scaling = image_width as f32/(right-left) * 0.99;
    let viewport_offset = Vec2::new(-left, -bottom);

    let eye_pos = scene_desc.eye.pos;
    let looking_dir = scene_desc.eye.looking_dir;
    //let eye_point_transform = Matrix4::new_eye_point_transform_looking_at_origin(&eye_pos);
    let eye_point_transform = Matrix4::new_eye_point_transform(&eye_pos, &looking_dir);

    for figure_desc in scene_desc.figures.iter() {

        let fig_mesh = match figure_desc.figure_type {
            FigureType::Tetrahedron() => { Mesh::new_tetrahedron() }
            FigureType::Torus(radius, ring_radius, rings_amt, ring_points_amt) => {
                Mesh::new_torus(radius, ring_radius, rings_amt, ring_points_amt)
            }
        };

        let mut fig = Figure{
            mesh: fig_mesh,
            ambient_reflection: figure_desc.ambient_reflection,
            diffuse_reflection: figure_desc.diffuse_reflection,
            specular_reflection: figure_desc.specular_reflection,
        };

        fig.mesh.triangulate();
        fig.mesh.transform(&Matrix4::new_rotation_x(-figure_desc.rotation_x_rad)); // negate angle cuz counter clockwise rotation
        fig.mesh.transform(&Matrix4::new_rotation_z(-figure_desc.rotation_z_rad)); // negate angle cuz counter clockwise rotation
        //todo: rotate around y
        fig.mesh.transform(&Matrix4::new_translation(&figure_desc.center));

        fig.mesh.transform(&Matrix4::new_eye_point_transform(&scene_desc.eye.pos, &scene_desc.eye.looking_dir));

        for face in fig.mesh.faces.iter() {
            let a = &fig.mesh.vertices[face.indexes[0]];
            let b = &fig.mesh.vertices[face.indexes[1]];
            let c = &fig.mesh.vertices[face.indexes[2]];
            draw_triangle(a, b, c,
                          viewport_scaling, &viewport_offset,
                          &fig.ambient_reflection, &fig.diffuse_reflection, &fig.specular_reflection,
                          &mut image);
        }
    }

    image.save(path_to_output_image).expect(&format!("writing image: {path_to_output_image} to file failed"));
}

#[test]
fn test_scene_rendering() {
    let scene = read_scene_description_from_ini_file("tori.ini");
    render_scene(&scene, "tori.bmp");
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
    let viewport_scaling = image_width as f32/(right-left) * 0.99;
    let viewport_offset = Vec2::new(-left, -bottom);

    let eye_pos = Vec4::new_point(20.0, 10.0, 15.0);
    //let eye_point_transform = Matrix4::new_eye_point_transform_looking_at_origin(&eye_pos);
    let eye_point_transform = Matrix4::new_eye_point_transform(&eye_pos, &eye_pos.neg());

    let mut figures = Vec::new();

    let mut mesh = Mesh::new_torus(3.0, 1.0, 36, 36);
    mesh.triangulate();
    mesh.transform(&eye_point_transform);

    let ambient_reflection = Color::new(1.0,0.0,1.0);
    let diffuse_reflection = ambient_reflection;
    let specular_reflection = ambient_reflection;

    let fig = Figure{ mesh, ambient_reflection, diffuse_reflection, specular_reflection };

    figures.push(fig);

    for fig in figures.iter() {
        for face in fig.mesh.faces.iter() {
            if face.indexes.len() != 3 {
                eprintln!("face must be a triangle");
                break;
            }
            let a = &fig.mesh.vertices[face.indexes[0]];
            let b = &fig.mesh.vertices[face.indexes[1]];
            let c = &fig.mesh.vertices[face.indexes[2]];
            draw_triangle(a, b, c,
                          viewport_scaling, &viewport_offset,
                          &fig.ambient_reflection, &fig.diffuse_reflection, &fig.specular_reflection,
                          &mut image);
        }
    }

    image.save("siccimage.bmp").expect("writing to file failed");
}



fn draw_triangle(a: &Vec4, b: &Vec4, c: &Vec4,
                 viewport_scaling: f32, viewport_offset: &Vec2,
                 ambient_reflection: &Color, diffuse_reflection: &Color, specular_reflection: &Color,
                 image: &mut Image) {
    // project a, b and c to screen space
    let proj_a = project_point(a, viewport_scaling, viewport_offset);
    let proj_b = project_point(b, viewport_scaling, viewport_offset);
    let proj_c = project_point(c, viewport_scaling, viewport_offset);

    // find min and max y values of the projected triangle (bounding box)
    let proj_y_values = vec![proj_a.y(), proj_b.y(), proj_c.y()];
    let min_y = proj_y_values.iter().fold(f32::INFINITY, |a, &b| a.min(b)) as u32;
    let max_y = proj_y_values.iter().fold(-f32::INFINITY, |a, &b| a.max(b)) as u32;

    let mut reflected_color = *ambient_reflection;

    for y_i in min_y..=max_y {
        // determine where to start drawing the horizontal "scanline" and where to end
        let (x_l, x_r) = calculate_scanline(y_i, &proj_a, &proj_b, &proj_c);
        for x_i in x_l..=x_r {
            image.put_pixel(x_i, y_i, reflected_color.to_pixel());
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