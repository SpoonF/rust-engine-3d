
mod model;
mod geometry;
mod scene;

use std::{path::Path};
use crate::{geometry::{ Vector3D}, model::Model, scene::Scene};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const DEPTH: usize = 255;
const SIZE: f32 = 2.0;



fn main() {
    let model = Model::read(Path::new("obj/head.obj"));
    let light_dir = Vector3D::new(0.0, 0.0, -1.0);
    let mut scene = Scene::new(WIDTH, HEIGHT);

    scene.wait_for_exit(|scene: &mut Scene| {
        let faces = &model.faces;

        for face in faces {
            let mut screen_coords: [Vector3D<i32>; 3] = [Vector3D::new(0, 0, 0);3];
            let mut world_coords: [Vector3D<f32>; 3] = [Vector3D::new(0.0, 0.0, 0.0);3];
            for i in 0..3 {

                let v = model.verticates[face[i] as usize];
                screen_coords[i] = Vector3D::new(
                    ((v.x + 1.) * WIDTH as f32 / SIZE).round() as i32,
                    ((v.y + 1.) * HEIGHT as f32 / SIZE).round() as i32,
                    ((v.z + 1.) * DEPTH  as f32  / SIZE).round() as i32
                );
                world_coords[i] = v;
            }
            let n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);

            let norm = n.normalize(1.0);

            let intensity = norm * light_dir;
            
            if intensity > 0.0 {
                scene.triangle_3d(screen_coords, get_gray(intensity));
            }
        }
    });
}

fn get_gray(intensity: f32) -> u32 {
    let mut result = (255.0*intensity) as u32;
    result += (255.0*intensity) as u32*256;
    result += (255.0*intensity) as u32*256*256;
    return result;
}