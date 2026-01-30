use std::{ops::{Add, BitXor, Div, Index, IndexMut, Mul, Sub}, usize};

use num::{Float, NumCast};

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize, T> {
    pub(crate) vec: [T; N]
}
impl<const N: usize, T> Vector<N, T> {
    pub fn new(vec: [T; N]) -> Self {
        Vector { vec }
    }
    pub fn empty() -> Self 
    where T: num::Zero + Copy {
        Vector { vec: [T::zero(); N] }
    }
    pub fn embed<const C: usize>(&self, fill: T) -> Vector<C, T> 
    where T: num::One + Copy {

        assert!(C > N);
        let mut vec = [fill; C];

        for i in 0..N {
            vec[i] = self[i]
        }
        
        Vector { vec }
    }
    pub fn proj<const C: usize>(&self) -> Vector<C, T> 
    where T: num::Zero + Copy {
        assert!(C < N);
        let mut vec: Vector<C, T> = Vector::empty();
        for i in 0..C {
            vec[i] = self[i]
        }

        vec 
    }
    pub fn len(&self) -> usize{
        N
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
            vec[i] = NumCast::from(*val).unwrap();
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
where T: Copy + Clone + Add<Output = T> + Mul<Output = T> + Float {
    pub fn norm(self) -> T
    {
        let mut result = T::zero();
        for val in self.vec.iter() {
            result = result + (*val * *val);
        }

        result.sqrt()
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
impl<const N: usize, T> Div<T> for Vector<N, T>  
where T: Div<Output = T> + Clone + Copy + num::Zero {
    type Output = Vector<N, T>;

    fn div(self, other: T) -> Self::Output {

        let mut vec: [T; N] = [T::zero(); N];

        for i in 0..N {
            vec[i] = self[i] / other;
        }

        Vector { vec }
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
impl<const N: usize, const MN: usize> From<Matrix<MN, 1>> for Vector<N, f32> {
    fn from(m: Matrix<MN, 1>) -> Vector<N, f32> {
        assert_eq!(N, MN - 1);
        let mut vec: [f32; N] = [0.0; N];

        for i in 0..N {
            vec[i] = m[i][0]/m[N][0];
        }
        
        Vector::new(vec)
    }
}
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Matrix<const ROWS: usize, const COLS: usize> {
    matrix: [Vector<COLS, f32>; ROWS],
    rows: usize,
    cols: usize,
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    pub fn new() -> Self {
        Matrix { 
            matrix: [Vector::empty(); ROWS], 
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
    pub fn col(&self, idx: usize) -> Vector<ROWS, f32> {
        assert!(idx < COLS);
        let mut vec: Vector<ROWS, f32> = Vector::empty();
        for i in 0..ROWS {
            vec[i] = self[i][idx];
        }
        vec
    }
    pub fn trunspose(&self) -> Matrix<COLS, ROWS> {
        let mut m: Matrix<COLS, ROWS> = Matrix::new();
        for i in  0..COLS {
            m[i] = self.col(i);
        }
        m
    }
    
    pub fn set_col(&mut self, idx: usize, v: Vector<ROWS, f32>) {
        assert!(idx < COLS);
        for i in ROWS..0 {
            self[i][idx] = v[i];
        }
    }
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
        for i in 0..N {
            for j in 0..N  {
                m[i][j] = if i == j { 1.0 } else { 0.0 }
            }
        }

        m
    }
}


impl<const ROWS: usize, const COLS: usize>  
Index<usize> for Matrix<ROWS, COLS> {
    type Output = Vector<COLS, f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

impl<const ROWS: usize, const COLS: usize>  
IndexMut<usize> for Matrix<ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

impl<const ROWS: usize, const COLS: usize, const N: usize> 
Mul<Matrix<COLS, N>> for Matrix<ROWS, COLS> 
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
impl<const ROWS: usize, const COLS: usize> 
    Mul<Vector<COLS, f32>> for Matrix<ROWS, COLS> 
{
    type Output = Vector<ROWS, f32>;

    fn mul(self, vec: Vector<COLS, f32>) -> Self::Output {
        let mut result = Vector::<ROWS, f32>::empty(); // или new()
        
        for i in 0..ROWS {  // Для каждой строки матрицы
            let mut sum = 0.0;
            
            for j in 0..COLS {  // Для каждого элемента в строке
                sum += self[i][j] * vec.vec[j];
            }
            
            result.vec[i] = sum;
        }
        
        result
    }
}

impl<const N: usize, const VN: usize> From<Vector<VN, f32>> for Matrix<N, 1> {
    fn from(v: Vector<VN, f32>) -> Matrix<N, 1> 
    {
        assert_eq!(N, VN + 1);
        let mut m: Matrix<N, 1> = Matrix::new();
        for i in 0..VN {
            m[i][0] = v[i];
        }
        m[VN][0] = 1.0;
        m
    }
}