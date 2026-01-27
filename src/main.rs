
mod model;
mod geometry;
mod scene;
mod tga;

use std::{any::Any, path::Path};
use crate::{geometry::{ Matrix, Vector2D, Vector3D}, model::Model, scene::Scene};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const DEPTH: usize = 255;
const SIZE: f32 = 2.0;



fn main() {
    let mut model = Model::read(Path::new("obj/head.obj"));
    model.read_texture(Path::new("obj/head.tga"));
    // let light_dir = Vector3D::new(1.0, -1.0, 1.0).normalize(1.0);
    let light_dir: Vector3D<f32> = Vector3D::new(1., -1., 1.).normalize(1.0);

    let eye: Vector3D<f32> = Vector3D::new(1.0, 1.0, 3.0);
    let center: Vector3D<f32> = Vector3D::new(0.0, 0.0, 0.0);

    let model_view = look_at(eye, center, Vector3D::new(0.0, 1.0, 0.0));
    let mut projection = Matrix:: identity(4);
    let viewport = viewport((WIDTH/8) as i32, (HEIGHT/8) as i32, (WIDTH*3/4) as i32, (HEIGHT*3/4) as i32);
    projection[3][2] = -1./(eye-center).norm();


    let mut scene = Scene::new(WIDTH, HEIGHT, DEPTH);

    let faces = &model.faces;

    for i in 0..faces.len() {
        let face = &faces[i];
        let mut screen_coords: [Vector3D<i32>; 3] = [Vector3D::new(0, 0, 0);3];
        let mut world_coords: [Vector3D<f32>; 3] = [Vector3D::new(0.0, 0.0, 0.0);3];
        let mut intensity: [f32; 3] = [0.0; 3];
        for j in 0..3 {

            let v: Vector3D<f32> = model.verticates[face[j].x as usize];
            let set = viewport.clone() * projection.clone() * model_view.clone() * Matrix::from(v);
            screen_coords[j] = Vector3D::from(set).cast();
            world_coords[j] = v;

            intensity[j] = model.norm(i as i32, j as i32) * light_dir;
        }
        scene.triangle_new(screen_coords,  intensity);
    }
    scene.update();
    println!("updated:");
    scene.wait_for_exit(|scene: &mut Scene| {

    });
}

fn viewport(x: i32, y: i32, w: i32, h: i32) -> Matrix {
    let mut m = Matrix::identity(4);
    m[0][3] = (x + w) as f32 / 2.0;
    m[1][3] = (y + h) as f32 / 2.0;
    m[2][3] = DEPTH as f32 / 2.0;

    m[0][0] = w as f32 / 2.0;
    m[1][1] = h as f32 / 2.0;
    m[2][2] = DEPTH as f32 / 2.0;

    m
}

fn look_at(eye: Vector3D<f32>, center: Vector3D<f32>, up: Vector3D<f32>) -> Matrix {
    let z: Vector3D<f32> =  (eye - center).normalize(1.0);
    let x: Vector3D<f32> =  cross(up, z).normalize(1.0);
    let y: Vector3D<f32> =  cross(z, x).normalize(1.0);

    let mut minv = Matrix::identity(4);
    let mut tr = Matrix::identity(4);

    for i in 0..3 {
        minv[0][i] = x[i];
        minv[1][i] = y[i];
        minv[2][i] = z[i];
        tr[i][3] = -center[i];
    }

    minv * tr
}

fn cross(v1: Vector3D<f32>, v2: Vector3D<f32>) -> Vector3D<f32> {
    Vector3D::new(
        v1.y * v2.z - v1.z * v2.y,
        -(v1.x * v2.z - v1.z * v2.x),
        v1.x * v2.y - v1.y * v2.x
    )
}