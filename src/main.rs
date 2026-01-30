
mod model;
mod geometry;
mod scene;
mod tga;
mod shader;

use std::{any::Any, path::Path};
use image::Delay;
use sdl2::keyboard::{Keycode, Mod};

use crate::{geometry::{ Matrix, Vector}, model::Model, scene::Scene};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const DEPTH: usize = 255;
const SIZE: f32 = 2.0;



fn main() {
    let mut model = Model::read(Path::new("obj/head.obj"));
    model.read_texture(Path::new("obj/head.tga"));
    // let light_dir = Vector3D::new(1.0, -1.0, 1.0).normalize(1.0);

    let light_dir: Vector<3,f32> = Vector::new([1., -1., 1.]).normalize(1.);
    let eye: Vector<3,f32> = Vector::new([1.0, 1.0, 3.0]);
    let center: Vector<3,f32> = Vector::new([0.0, 0.0, 0.0]);
    let up: Vector<3,f32> = Vector::new([0.0, 1.0, 0.0]);

    
    let model_view: Matrix<4, 4> = look_at(eye, center, up);
    let viewport: Matrix<4, 4> = viewport((WIDTH/8) as i32, (HEIGHT/8) as i32, (WIDTH*3/4) as i32, (HEIGHT*3/4) as i32);
    let projection: Matrix<4, 4> = projection(-1./(eye-center).norm());
    let light_dir = (projection.clone() * model_view.clone() * light_dir.embed::<4>(0.0)).proj::<3>().normalize(1.);

    

    let mut scene = Scene::new(WIDTH, HEIGHT, DEPTH);

    scene.wait_for_exit(|scene: &mut Scene, keycodes| {


        let mut shader = Shader::new(&model, &projection, &model_view);
        let faces = &model.faces;

        for i in 0..faces.len() {
            for j in 0..3 {
                shader.vertex(i, j);
            }
            scene.triangle(&shader.varing_tri,  &shader, &viewport);
        }
    });
}

fn viewport(x: i32, y: i32, w: i32, h: i32) -> Matrix<4, 4> {
    let mut m: Matrix<4, 4> = Matrix::identity();
    m[0][3] = (x + w) as f32 / 2.0;
    m[1][3] = (y + h) as f32 / 2.0;
    m[2][3] = DEPTH as f32 / 2.0;

    m[0][0] = w as f32 / 2.0;
    m[1][1] = h as f32 / 2.0;
    m[2][2] = DEPTH as f32 / 2.0;

    m
}

fn look_at(_eye: Vector<3, f32>, _center: Vector<3, f32>, _up: Vector<3, f32>) -> Matrix<4, 4> {
    let z  =  (_eye - _center).normalize(1.0);
    let x =  cross(_up, z).normalize(1.0);
    let y =  cross(z, x).normalize(1.0);

    let mut minv: Matrix<4, 4> = Matrix::identity();
    let mut tr: Matrix<4, 4> = Matrix::identity();

    for i in 0..3 {
        minv[0][i] = x[i];
        minv[1][i] = y[i];
        minv[2][i] = z[i];
        tr[i][3] = -_center[i];
    }

    minv * tr
}

fn cross(v1: Vector<3, f32>, v2: Vector<3, f32>) -> Vector<3, f32> {
    Vector::new([
        v1[1] * v2[2] - v1[2] * v2[1],
        -(v1[0] * v2[2] - v1[2] * v2[0]),
        v1[0] * v2[1] - v1[1] * v2[0]
    ])
}
fn projection(coef: f32) -> Matrix<4, 4>{
    let mut result: Matrix<4, 4> = Matrix::identity();
    result[3][2] = coef;
    result
}

struct Shader<'a> {
    varing_uv: Matrix<2, 3>,
    varing_tri: Matrix<4, 3>,
    model: &'a Model,
    projection: &'a Matrix<4,4>,
    model_view: &'a Matrix<4,4>,
}

impl<'a>  Shader<'_> {
    pub fn new(model: &'a Model, projection: &'a Matrix<4,4>, model_view: &'a Matrix<4,4>) -> Shader<'a> {
        Shader {
            varing_uv: Matrix::new(),
            varing_tri: Matrix::new(),
            model,
            projection,
            model_view
        }
    }
    
    pub fn vertex(&mut self, iface: usize, nthvert: usize) -> Vector<4,f32> {
        self.varing_uv.set_col(nthvert, self.model.uv(iface, nthvert).cast());
        let gl_vertex: Vector<4, f32> = self.projection.clone() * self.model_view.clone() * self.model.vert(iface, nthvert).embed::<4>(1.0);
        self.varing_tri.set_col(nthvert, gl_vertex);
        gl_vertex
    }   
    pub fn fragment(&self, bar: Vector<3, f32>, color: &mut u32) -> bool {
        let uv = self.varing_uv.clone() * bar;
        *color = self.model.diffuse(uv);
        false
    }
}