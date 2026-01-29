use std::ops::{Add, BitXor, Index, IndexMut, Mul, Sub};

use num::{Float, NumCast, zero};

#[derive(Clone, Copy)]
pub struct Vector<const N: usize, T> {
    vec: [T; N]
}
impl<const N: usize, T> Vector<N, T> {
    pub fn new(vec: [T; N]) -> Self {
        Vector { vec }
    }
}
impl<const N: usize, T> Index<usize> for Vector<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.vec[index]
    }
}
impl<const N: usize, T> IndexMut<usize> for Vector<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}
impl<const N: usize, T: NumCast + Copy> Vector<N, T> {
    // Метод для преобразования типа
    pub fn cast<U: NumCast + num::Zero + Copy>(self) -> Vector<N, U>
    {
        let mut vec = [U::zero(); N];

        for (i, val) in self.vec.iter().enumerate() {
            vec[i] = NumCast::from(*val).unwrap()
        }
    
        Vector { vec }
    }
}
impl<const N: usize, f32> Vector<N, f32> 
where f32: NumCast + Float{
    pub fn round<U: NumCast + num::Zero + Copy>(self) -> Vector<N, U> 
    {
        let mut vec = [U::zero(); N];

        for (i, val) in self.vec.iter().enumerate() {
            vec[i] = NumCast::from(val.round()).unwrap()
        }
    
        Vector { vec }
    }
}
impl<const N: usize, T> Vector<N, T> 
where T: Copy + Clone + Add<Output = T> + Mul<Output = T> + Float  {
    pub fn norm(self) -> T
    {
        let result = T::zero();
        for (i, val) in self.vec.iter().enumerate() {
            result.add(*val * *val);
        }

        result
    }
    pub fn normalize(self, lenght: T) -> Vector<N, T>  
    {
        self * (lenght / self.norm())
    }
}
impl<const N: usize, T> Add for Vector<N, T> 
where T: Add<Output = T> + Copy + num::Zero{
    type Output = Vector<N, T>;

    fn add(self, other: Vector<N, T>) -> Self::Output {

        let mut vec: [T; N] = [T::zero(); N];

        for i in 0..N {
            vec[i] = self[i] + other[i];
        }

        Vector { vec }
    }
}

impl<const N: usize, T> Sub for Vector<N, T>  
where T: Sub<Output = T> + Copy + num::Zero {
    type Output = Vector<N, T>;

    fn sub(self, other: Vector<N, T>) -> Self::Output {
        let mut vec: [T; N] = [T::zero(); N];

        for i in 0..N {
            vec[i] = self[i] - other[i];
        }

        Vector { vec }
    }
}

impl<const N: usize, T> Mul<T> for Vector<N, T>  
where T: Mul<Output = T> + Clone + Copy + num::Zero {
    type Output = Vector<N, T>;

    fn mul(self, other: T) -> Self::Output {

        let mut vec: [T; N] = [T::zero(); N];

        for i in 0..N {
            vec[i] = self[i] * other;
        }

        Vector { vec }
    }
}
impl<const N: usize, T> Mul for Vector<N, T>
where T: Mul<Output = T> + Add<Output = T> + num::Zero + Copy{
    type Output = T;

    fn mul(self, other: Vector<N, T>) -> T {
        let mut result = T::zero();

        for i in 0..N {
            result = result + (self[i] * other[i]);
        }
        result

    }
}

impl<const N: usize, T> BitXor for Vector<N, T> 
where T: Mul<Output = T> + Sub<Output = T> + Copy + num::Zero
{
    type Output = Vector<N, T>;

    fn bitxor(self, other: Vector<N, T>) -> Self::Output {
        
        let mut vec: [T; N] = [T::zero(); N];

        for i in 0..N {
            let a = if i == N - 1 { 0 } else {i + 1};
            let b = if a == 0 { 1 } else {a + 1};
            vec[i] = other[a] * self[b] - self[b] * other[a];
        }
        
        Vector::new(vec)
    }
}
impl<const N: usize> Vector<N, f32> {
    pub fn from(m: Matrix<N, 1>) -> Vector<N, f32> {
        let mut vec: [f32; N] = [0.0; N];

        for i in 0..N - 1 {
            vec[i] = m[i][0]/m[N - 1][0];
        }
        
        Vector::new(vec)
    }
}
//////////////////////////////////////////////////////////////
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
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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
    pub fn from(m: Matrix<4, 1>) -> Vector3D<f32> {
        Vector3D::new(
            m[0][0]/m[3][0], 
            m[1][0]/m[3][0], 
            m[2][0]/m[3][0]
        )
    }
}
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy)]
pub struct Vector4D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub t: T
}

