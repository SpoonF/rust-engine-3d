
mod model;
mod geometry;

use std::{path::Path};

use sdl2::{event::Event, keyboard::Keycode, pixels::{Color, PixelFormatEnum}, rect::{Point, Rect}, render::Canvas, video::Window};
use rand::prelude::*;
use std::ops::{Add, BitXor, Mul, Sub};

use crate::{geometry::{Vector2D, Vector3D}, model::Model};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const SIZE: f32 = 2.0;



fn main() {
    let model = Model::read(Path::new("obj/head.obj"));
    // println!("{:?}", model);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 12));
    canvas.clear();

    let light_dir = Vector3D::new(0.0, 0.0, -1.0);

    let faces = model.faces;

    for face in faces {
        let mut screen_coords: [Vector2D<f32>; 3] = [Vector2D::new(0.0, 0.0);3];
        let mut world_coords: [Vector3D<f32>; 3] = [Vector3D::new(0.0, 0.0, 0.0);3];
        for i in 0..3 {

            let v = model.verticates[face[i] as usize];
            screen_coords[i] = Vector2D::new(
                (v.x + 1.) * WIDTH as f32 / SIZE,
                (v.y + 1.) * HEIGHT as f32 / SIZE,
            );
            world_coords[i] = v;
        }
        let n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);

        let norm = n.normalize(1.0);

        let intensity = norm * light_dir;
        
        if intensity > 0.0 {
            let color = (intensity * 255.0) as u8;
            triangle(&mut canvas, screen_coords, Color::RGB(color, color, color));
        }
        
    }


    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut rng = rand::rng();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                _ => {}
            }
        }
    }
    // canvas.present();
}

fn line(canvas: &mut Canvas<Window>, p: Vec<Vector2D<i32>>, color: Color, y_buffer: &[i32]) {
    let mut points: Vec<Point> = vec![];

    _line_math(p[0], p[1], &mut points);

    canvas.set_draw_color(color);
    canvas.draw_points(&points[..]).unwrap();
}

fn _line_math(mut p0: Vector2D<i32>, mut p1: Vector2D<i32>, points: &mut Vec<Point>) {
    let mut steep = false;
    
    if (p0.x - p1.x).abs() < (p0.y - p1.y).abs() {
        std::mem::swap(&mut p0.x, &mut p0.y);
        std::mem::swap(&mut p1.x, &mut p1.y);
        steep = true;
    }

    if p0.x > p1.x {
        std::mem::swap(&mut p0.x, &mut p1.x);
        std::mem::swap(&mut p0.y, &mut p1.y);
    }
    let dx = p1.x-p0.x;
    let dy = p1.y-p0.y;

    let derror = dy.abs() * 2;
    let mut error = 0;

    let mut y = p0.y;
    let mut x = p0.x;
    while x <= p1.x {
        if steep {
            points.push(Point::new(y, x ));
        } else {
            points.push(Point::new(x, y));
        }

        x += 1;
        error += derror;

        if error > dx {
            y += if p1.y > p0.y { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

fn triangle(canvas: &mut Canvas<Window>, p: [Vector2D<f32>; 3], color: Color) {
    let mut points: Vec<Point> = vec![];


    _triangle_math(p, &mut points);

    canvas.set_draw_color(color);
    canvas.draw_points(&points[..]).unwrap();
}

fn _triangle_math(mut p: [Vector2D<f32>; 3], points: &mut Vec<Point>) {
    if p[0].y==p[1].y && p[0].y==p[2].y {return}; // i dont care about degenerate triangles
    // sort the vertices, p[0], p[1], p[2] lower-to-upper (bubblesort yay!)
    if p[0].y>p[1].y { p.swap(0, 1)};
    if p[0].y>p[2].y { p.swap(0, 2)};
    if p[1].y>p[2].y { p.swap(1, 2)};

    let total_height = p[2].y-p[0].y;

    for i in 0..total_height as i32 {
        let second_half = i as f32 > p[1].y - p[0].y || p[1].y == p[0].y;
        let segment_height = if second_half { p[2].y - p[1].y } else { p[1].y - p[0].y };
        let alpha = i as f32 /total_height ;
        let beta  = (i as f32 - ( if second_half {p[1].y - p[0].y } else { 0.0 }))/segment_height; // be careful: with above conditions no division by zero here
        let mut a = (p[0] + (p[2]-p[0])*alpha).cast::<i32>();
        let mut b = (if second_half  {p[1] + (p[2]-p[1])*beta} else { p[0] + (p[1]-p[0])*beta }).cast::<i32>();
        if a.x > b.x {std::mem::swap(&mut a, &mut b)};
        for j in a.x..(b.x + 1) {
            points.push(Point::new(j, p[0].y as i32 + i));
        }
    }
}

#[test]
fn _test_line() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 12));
    canvas.clear();

    let y_buffer = [i32::MIN; WIDTH];

    line(&mut canvas, vec![Vector2D::new(20, 34), Vector2D::new(744, 400)], Color::RED, y_buffer);
    line(&mut canvas, vec![Vector2D::new(120, 434), Vector2D::new(444, 400)], Color::GREEN, y_buffer);
    line(&mut canvas, vec![Vector2D::new(330, 463), Vector2D::new(594, 200)], Color::BLUE, y_buffer);


    line(&mut canvas, vec![Vector2D::new(10, 10), Vector2D::new(790, 10)], Color::WHITE);
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                _ => {}
            }
        }
    }
}