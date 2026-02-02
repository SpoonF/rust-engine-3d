mod model;
mod geometry;
mod scene;
mod tga;
mod our_gl;

use std::{path::Path};

use crate::{geometry::{ Matrix, Vector}, model::Model, our_gl::{Shader, utils::{look_at, projection, viewport}}, scene::Scene};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const DEPTH: usize = 255;
const SIZE: f32 = 2.0;



fn main() {
    let mut model = Model::read(Path::new("obj/head.obj"));
    model.read_texture(Path::new("obj/head.tga"));
    // let light_dir = Vector3D::new(1.0, -1.0, 1.0).normalize(1.0);

    let light_dir: Vector<3,f32> = Vector::new([1., 1., 0.0]);
    let eye: Vector<3,f32> = Vector::new([1.0, 1.0, 3.0]);
    let center: Vector<3,f32> = Vector::new([0.0, 0.0, 0.0]);
    let up: Vector<3,f32> = Vector::new([0.0, 1.0, 0.0]);

    
    let model_view: Matrix<4, 4> = look_at(eye, center, up);
    let viewport: Matrix<4, 4> = viewport((WIDTH/8) as i32, (HEIGHT/8) as i32, (WIDTH*3/4) as i32, (HEIGHT*3/4) as i32);
    let projection: Matrix<4, 4> = projection(-1./(eye-center).norm());
    let m = viewport * projection * model_view;
    // let light_dir = (projection.clone() * model_view.clone() * light_dir.embed::<4>(0.0)).proj::<3>().normalize(1.);



    let mut scene = Scene::new(WIDTH, HEIGHT, DEPTH);

    let mut shader = Shader::new(
        &model, 
        &projection, 
        &model_view, 
        &light_dir, 
        model_view, 
        (projection * model_view).invert_transpose(), 
        m * m.invert()
    );
    let mut screen_coords: Vec<Vector<4, f32>> = vec![Vector::empty(); 3];
    let faces = &model.faces;

    for i in 0..faces.len() {
        for j in 0..3 {
            screen_coords[j] = shader.vertex(i, j);
        }
        scene.triangle(screen_coords.clone(),  &shader, &viewport);
    }

    scene.update();

    scene.wait_for_exit(|scene: &mut Scene, keycodes| {


        // let mut shader = Shader::new(&model, &projection, &model_view);
        // let faces = &model.faces;

        // for i in 0..faces.len() {
        //     for j in 0..3 {
        //         shader.vertex(i, j);
        //     }
        //     scene.triangle(&shader.varing_tri,  &shader, &viewport);
        // }
    });
}

