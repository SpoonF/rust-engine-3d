
use sdl2::{Sdl, event::Event, keyboard::Keycode, pixels::{Color, PixelFormatEnum}, rect::Rect, render::{Canvas, TextureCreator}, video::{Window, WindowContext} };
use std::mem;
use crate::{Shader, geometry::{Matrix, Vector}, model::Model};
pub struct Scene {
    scene: Vec<Vec<u32>>,
    zbuffer: Vec<f32>,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    sdl_context: Sdl,
    width: usize,
    height: usize,
}

impl Scene {
    pub fn new(width: usize, height: usize, depth: usize) -> Scene {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("demo", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let  texture_creator = canvas.texture_creator();

        

        canvas.set_draw_color(Color::RGB(0, 0, 12));
        canvas.clear();

        Scene {
            scene: vec![vec![0; width]; height],
            zbuffer: vec![f32::MIN; width*height],
            canvas,
            texture_creator,
            sdl_context,
            width,
            height,
        }
    }
    pub fn update(&mut self) {

        let mut scene_render = self.texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 
                                       self.width as u32, self.height as u32).unwrap();
        scene_render.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..self.height {
                for x in 0..self.width {
                    let offset = y*pitch + x*3;
                    let color = self.scene[x][self.height - y - 1];
                    buffer[offset + 0] = (color >> (8*2)) as u8;
                    buffer[offset + 1] = (color >> (8*1)) as u8;
                    buffer[offset + 2] = color as u8;
                }
            }
        }).unwrap();

        self.canvas.clear();
        self.canvas.copy(&scene_render, None, Some(Rect::new(0, 0, 
                                                self.width as u32, self.height as u32))).unwrap();
        self.canvas.present();

    }

    pub fn set(&mut self, x: usize, y: usize, color: u32) {
        self.scene[x][y] = color;
    }
    pub fn triangle(&mut self, clipc: &Matrix<4, 3>, shader: &Shader, viewport: &Matrix<4, 4>) {
        let pts: Matrix<3, 4> = (viewport.clone() * clipc.clone()).trunspose();
        let mut pts2: Matrix<3, 2> = Matrix::new();

        for i in 0..3 {
            pts2[i] = (pts[i] / pts[i][3]).proj::<2>();
        }                
        println!("{:?}", clipc.clone());

        let mut bboxmin = Vector::new([f32::MAX, f32::MAX]);
        let mut bboxmax = Vector::new([-f32::MAX, -f32::MAX]);
        let clamp = Vector::new([(self.width - 1) as f32, (self.height - 1) as f32]);

        for i in 0..3 {
            for j in 0..2 {
                bboxmin[j] = 0.0_f32.max(bboxmin[j].min(pts2[i][j]));
                bboxmax[j] = clamp[j].min(bboxmax[j].max(pts2[i][j]));
            }
        }

        let mut p = Vector::new([bboxmin[0], bboxmin[1]]);
        let mut color = 0;


        while p[0] <= bboxmax[0] {
            while p[1] <= bboxmax[1] {
                let bc_screen = barycentric(pts2[0], pts2[1], pts2[2], p);
                let mut bc_clip = Vector::new([bc_screen[0]/pts[0][3], bc_screen[1]/pts[1][3], bc_screen[2]/pts[2][3]]);
                bc_clip = bc_clip/(bc_clip[1] + bc_clip[1] + bc_clip[0]);
                let frag_depth = clipc[2] * bc_clip;

                if (bc_screen[0] < 0.0 || 
                        bc_screen[1] < 0.0 || 
                            bc_screen[2] < 0.0 || 
                                self.zbuffer[(p[0] + p[1] * self.width as f32) as usize] > frag_depth) {
                        continue;
                    }
                let discard = shader.fragment(bc_clip, &mut color);
                if !discard {
                    self.zbuffer[(p[0] + p[1] * self.width as f32) as usize] = frag_depth;
                    self.set(p[0] as usize, p[1] as usize, color);
                }

                p[1] += 1.0;
            }
            p[0] += 1.0;
        }
    }
    pub fn wait_for_exit(&mut self, mut action: impl FnMut(&mut Scene, Vec<Keycode>)) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        // let mut rng = rand::rng();
        'running: loop {
            let mut keys:Vec<Keycode> = vec![];
            let mut mouse: Vec<Keycode> = vec![];

            for event in event_pump.poll_iter() {
                
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                        break 'running;
                    },
                    Event::KeyDown {keycode: Some(key), ..} => {
                        keys.push(key);
                    },
                    _ => {}
                }
            }

            action(self, keys);
            self.update();
        }
    }
}

fn get_color(color: u32, mut intensity: f32) -> u32{
    intensity = intensity.clamp(0.0, 1.0); 

    let mut result = (color as f32*intensity) as u32;
    result += (color as f32*intensity) as u32*256;
    result += (color as f32*intensity) as u32*256*256;
    result
}
fn get_color_from_rgb(color: [u8; 3], intensity: f32) -> u32{
    let mut result = (color[2] as f32*intensity) as u32;
    result += (color[1] as f32*intensity) as u32*256;
    result += (color[0] as f32*intensity) as u32*256*256;
    result
}

fn barycentric(a: Vector<2, f32>, b: Vector<2, f32>, c: Vector<2, f32>, p: Vector<2, f32>) -> Vector<3, f32> {
    let mut s: [Vector<3, f32>; 2] = [Vector::empty(), Vector::empty()];

    for i in 0..2 {
        s[i][0] = c[i] - a[i];
        s[i][1] = b[i] - a[i];
        s[i][2] = a[i] - p[i];
    }
    let u = cross(s[0], s[1]);

    if u[2].abs() > 1e-2 {
        Vector::new([1.0-(u[0] + u[1])/u[2], u[1]/u[2], u[0]/u[2]])
    } else {
        Vector::new([-1.0, 1.0, 1.0])
    }
}

fn cross(v1: Vector<3, f32>, v2: Vector<3, f32>) -> Vector<3, f32> {
    Vector::new([
        v1[1] * v2[2] - v1[2] * v2[1],
        -(v1[0] * v2[2] - v1[2] * v2[0]),
        v1[0] * v2[1] - v1[1] * v2[0]
    ])
}