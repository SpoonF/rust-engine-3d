use std::ops::{Add, BitXor, Index, IndexMut, Mul, Sub};

use num::{Float, NumCast};

#[derive(Debug, Clone, Copy)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3D<T> {
        Vector3D { x, y, z }
    }
    pub fn x(self) -> T {
        self.x
    }
    pub fn y(self) -> T {
        self.y
    }
    pub fn z(self) -> T {
        self.z
    }
    fn _get_by_index(&self, index: usize) -> &T {

        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Undefinded index {} in Vector3D", index)
        }
    }

    fn _get_by_index_mut(&mut self, index: usize) -> &mut T {

        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Undefinded index {} in Vector3D", index)
        }
    }
}
impl<T> Index<usize> for Vector3D<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self._get_by_index(index)
    }
}
impl<T> IndexMut<usize> for Vector3D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self._get_by_index_mut(index)
    }
}
impl<T: NumCast> Vector3D<T> {
    // Метод для преобразования типа
    pub fn cast<U: NumCast>(self) -> Vector3D<U>
    {
        Vector3D {
            x: NumCast::from(self.x).unwrap(),
            y: NumCast::from(self.y).unwrap(),
            z: NumCast::from(self.z).unwrap(),
        }
    }
}
impl<f32> Vector3D<f32> 
where f32: NumCast + Float{
    pub fn round<U: NumCast>(self) -> Vector3D<U>
    {
        Vector3D {
            x: NumCast::from(self.x.round()).unwrap(),
            y: NumCast::from(self.y.round()).unwrap(),
            z: NumCast::from(self.z.round()).unwrap(),
        }
    }
}
impl<T> Vector3D<T> 
where T: Copy + Add<Output = T> + Mul<Output = T> + Float + Copy {
    pub fn norm(self) -> T
    {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(self, lenght: T) -> Vector3D<T>
    {
        self * (lenght / self.norm())
    }
}
impl<T: Add<Output = T>> Add for Vector3D<T> {
    type Output = Vector3D<T>;

    fn add(self, other: Vector3D<T>) -> Vector3D<T> {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector3D<T>  {
    type Output = Vector3D<T>;

    fn sub(self, other: Vector3D<T>) -> Vector3D<T> {
        Vector3D { 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for Vector3D<T>  {
    type Output = Vector3D<T>;

    fn mul(self, other: T) -> Vector3D<T> {
        Vector3D { 
            x: self.x * other.clone(),
            y: self.y * other.clone(),
            z: self.z * other.clone(),
        }
    }
}
impl<T> Mul for Vector3D<T>
where T: Mul<Output = T> + Add<Output = T>{
    type Output = T;

    fn mul(self, other: Vector3D<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T> BitXor for Vector3D<T> 
where T: Mul<Output = T> + Sub<Output = T> + Copy
{
    type Output = Vector3D<T>;

    fn bitxor(self, other: Vector3D<T>) -> Self::Output {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Vector3D<f32> {
    pub fn from(m: Matrix) -> Vector3D<f32> {
        Vector3D::new(
            m[0][0]/m[3][0], 
            m[1][0]/m[3][0], 
            m[2][0]/m[3][0]
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
    
}

impl<T> Vector2D<T> {
    
    pub fn new(x: T, y: T) -> Vector2D<T> {
        Vector2D { x, y}
    }


    pub fn x(self) -> T {
        self.x
    }
    pub fn y(self) -> T {
        self.y
    }
}

impl<T: NumCast> Vector2D<T> {
    // Метод для преобразования типа
    pub fn cast<U: NumCast>(self) -> Vector2D<U>
    {
        Vector2D {
            x: NumCast::from(self.x).unwrap(),
            y: NumCast::from(self.y).unwrap(),
        }
    }
}
impl<f32> Vector2D<f32> 
where f32: NumCast + Float{
    pub fn round<U: NumCast>(self) -> Vector2D<U>
    {
        Vector2D {
            x: NumCast::from(self.x.round()).unwrap(),
            y: NumCast::from(self.y.round()).unwrap(),
        }
    }
}
impl<T: Add<Output = T>> Add for Vector2D<T> {
    type Output = Vector2D<T>;

    fn add(self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector2D<T>  {
    type Output = Vector2D<T>;

    fn sub(self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D { 
            x: self.x - other.x,
            y: self.y - other.y 
        }
    }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for Vector2D<T>  {
    type Output = Vector2D<T>;

    fn mul(self, other: T) -> Vector2D<T> {
        Vector2D { 
            x: self.x * other.clone(),
            y: self.y * other.clone() 
        }
    }
}
impl<T: BitXor + Mul<Output = T> + Sub<Output = T> + Copy> BitXor for Vector2D<T> {
    type Output = Vector2D<T>;

    fn bitxor(self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.y * other.x - self.x * other.y,
            y: self.y * other.x - self.x * other.y,
        }
    }
}

#[derive(Clone)]
pub struct Matrix {
    matrix: Vec<Vec<f32>>,
    rows: i32,
    cols: i32,
}

impl Matrix {
    pub fn new(rows: i32, cols: i32) -> Matrix{
        Matrix { 
            matrix: vec![vec![0.0; cols as usize]; rows as usize], 
            rows, 
            cols
        }
    }
    pub fn identity(dimensions: i32) -> Matrix {
        let mut m = Matrix::new(dimensions, dimensions);
        for i in 0..dimensions as usize {
            for j in 0..dimensions as usize {
                m[i][j] = if i == j { 1.0 } else { 0.0 }
            }
        }

        m
    }
    pub fn nrows(&self) -> i32 {
        self.rows
    }
    pub fn ncols(&self) -> i32 {
        self.cols
    }
    pub fn trunspose(&self) -> Matrix {
        let mut m = Matrix::new(self.cols, self.rows);
        for i in  0..self.rows as usize {
            for j in  0..self.cols as usize {
                m[j][i] = self[i][i];
            }
        }
        m
    }
    pub fn inverse(&mut self) -> Matrix {
        assert_eq!(self.rows, self.cols);

        let mut m = Matrix::new(self.rows, self.cols * 2);

        for i in  0..self.rows as usize {
            for j in  0..self.cols as usize {
                m[i][j] = self[i][j];
            }
        }
        for i in  0..self.rows as usize {
            m[i][i + self.cols as usize] = 1.0;
        }

        // first pass
        for i in 0..(self.rows-1) as usize{
            // normalize the first row
            for j in (m.cols-1) as usize..=0 {
                m[i][j] /= m[i][i];
            }
            for k in i + 1..self.rows as usize {
                let coeff = m[k][i];
                for j in 0..m.cols as usize {
                    m[k][j] -= m[i][j]*coeff;
                }
            }
        }
        // normalize the last row
        for j in (m.cols-1) as usize..=(self.rows-1) as usize  {
            m[(self.rows-1) as usize][j] /= m[(self.rows-1) as usize][(self.rows-1) as usize];
        }
        // second pass
        for i in (self.rows-1) as usize..0 {
            for k in i-1..=0 {
                let coeff = m[k][i];
                for j in 0..m.cols as usize {
                    m[k][j] -= m[i][j]*coeff;
                }
            }
        }

        let mut trancate = Matrix::new(self.rows, self.cols);
        
        for i in  0..self.rows as usize {
            for j in  0..self.cols as usize {
                trancate[i][j] = m[i][j + self.cols as usize];
            }
        }
        trancate
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Self::Output {
        assert_eq!(self.cols, other.rows);
        let mut m = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows as usize {
            for j in 0..other.cols as usize {
                m[i][j] = 0.0;

                for k in 0..self.cols as usize {
                    m[i][j] += self[i][k] * other[k][j];
                }
            }
        }
        m
    }
}

impl Matrix {
    pub fn from(v: Vector3D<f32>) -> Matrix {
        let mut m = Matrix::new(4, 1);
        m[0][0] = v.x;
        m[1][0] = v.x;
        m[2][0] = v.x;
        m
    }
}