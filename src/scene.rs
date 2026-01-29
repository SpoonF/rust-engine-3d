
use sdl2::{Sdl, event::Event, keyboard::Keycode, pixels::{Color, PixelFormatEnum}, rect::Rect, render::{Canvas, TextureCreator}, video::{Window, WindowContext} };
use std::mem;
use crate::{geometry::{Vector}, model::Model};
pub struct Scene {
    scene: Vec<Vec<u32>>,
    zbuffer: Vec<i32>,
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
            zbuffer: vec![i32::MIN; width*height],
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
    pub fn triangle(&mut self, mut p: [Vector<2,f32>; 3], color: u32) {
        if p[0][1]==p[1][1] && p[0][1]==p[2][1] {return}; // i dont care about degenerate triangles
        // sort the vertices, p[0], p[1], p[2] lower-to-upper (bubblesort yay!)
        if p[0][1]>p[1][1] { p.swap(0, 1)};
        if p[0][1]>p[2][1] { p.swap(0, 2)};
        if p[1][1]>p[2][1] { p.swap(1, 2)};

        let total_height = p[2][1]-p[0][1];

        for i in 0..total_height as i32 {
            let second_half = i as f32 > p[1][1] - p[0][1] || p[1][1] == p[0][1];
            let segment_height = if second_half { p[2][1] - p[1][1] } else { p[1][1] - p[0][1] };

            let alpha = i as f32 /total_height ;
            let beta  = (i as f32 - ( if second_half {p[1][1] - p[0][1] } else { 0.0 }))/segment_height; // be careful: with above conditions no division by zero here
            
            let mut a = (p[0] + (p[2]-p[0])*alpha).cast::<i32>();
            let mut b = (if second_half  {p[1] + (p[2]-p[1])*beta} else { p[0] + (p[1]-p[0])*beta }).cast::<i32>();

            if a[0] > b[0] {mem::swap(&mut a, &mut b)};
            for j in a[0]..=b[0] {
                self.scene[j as usize][p[0][1] as usize] = color;
            }
        }
    }
    pub fn triangle_3d(&mut self, mut p: [Vector<3,i32>; 3], mut uv: [Vector<2,i32>; 3], intesity: f32, model: &Model) {
        if p[0][1] == p[1][1] && p[0][1] == p[2][1] {
            return
        }; 
        if p[0][1]>p[1][1] { p.swap(0, 1); uv.swap(0, 1);};
        if p[0][1]>p[2][1] { p.swap(0, 2); uv.swap(0, 2);};
        if p[1][1]>p[2][1] { p.swap(1, 2); uv.swap(1, 2);};

        let total_height = p[2][1]-p[0][1];

        for i in 0..total_height {
            let second_half = i > p[1][1] - p[0][1] || p[1][1] == p[0][1];
            let segment_height = if second_half { p[2][1] - p[1][1] } else { p[1][1] - p[0][1] };

            let alpha = i as f32 / total_height  as f32;
            let beta  = (i - ( if second_half { p[1][1] - p[0][1] } else { 0 })) as f32 / segment_height as f32; // be careful: with above conditions no division by zero here
            
            let mut a: Vector<3,f32> = p[0].cast() + (p[2]-p[0]).cast() * alpha;
            let mut b: Vector<3,f32> = if second_half { 
                p[1].cast() + (p[2]-p[1]).cast() * beta
            } else { 
                p[0].cast() + (p[1]-p[0]).cast() * beta
            };

            let mut uva: Vector<2,f32> = uv[0].cast() + (uv[2] - uv[0]).cast() * alpha;
            let mut uvb: Vector<2,f32> = if second_half {  
                uv[1].cast() + (uv[2] - uv[1]).cast() * beta
            } else {
                uv[0].cast() + (uv[1] - uv[0]).cast() * beta
            };

            if a[0] > b[0] {
                mem::swap(&mut a, &mut b); 
                mem::swap(&mut uva, &mut uvb)
            };


            for j in a[0] as i32..=b[0] as i32 {
                let phi = if b[0] == a[0] { 1. } else { (j as f32 - a[0]) / (b[0] - a[0])};
                let pp: Vector<3,i32> = (a + ((b - a) * phi)).round();
                let uvpp: Vector<2,f32> = uva + ((uvb - uva) * phi);

                let idx = (pp[0] + pp[1] * self.width as i32) as usize;

                if self.zbuffer[idx] < pp[2] {
                    self.zbuffer[idx] = pp[2];

                    let color = model.diffuse(uvpp);
                    self.set(pp[0] as usize, pp[1] as usize, get_color(color, intesity));
                }
            }
        }
    }
    pub fn triangle_new(&mut self, mut p: [Vector<3,i32>; 3], mut ity: [f32; 3]) {
        if p[0][1] == p[1][1] && p[0][1] == p[2][1] {
            return
        }; 
        if p[0][1]>p[1][1] { p.swap(0, 1); ity.swap(0, 1);};
        if p[0][1]>p[2][1] { p.swap(0, 2); ity.swap(0, 2);};
        if p[1][1]>p[2][1] { p.swap(1, 2); ity.swap(1, 2);};

        let total_height = p[2][1]-p[0][1];

        for i in 0..total_height {
            let second_half = i > p[1][1] - p[0][1] || p[1][1] == p[0][1];
            let segment_height = if second_half { p[2][1] - p[1][1] } else { p[1][1] - p[0][1] };

            let alpha = i as f32 / total_height  as f32;
            let beta  = (i - ( if second_half { p[1][1] - p[0][1] } else { 0 })) as f32 / segment_height as f32; // be careful: with above conditions no division by zero here
            
            let mut a: Vector<3,f32> = p[0].cast() + (p[2]-p[0]).cast() * alpha;
            let mut b: Vector<3,f32> = if second_half { 
                p[1].cast() + (p[2]-p[1]).cast() * beta
            } else { 
                p[0].cast() + (p[1]-p[0]).cast() * beta
            };

            let mut itya = ity[0] + (ity[2] - ity[0]) * alpha;
            let mut ityb = if second_half {  
                ity[1] + (ity[2] - ity[1]) * beta
            } else {
                ity[0] + (ity[1] - ity[0]) * beta
            };

            if a[0] > b[0] {
                mem::swap(&mut a, &mut b); 
                mem::swap(&mut itya, &mut ityb)
            };
            for j in a[0] as i32..=b[0] as i32 {
                let phi = if b[0] == a[0] { 1. } else { (j as f32 - a[0]) / (b[0] - a[0])};
                let p: Vector<3,i32> = (a + ((b - a) * phi)).round();
                let ityp = itya + ((ityb - itya) * phi);

                let idx = (p[0] + p[1] * self.width as i32) as usize;


                if p[0] >= self.width as i32 || p[1] >= self.height as i32 || p[0] < 0 || p[1] < 0 {
                    continue;
                }
                if self.zbuffer[idx] < p[2] {
                    self.zbuffer[idx] = p[2];

                    self.set(p[0] as usize, p[1] as usize, get_color(255, ityp));
                }
            }
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

