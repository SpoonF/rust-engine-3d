use std::{fs::{self, File}, io::{BufRead, BufReader, Read}, path::Path};

use image::{GenericImageView, ImageBuffer};

use crate::{geometry::{Vector}, tga::Tga};

#[derive(Clone)]
pub struct Model {
    pub verticates: Vec<Vector<3,f32>>,
    pub faces: Vec<Vec<Vector<3,i32>>>,
    pub uv: Vec<Vector<2,f32>>,
    pub norms: Vec<Vector<3,f32>>,
    pub diffusemap: Option<Tga>,
    pub normalmap: Option<Tga>,
    pub specularmap: Option<Tga>,
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
                    Vector::new([
                        parts[1].parse::<f32>().unwrap(), 
                        parts[2].parse::<f32>().unwrap(), 
                        parts[3].parse::<f32>().unwrap() 
                    ])
                );
            } else if line.starts_with("f ") {
                let mut parts: Vec<&str> = line.split_whitespace().collect();
                parts.remove(0);
                let mut x = vec![];
                
                for part in parts {
                    let t: Vec<&str> = part.split("/").collect();

                    x.push(Vector::new([
                        t[0].parse::<i32>().unwrap() - 1, 
                        t[1].parse::<i32>().unwrap() - 1, 
                        t[2].parse::<i32>().unwrap() - 1 
                    ]));
                }
                faces.push(x);
            } else if line.starts_with("vt ") {
                let parts: Vec<&str> = line.split_whitespace().collect();

                uv.push(
                    Vector::new([
                        parts[1].parse::<f32>().unwrap(), 
                        parts[2].parse::<f32>().unwrap(),
                    ])
                );
            } else if line.starts_with("vn ") {
                let parts: Vec<&str> = line.split_whitespace().collect();

                norms.push(
                    Vector::new([
                        parts[1].parse::<f32>().unwrap(), 
                        parts[2].parse::<f32>().unwrap(),
                        parts[3].parse::<f32>().unwrap(),
                    ])
                );
            }
        }

        Model {
            verticates,
            faces,
            uv,
            norms,
            diffusemap: None,
            normalmap: None,
            specularmap: None,
        }
    }
    pub fn read_texture(&mut self, path: &Path) {
        self.diffusemap = Some(Tga::read_file(path));
        
    }
    pub fn diffuse(&self, mut uvf: Vector<2, f32>) -> [u8; 4] {
        let texture = self.diffusemap.as_ref().unwrap();
        println!("{:?}", uvf);
        let uv = Vector::new([uvf[0] * texture.width() as f32, uvf[1] * texture.height() as f32]).cast::<i32>();
        println!("{:?}", uv);
        get_rgba(texture.get_pixel(uv[0], uv[1]))
    }
    pub fn uv(&self, iface: usize, nvert: usize) -> Vector<2, f32> {
        self.uv[self.faces[iface][nvert][1] as usize]

    }
    pub fn norm(&self, iface: usize, nvert: usize) -> Vector<3,f32>{
        let idx = self.faces[iface][nvert][2];
        self.norms[idx as usize].normalize(1.0)
    }
    pub fn vert(&self, iface: usize, nvert: usize) -> Vector<3,f32> {
        self.verticates[self.faces[iface][nvert][0] as usize]
    }
    
    pub fn normal(&self, uvf: Vector<2, f32>) -> Vector<3, f32> {
        
        if self.normalmap.is_some() {
            let normalpam = self.normalmap.as_ref().unwrap();

            let uv = Vector::new([
                uvf[0] * normalpam.width() as f32,
                uvf[1] * normalpam.height() as f32,
            ]).cast();

            let c = normalpam.get_pixel(uv[0], uv[1]);

            let mut res: Vector<3, f32> = Vector::empty();
            let rgba = get_rgba(c);

            for i in 0..3 {
                res[2-i] = rgba[i] as f32 / 255.0 * 2.0 - 1.0;
            }

            return res;
        }
        Vector::empty()        
    }
    pub fn specular(&self, uvf: Vector<2, f32>) -> f32 {
        if self.specularmap.is_some() {
            let specularmap = self.specularmap.as_ref().unwrap();

            let uv = Vector::new([
                uvf[0] * specularmap.width() as f32,
                uvf[1] * specularmap.height() as f32,
            ]).cast();

            return get_rgba(specularmap.get_pixel(uv[0], uv[1]))[0] as f32 / 1.0;
        }
        0.0
    }
}
impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model").field("verticates", &self.verticates).field("faces", &self.faces).finish()
    }
}

fn get_rgb(color: u32) -> [u8; 3] {
    let mut rgb: [u8; 3] = [0xFF; 3];
    rgb[0] = ((color >> 16) & 0xFF) as u8;
    rgb[1] = ((color >> 8) & 0xFF) as u8;
    rgb[2] = (color & 0xFF) as u8;

    rgb
}
fn get_rgba(color: u32) -> [u8; 4] {
    let mut rgba: [u8; 4] = [0xFF; 4];
    rgba[0] = ((color >> 24) & 0xFF) as u8;
    rgba[1] = ((color >> 16) & 0xFF) as u8;
    rgba[2] = ((color >> 8) & 0xFF) as u8;
    rgba[3] = (color & 0xFF) as u8;

    rgba
}