use crate::{DEPTH, HEIGHT, WIDTH, geometry::{Matrix, Vector}, model::{self, Model}};

pub(crate) mod utils {
    use crate::geometry::{Matrix, Vector};

    pub fn viewport(x: i32, y: i32, w: i32, h: i32) -> Matrix<4, 4> {
        let mut m: Matrix<4, 4> = Matrix::identity();
        m[0][3] = (x + w) as f32 / 2.0;
        m[1][3] = (y + h) as f32 / 2.0;
        m[2][3] = 255.0 / 2.0;

        m[0][0] = w as f32 / 2.0;
        m[1][1] = h as f32 / 2.0;
        m[2][2] = 255.0 / 2.0;

        m
    }

    pub fn look_at(_eye: Vector<3, f32>, _center: Vector<3, f32>, _up: Vector<3, f32>) -> Matrix<4, 4> {
        let z =  (_eye - _center).normalize(1.0);
        let x =  cross(_up, z).normalize(1.0);
        let y =  cross(z, x).normalize(1.0);

        let mut minv: Matrix<4, 4> = Matrix::identity();
        let mut tr: Matrix<4, 4> = Matrix::identity();

        for i in 0..3 {
            minv[0][i] = x[i];
            minv[1][i] = y[i];
            minv[2][i] = z[i];
            tr[i][3] = -_center[i];
        }

        minv * tr
    }

    pub fn cross(v1: Vector<3, f32>, v2: Vector<3, f32>) -> Vector<3, f32> {
        Vector::new([
            v1[1] * v2[2] - v1[2] * v2[1],
            -(v1[0] * v2[2] - v1[2] * v2[0]),
            v1[0] * v2[1] - v1[1] * v2[0]
        ])
    }
    pub fn projection(coef: f32) -> Matrix<4, 4>{
        let mut result: Matrix<4, 4> = Matrix::identity();
        result[3][2] = coef;
        result
    }
    pub fn barycentric(a: Vector<2, f32>, b: Vector<2, f32>, c: Vector<2, f32>, p: Vector<2, f32>) -> Vector<3, f32> {
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
    pub fn get_rgb(color: u32) -> [u8; 3] {
        let mut rgb: [u8; 3] = [0xFF; 3];
        rgb[0] = ((color >> 16) & 0xFF) as u8;
        rgb[1] = ((color >> 8) & 0xFF) as u8;
        rgb[2] = (color & 0xFF) as u8;

        rgb
    }
    pub fn get_rgba(color: u32) -> [u8; 4] {
        let mut rgba: [u8; 4] = [0xFF; 4];
        rgba[0] = ((color >> 24) & 0xFF) as u8;
        rgba[1] = ((color >> 16) & 0xFF) as u8;
        rgba[2] = ((color >> 8) & 0xFF) as u8;
        rgba[3] = (color & 0xFF) as u8;

        rgba
    }
    pub fn get_color_rgb(rgb: [u8; 3]) -> u32 {
        let color = (rgb[0] as u32) << 16 | (rgb[1] as u32) << 8 | rgb[2] as u32;
        
        color
    }
    pub fn get_color_rgba(rgba: [u8; 4]) -> u32 {
        let color = (rgba[0] as u32) << 24 | (rgba[1] as u32) << 16 | (rgba[2] as u32) << 8 | rgba[3] as u32;
        
        color
    }
}

#[derive(Debug)]
pub struct Shader<'a> {
    pub varing_uv: Matrix<2, 3>,
    pub varing_tri: Matrix<3, 3>,
    uniform_m: Matrix<4, 4>,
    uniform_mit: Matrix<4, 4>,
    uniform_ms: Matrix<4, 4>,
    shadowbuffer: Vec<f32>,
    model: &'a Model,
    projection: &'a Matrix<4,4>,
    model_view: &'a Matrix<4,4>,
    light_dir: &'a Vector<3, f32>,
}

impl<'a>  Shader<'_> {
    pub fn new(model: &'a Model, projection: &'a Matrix<4,4>, model_view: &'a Matrix<4,4>, light_dir: &'a Vector<3, f32>,
                m: Matrix<4, 4>, mit: Matrix<4, 4>, ms: Matrix<4, 4>) -> Shader<'a> {
        Shader {
            varing_uv: Matrix::new(),
            varing_tri: Matrix::new(),
            uniform_m: m,
            uniform_mit: mit,
            uniform_ms: ms,
            shadowbuffer: vec![f32::MIN; WIDTH*HEIGHT],
            model,
            projection,
            model_view,
            light_dir
        }
    }
    
    pub fn vertex(&mut self, iface: usize, nthvert: usize) -> Vector<4,f32> {
        self.varing_uv.set_col(nthvert, self.model.uv(iface, nthvert).cast());
        
        let gl_vertex: Vector<4, f32> = self.projection.clone() * self.model_view.clone() * self.model.vert(iface, nthvert).embed::<4>(1.0);
        self.varing_tri.set_col(nthvert, (gl_vertex/gl_vertex[3]).proj::<3>());
        gl_vertex
    }  

    pub fn fragment(&self, bar: Vector<3, f32>, color: &mut [u8; 4] ) -> bool {
        let mut sb_p = self.uniform_ms.clone() * (self.varing_tri.clone() * bar).embed(1.0);
        sb_p = sb_p/sb_p[3];

        let idx = sb_p[0] as usize + sb_p[1] as usize * WIDTH;
        let shadow = 0.3 + 0.7 * ((self.shadowbuffer[idx] < sb_p[2]) as i32) as f32;

        let uv = self.varing_uv.clone() * bar;
        let n = (self.uniform_mit.clone() * self.model.normal(uv).embed::<4>(1.0)).proj::<3>().normalize(1.);
        let l = (self.uniform_m.clone() * self.light_dir.embed::<4>(1.0)).proj::<3>().normalize(1.);
        let r = (n * (n * l * 2.0) - l).normalize(1.0);

        let spec = f32::powf(r.z().max(0.0), self.model.specular(uv));
        let diff = 0.0_f32.max(n * l);

        let c = self.model.diffuse(uv);

        for i in 0..3 {
            color[i] = (20.0 + c[i] as f32 * shadow * (1.2 * diff + 0.6 * spec)).min(255.0) as u8;
        }

        false
    }
}