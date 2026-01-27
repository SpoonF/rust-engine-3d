use std::{fs::{self, File}, io::{BufRead, BufReader, Read}, path::Path};

use image::{GenericImageView, ImageBuffer};

use crate::{geometry::{Vector2D, Vector3D}, tga::Tga};

#[derive(Clone)]
pub struct Model {
    pub verticates: Vec<Vector3D<f32>>,
    pub faces: Vec<Vec<Vector3D<i32>>>,
    pub uv: Vec<Vector2D<f32>>,
    pub norms: Vec<Vector3D<f32>>,
    pub texture: Option<Tga>
}

impl Model {
    pub fn read(path: &Path) -> Model {
        let file= File::open(path).unwrap();

        let reader = BufReader::new(file);

        let mut verticates = vec![];
        let mut faces = vec![];
        let mut uv = vec![];
        let mut norms = vec![];
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
            } else if line.starts_with("vn ") {
                let parts: Vec<&str> = line.split_whitespace().collect();

                norms.push(
                    Vector3D::new(
                        parts[1].parse::<f32>().unwrap(), 
                        parts[2].parse::<f32>().unwrap(),
                        parts[3].parse::<f32>().unwrap(),
                    )
                );
            }
        }

        Model {
            verticates,
            faces,
            uv,
            norms,
            texture: None,
        }
    }
    pub fn read_texture(&mut self, path: &Path) {
        self.texture = Some(Tga::read_file(path));
        
    }
    pub fn diffuse(&self, mut uv: Vector2D<i32>) -> u32 {
        let texture = self.texture.as_ref().unwrap();

        if texture.width() < uv.x as usize {
            uv.x = (texture.width() - 1) as i32;
        }
        if texture.height() < uv.y as usize {
            uv.y = (texture.height() - 1) as i32;
        }
        texture.get_pixel(uv.x, uv.y)
    }
    pub fn uv(&self, iface: i32, nvert: i32) -> Vector2D<i32> {
        let idx = self.faces[iface as usize][nvert as usize].y as usize;
        let texture = self.texture.as_ref().unwrap();
        Vector2D::new(
            self.uv[idx].x * texture.width() as f32,  
            self.uv[idx].y * texture.height() as f32
        ).cast()

    }
    pub fn norm(&self, iface: i32, nvert: i32) -> Vector3D<f32>{
        let idx = self.faces[iface as usize][nvert as usize][2];
        self.norms[idx as usize].normalize(1.0)
    }
}
impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model").field("verticates", &self.verticates).field("faces", &self.faces).finish()
    }
}