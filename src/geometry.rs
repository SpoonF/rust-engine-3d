use std::ops::{Add, BitXor, Mul, Sub};

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
