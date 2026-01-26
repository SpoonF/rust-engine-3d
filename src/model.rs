use std::{fs::{self, File}, io::{BufRead, BufReader, Read}, path::Path};

use image::{GenericImageView, ImageBuffer};

use crate::{geometry::{Vector2D, Vector3D}, tga::Tga};

#[derive(Clone)]
pub struct Model {
    pub verticates: Vec<Vector3D<f32>>,
    pub faces: Vec<Vec<Vector3D<i32>>>,
    pub uv: Vec<Vector2D<f32>>,
    texture: Option<Tga>,
    pub texture_alt: Option<ImageBuffer<image::Rgb<u8>, Vec<u8>>>,
}

impl Model {
    pub fn read(path: &Path) -> Model {
        let file= File::open(path).unwrap();

        let reader = BufReader::new(file);

        let mut verticates = vec![];
        let mut faces = vec![];
        let mut uv = vec![];
        // println!("{:?}", buffer);
        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("v ") {
                let parts: Vec<&str> = line.split_whitespace().collect();

                verticates.push(
                    Vector3D::new(
                        parts[1].parse::<f32>().unwrap(), 
                        parts[2].parse::<f32>().unwrap(), 
                        parts[3].parse::<f32>().unwrap() 
                    )
                );
            } else if line.starts_with("f ") {
                let mut parts: Vec<&str> = line.split_whitespace().collect();
                parts.remove(0);
                let mut x = vec![];
                
                for part in parts {
                    let t: Vec<&str> = part.split("/").collect();
                    // let xd = t[0].parse::<i32>().unwrap() - 1;
                    x.push(Vector3D::new(
                        t[0].parse::<i32>().unwrap() - 1, 
                        t[1].parse::<i32>().unwrap() - 1, 
                        t[2].parse::<i32>().unwrap() - 1 
                    ));
                }
                faces.push(x);
            } else if line.starts_with("vt ") {
                let parts: Vec<&str> = line.split_whitespace().collect();

                uv.push(
                    Vector2D::new(
                        parts[1].parse::<f32>().unwrap(), 
                        parts[2].parse::<f32>().unwrap(),
                    )
                );
            }
        }

        Model {
            verticates,
            faces,
            uv,
            texture: None,
            texture_alt: None
        }
    }
    pub fn read_texture(&mut self, path: &Path) {
        let mut img = image::open(path).unwrap();
        let img = img.rotate180();
        let texture = Tga::read_file(path);
        let texture_alt = img.to_rgb8();
        self.texture = Some(texture);
        self.texture_alt = Some(texture_alt.clone());
        
    }
    pub fn diffuse(&self, uv: Vector2D<i32>) -> u32 {
        let texture = self.texture.as_ref().unwrap();
        let x = if uv.x > texture.width() as i32 { texture.width() as i32 - 1 } else { uv.x };
        let y = if uv.y > texture.height() as i32 { texture.height() as i32 - 1 } else { uv.y };
        texture.get_pixel(x, y)
    }
    pub fn uv(&self, iface: i32, nvert: i32) -> Vector2D<i32> {
        let idx = self.faces[iface as usize][nvert as usize].y as usize;
        let texture = self.texture.as_ref().unwrap();
        Vector2D::new(
            self.uv[idx].x * texture.width() as f32,  
            self.uv[idx].y * texture.height() as f32
        ).cast()
    }
}
impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model").field("verticates", &self.verticates).field("faces", &self.faces).finish()
    }
}