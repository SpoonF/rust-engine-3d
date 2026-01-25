use std::{fs::{self, File}, io::{BufRead, BufReader, Read}, path::Path};

use crate::geometry::Vector3D;
pub struct Model {
    pub verticates: Vec<Vector3D<f32>>,
    pub faces: Vec<Vec<i32>>
}

impl Model {
    pub fn read(path: &Path) -> Model {
        let file= File::open(path).unwrap();

        let reader = BufReader::new(file);

        let mut verticates = vec![];
        let mut faces = vec![];
        // println!("{:?}", buffer);
        for line in reader.lines() {
            let line = line.unwrap();
            if(line.starts_with("v ")) {
                let parts: Vec<&str> = line.split_whitespace().collect();

                verticates.push(
                    Vector3D::new(
                        parts[1].parse::<f32>().unwrap(), 
                        parts[2].parse::<f32>().unwrap(), 
                        parts[3].parse::<f32>().unwrap() 
                    )
                );
            } else if (line.starts_with("f ")) {
                let mut parts: Vec<&str> = line.split_whitespace().collect();
                parts.remove(0);
                let mut x = vec![];
                
                for part in parts {
                    let t: Vec<&str> = part.split("/").collect();
                    let xd = t[0].parse::<i32>().unwrap() - 1;
                    x.push(xd);
                }
                faces.push(x);
            }
        }

        Model {
            verticates,
            faces
        }
    }
}
impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model").field("verticates", &self.verticates).field("faces", &self.faces).finish()
    }
}