impl<T> Vector4D<T> {
    pub fn new(x: T, y: T, z: T, t: T) -> Vector4D<T> {
        Vector4D { x, y, z , t}
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

    pub fn t(self) -> T {
        self.t
    }
    fn _get_by_index(&self, index: usize) -> &T {

        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.t,
            _ => panic!("Undefinded index {} in Vector4D", index)
        }
    }

    fn _get_by_index_mut(&mut self, index: usize) -> &mut T {

        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.t,
            _ => panic!("Undefinded index {} in Vector4D", index)
        }
    }
}
impl<T> Index<usize> for Vector4D<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self._get_by_index(index)
    }
}
impl<T> IndexMut<usize> for Vector4D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self._get_by_index_mut(index)
    }
}
impl<T: NumCast> Vector4D<T> {
    // Метод для преобразования типа
    pub fn cast<U: NumCast>(self) -> Vector4D<U>
    {
        Vector4D {
            x: NumCast::from(self.x).unwrap(),
            y: NumCast::from(self.y).unwrap(),
            z: NumCast::from(self.z).unwrap(),
            t: NumCast::from(self.t).unwrap(),
        }
    }
}
impl<f32> Vector4D<f32> 
where f32: NumCast + Float{
    pub fn round<U: NumCast>(self) -> Vector4D<U>
    {
        Vector4D {
            x: NumCast::from(self.x.round()).unwrap(),
            y: NumCast::from(self.y.round()).unwrap(),
            z: NumCast::from(self.z.round()).unwrap(),
            t: NumCast::from(self.t.round()).unwrap(),
        }
    }
}
impl<T> Vector4D<T> 
where T: Copy + Add<Output = T> + Mul<Output = T> + Float + Copy {
    pub fn norm(self) -> T
    {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.t * self.t).sqrt()
    }
    pub fn normalize(self, lenght: T) -> Vector4D<T>
    {
        self * (lenght / self.norm())
    }
}
impl<T: Add<Output = T>> Add for Vector4D<T> {
    type Output = Vector4D<T>;

