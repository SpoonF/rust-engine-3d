
use sdl2::{Sdl, event::Event, keyboard::Keycode, pixels::{Color, PixelFormatEnum}, rect::Rect, render::{Canvas, TextureCreator}, video::{Window, WindowContext} };
use std::mem;
use crate::{Shader, geometry::{Matrix, Vector}, model::Model, our_gl::utils::{barycentric, get_color_rgba, get_rgba}};
pub struct Scene {
    pub scene: Vec<Vec<u32>>,
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
    pub fn triangle(&mut self, pts: Vec<Vector<4, f32>>, shader: &Shader, viewport: &Matrix<4, 4>) {        

        let mut bboxmin = Vector::new([f32::MAX, f32::MAX]);
        let mut bboxmax = Vector::new([-f32::MAX, -f32::MAX]);
        
        for i in 0..3 {
            for j in 0..2 {
                bboxmin[j] = bboxmin[j].min(pts[i][j] / pts[i][3]);
                bboxmax[j] = bboxmax[j].max(pts[i][j] / pts[i][3]);
            }
        }

        let mut p = Vector::new([bboxmin[0], bboxmin[1]]);
        let mut color = get_rgba(0xFFFFFF);


        while p[0] <= bboxmax[0] {
            while p[1] <= bboxmax[1] {
                let c = barycentric(
                    (pts[0]/pts[0][3]).proj::<2>(), 
                    (pts[1]/pts[1][3]).proj::<2>(), 
                    (pts[2]/pts[2][3]).proj::<2>(), 
                    p
                );

                let z = pts[0][2] * c.x() + pts[1][2] * c.y() + pts[2][2] * c.z();
                let w = pts[0][3] * c.x() + pts[1][3] * c.y() + pts[2][3] * c.z();

                let frag_depth = z / w;
                
                if c[0] < 0.0 || 
                        c[1] < 0.0 || 
                            c[2] < 0.0 || 
                                self.zbuffer[(p[0] + p[1] * self.width as f32) as usize] > frag_depth 
                {
                    p[1] += 1.0;
                    continue;
                }

                let discard = shader.fragment(c, &mut color);
                
                if !discard {
                    self.zbuffer[(p[0] + p[1] * self.width as f32) as usize] = frag_depth;
                    self.set(p[0] as usize, p[1] as usize, get_color_rgba(color));
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
            let mouse: Vec<Keycode> = vec![];

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
            // self.update();
        }
    }
}

fn cross(v1: Vector<3, f32>, v2: Vector<3, f32>) -> Vector<3, f32> {
    Vector::new([
        v1[1] * v2[2] - v1[2] * v2[1],
        -(v1[0] * v2[2] - v1[2] * v2[0]),
        v1[0] * v2[1] - v1[1] * v2[0]
    ])
}