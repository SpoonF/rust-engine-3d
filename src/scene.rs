use sdl2::{Sdl, event::Event, keyboard::Keycode, pixels::{Color, PixelFormat, PixelFormatEnum}, rect::Rect, render::{Canvas, Texture, TextureCreator}, surface::Surface, video::{Window, WindowContext} };
use std::mem;
use crate::{geometry::{Vector2D, Vector3D}, model::Model};
pub struct Scene {
    scene: Vec<Vec<u32>>,
    zbuffer: Vec<i32>,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    sdl_context: Sdl,
    width: usize,
    height: usize
}

impl Scene {
    pub fn new(width: usize, height: usize) -> Scene {
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
            height
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

    // pub fn line(&mut self, mut p: Vec<Vector2D<i32>>, color: Color) {
    //     let mut steep = false;
        
    //     if (p[0].x - p[1].x).abs() < (p[0].y - p[1].y).abs() {
    //         mem::swap(&mut p[0].x, &mut p[0].y);
    //         mem::swap(&mut p[1].x, &mut p[1].y);
    //         steep = true;
    //     }

    //     if p[0].x > p[1].x {
    //         mem::swap(&mut p[0].x, &mut p[1].x);
    //         mem::swap(&mut p[0].y, &mut p[1].y);
    //     }
    //     let dx = p[1].x-p[0].x;
    //     let dy = p[1].y-p[0].y;

    //     let derror = dy.abs() * 2;
    //     let mut error = 0;

    //     let mut y = p[0].y;
    //     let mut x = p[0].x;
    //     while x <= p[1].x {
    //         if steep {
    //             self.scene[x as usize][y as usize] = color;
    //         } else {
    //             self.scene[x as usize][y as usize] = color;
    //         }

    //         x += 1;
    //         error += derror;

    //         if error > dx {
    //             y += if p[1].y > p[0].y { 1 } else { -1 };
    //             error -= dx * 2;
    //         }
    //     }
    // }
    pub fn triangle(&mut self, mut p: [Vector2D<f32>; 3], color: u32) {
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

            if a.x > b.x {mem::swap(&mut a, &mut b)};
            for j in a.x..=b.x {
                self.scene[j as usize][p[0].y as usize] = color;
            }
        }
    }
    pub fn triangle_3d(&mut self, mut p: [Vector3D<i32>; 3], uv: [Vector2D<i32>; 3], intesity: f32, model: &Model) {
        if p[0].y == p[1].y && p[0].y == p[2].y {
            return
        }; 
        if p[0].y>p[1].y { p.swap(0, 1)};
        if p[0].y>p[2].y { p.swap(0, 2)};
        if p[1].y>p[2].y { p.swap(1, 2)};

        let total_height = p[2].y-p[0].y;

        for i in 0..total_height {
            let second_half = i > p[1].y - p[0].y || p[1].y == p[0].y;
            let segment_height = if second_half { p[2].y - p[1].y } else { p[1].y - p[0].y };

            let alpha = i as f32 / total_height  as f32;
            let beta  = (i - ( if second_half { p[1].y - p[0].y } else { 0 })) as f32 / segment_height as f32; // be careful: with above conditions no division by zero here
            
            let mut a: Vector3D<f32> = p[0].cast() + (p[2]-p[0]).cast() * alpha;
            let mut b: Vector3D<f32> = if second_half { 
                p[1].cast() + (p[2]-p[1]).cast() * beta
            } else { 
                p[0].cast() + (p[1]-p[0]).cast() * beta
            };

            let mut uva: Vector2D<f32> = uv[0].cast() + (uv[2] - uv[0]).cast() * alpha;
            let mut uvb: Vector2D<f32> = if second_half {  
                uv[1].cast() + (uv[2] - uv[1]).cast() * beta
            } else {
                uv[0].cast() + (uv[1] - uv[0]).cast() * beta
            };

            if a.x > b.x {
                mem::swap(&mut a, &mut b); 
                mem::swap(&mut uva, &mut uvb)
            };


            for j in a.x as i32..=b.x as i32 {
                let phi = if b.x == a.x { 1. } else { (j as f32 - a.x) / (b.x - a.x)};
                let pp: Vector3D<i32> = (a + ((b - a) * phi)).round();
                let uvpp: Vector2D<i32> = (uva + ((uvb - uva) * phi)).round();

                let idx = (pp.x + pp.y * self.width as i32) as usize;

                // println!("in: {:?}", uvpp);
                if self.zbuffer[idx] < pp.z {
                    self.zbuffer[idx] = pp.z;
                    // let color = model.diffuse(uvpp);
                    let texture_alt = model.texture_alt.as_ref().unwrap();
                    let color = texture_alt.get_pixel(pp.x as u32, pp.y as u32);
// let mut surface = Surface::new(1, 1, PixelFormatEnum::RGB888).unwrap();
// let pixel_format = surface.pixel_format();
                    // self.scene[pp.x as usize][pp.y as usize] = get_color_from_rgb(color as u32, intesity);
                    self.scene[pp.x as usize][pp.y as usize] = Color::RGB(color.0[0], color.0[1], color.0[2]).to_u32(&pixel_format);
                }
            }
        }
    }

    pub fn wait_for_exit(&mut self, action: impl Fn(&mut Scene)) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        // let mut rng = rand::rng();

        'running: loop {
            action(self);
            self.update();
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
}

fn get_color_from_rgb(color: u32, intensity: f32) -> u32{
    let mut result = ((color as u8) as f32*intensity) as u32;
    result += (((color >> 8) as u8) as f32*intensity) as u32*256;
    result += (((color >> 16) as u8) as f32*intensity) as u32*256*256;
    result
}