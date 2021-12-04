use std::ops;

use crate::Point;

#[derive(Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize> {
    pub inner: [[f32; C]; R]
}

pub type Vector2D = Matrix<2, 1>;

impl Vector2D {
    pub fn new_from_x_and_y(x: f32, y: f32) -> Self {
        Vector2D {
            inner: [[x], [y]],
        }
    }

    pub fn normalize(&self) -> Self {
        let square_root = f32::sqrt(self.x()*self.x()+self.y()*self.y());
        Vector2D::new_from_x_and_y(self.x()/square_root, self.y()/square_root)
    }

    pub fn x(&self) -> f32 {
        self.inner[0][0]
    }

    pub fn y(&self) -> f32 {
        self.inner[0][1]
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(inner: [[f32; C]; R]) -> Self {
        Matrix {inner}
    }

    pub fn dot_product<const K: usize>(&self, row: usize, other: Matrix<C, K>, col: usize) -> f32 {
        let mut sum: f32 = 0.;
        for i in 0..C {
            sum += self.inner[row][i] * other.inner[i][col];
        }
        sum
    }

    pub fn multiply_matrix<const K: usize>(&self, other: Matrix<C, K>) -> Matrix<R, K> {
        let mut inner: [[f32; K]; R] = [[0.; K]; R];
        for r in 0..R {
            for c in 0..K {
                inner[r][c] = self.dot_product(r, other, c);
            }
        }
        Matrix {inner}
    }

    pub fn sum(&self) -> f32 {
        let mut sum = 0.;
        for r in 0..R {
            for c in 0..C {
                sum += self.inner[r][c];
            }
        }
        sum
    }
}

impl<const R: usize, const C: usize> ops::Add<Matrix<R, C>> for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn add(self, rhs: Matrix<R, C>) -> Matrix<R, C> {
        let mut inner: [[f32; C]; R] = [[0.; C]; R];
        for r in 0..R {
            for c in 0..C {
                inner[r][c] = self.inner[r][c] + rhs.inner[r][c];
            }
        }
        Matrix::new(inner)
    }
}

impl<const R: usize, const C: usize> ops::Sub<Matrix<R, C>> for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn sub(self, rhs: Matrix<R, C>) -> Matrix<R, C> {
        self - rhs
    }
}

impl<const R: usize, const C: usize> ops::Mul<f32> for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn mul(self, rhs: f32) -> Matrix<R, C> {
        let mut inner: [[f32; C]; R] = [[0.; C]; R];
        for r in 0..R {
            for c in 0..C {
                inner[r][c] = self.inner[r][c] * rhs;
            }
        }
        Matrix::new(inner)
    }
}

impl<const R: usize, const C: usize> ops::Mul<Matrix<R, C>> for f32 {
    type Output = Matrix<R, C>;
    fn mul(self, rhs: Matrix<R, C>) -> Matrix<R, C> {
        rhs * self
    }
}

impl<const R: usize, const C: usize> ops::Mul<Matrix<R, C>> for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn mul(self, rhs: Matrix<R, C>) -> Matrix<R, C> {
        let mut inner: [[f32; C]; R] = [[0.; C]; R];
        for r in 0..R {
            for c in 0..C {
                inner[r][c] = self.inner[r][c] * rhs.inner[r][c];
            }
        }
        Matrix::new(inner)
    }
}