    fn add(self, other: Vector4D<T>) -> Vector4D<T> {
        Vector4D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            t: self.t + other.t,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector4D<T>  {
    type Output = Vector4D<T>;

    fn sub(self, other: Vector4D<T>) -> Vector4D<T> {
        Vector4D { 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            t: self.t - other.t,
        }
    }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for Vector4D<T>  {
    type Output = Vector4D<T>;

    fn mul(self, other: T) -> Vector4D<T> {
        Vector4D { 
            x: self.x * other.clone(),
            y: self.y * other.clone(),
            z: self.z * other.clone(),
            t: self.t * other.clone(),
        }
    }
}
impl<T> Mul for Vector4D<T>
where T: Mul<Output = T> + Add<Output = T>{
    type Output = T;

    fn mul(self, other: Vector4D<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.t * other.t
    }
}

impl<T> BitXor for Vector4D<T> 
where T: Mul<Output = T> + Sub<Output = T> + Copy
{
    type Output = Vector4D<T>;

    fn bitxor(self, other: Vector4D<T>) -> Self::Output {
        Vector4D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.t - self.t * other.z,
            z: self.t * other.x - self.x * other.t,
            t: self.x * other.y - self.y * other.x,
        }
    }
}

impl Vector4D<f32> {
    pub fn from(m: Matrix<5, 1>) -> Vector4D<f32> {
        Vector4D::new(
            m[0][0]/m[4][0], 
            m[1][0]/m[4][0], 
            m[2][0]/m[4][0],
            m[3][0]/m[4][0],
        )
    }
}
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Matrix<const ROWS: usize, const COLS: usize> {
    matrix: [[f32; COLS]; ROWS],
    rows: usize,
    cols: usize,
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    pub fn new() -> Self {
        Matrix { 
            matrix: [[0.0; COLS]; ROWS], 
            rows: ROWS, 
            cols: COLS
        }
    }
    pub fn nrows(&self) -> usize {
        self.matrix.len()
    }
    pub fn ncols(&self) -> usize {
        self.matrix[0].len()
    }
    pub fn trunspose(&self) -> Matrix<COLS, ROWS> {
        let mut m: Matrix<COLS, ROWS> = Matrix::new();
        for i in  0..ROWS {
            for j in  0..COLS {
                m[j][i] = self[i][i];
            }
        }
        m
    }
    
    // pub fn set_col(&self, idx: usize, v: V) {
    //     todo!()
    // }
}
impl<const N: usize> Matrix<N, N> {
    /// Вычисляет обратную матрицу методом Гаусса-Жордана
    /// Возвращает Option<Matrix<N, N>> - None если матрица вырожденная
    pub fn inverse(&self) -> Option<Matrix<N, N>> {
        
        // Создаём расширенную матрицу [A | I]
        let mut augmented: Vec<Vec<f32>> = vec![vec![0.0; 2 * N]; N];
        
        // Копируем исходную матрицу и добавляем единичную
        for i in 0..N {
            for j in 0..N {
                augmented[i][j] = self.matrix[i][j];
                augmented[i][j + N] = if i == j { 1.0 } else { 0.0 };
            }
        }
        
        // Приведение к ступенчатому виду (прямой ход)
        for col in 0..N {
            // Поиск опорного элемента (pivot)
            let mut pivot_row = col;
            for row in col..N {
                if augmented[row][col].abs() > augmented[pivot_row][col].abs() {
                    pivot_row = row;
                }
            }
            
            // Если опорный элемент близок к нулю - матрица вырожденная
            if augmented[pivot_row][col].abs() < 1e-10 {
                return None;
            }
            
            // Обмен строк, если нужно
            if pivot_row != col {
                augmented.swap(col, pivot_row);
            }
            
            // Нормализация текущей строки
            let pivot = augmented[col][col];
            for j in col..(2 * N) {
                augmented[col][j] /= pivot;
            }
            
            // Обнуление элементов в текущем столбце
            for i in 0..N {
                if i != col {
                    let factor = augmented[i][col];
                    for j in col..(2 * N) {
                        augmented[i][j] -= factor * augmented[col][j];
                    }
                }
            }
        }
        
        // Извлекаем обратную матрицу из правой части
        let mut inverse = Matrix::<N, N>::new();
        for i in 0..N {
            for j in 0..N {
                inverse.matrix[i][j] = augmented[i][j + N];
            }
        }
        
        Some(inverse)
    }

    pub fn identity() -> Matrix<N, N> {
        let mut m: Matrix<N, N> = Matrix::new();
        for i in 0..N as usize {
            for j in 0..N as usize {
                m[i][j] = if i == j { 1.0 } else { 0.0 }
            }
        }

        m
    }
}


impl<const ROWS: usize, const COLS: usize>  Index<usize> for Matrix<ROWS, COLS> {
    type Output = [f32; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

impl<const ROWS: usize, const COLS: usize>  IndexMut<usize> for Matrix<ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

impl<const ROWS: usize, const COLS: usize, const N: usize> Mul<Matrix<COLS, N>> for Matrix<ROWS, COLS> 
{
    type Output = Matrix<COLS, N>;

    fn mul(self, other: Matrix<COLS, N>) -> Self::Output {
        assert_eq!(self.cols, other.rows);
        let mut m = Matrix::new();
        for i in 0..ROWS {
            for j in 0..N {
                m[i][j] = 0.0;

                for k in 0..COLS {
                    m[i][j] += self[i][k] * other[k][j];
                }
            }
        }
        m
    }
}

impl Matrix<4, 1> {
    pub fn from(v: Vector3D<f32>) -> Matrix<4, 1> {
        let mut m: Matrix<4, 1> = Matrix::new();
        m[0][0] = v.x;
        m[1][0] = v.y;
        m[2][0] = v.x;
        m[3][0] = 1.0;
        m
    }
}
