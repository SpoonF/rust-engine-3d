use crate::geometry::{Matrix, Vector};

trait Shader {
    fn varing_uv(&self) -> Matrix<2, 3>;
    fn varing_tri(&self) -> Matrix<4, 3>;
    // fn vertex(&mut self, iface: i32, nthvert: i32) -> Vector<4,f32> {
    //     Vector::new(vec)
    // }   
}