use std::ops::{self, AddAssign, DivAssign, MulAssign, SubAssign};
use std::ops::{Index, IndexMut};

use cgmath::num_traits::ToPrimitive;

use super::Vector;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Matrix {
    data: Vec<f32>,
    rows: usize,
    cols: usize,
}

#[macro_export]
macro_rules! matrix {
    ( $( $arr:expr ),* ) => {
        Matrix::from_vec(vec![ $( $arr.to_vec() ),* ])
    };
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f32>) -> Matrix {
        assert_eq!(data.len(), rows * cols);
        Matrix { data, rows, cols }
    }

    pub fn zeroes(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.; rows * cols],
            rows,
            cols,
        }
    }

    pub fn ones(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![1.; rows * cols],
            rows,
            cols,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn row(&self, i: usize) -> Vec<f32> {
        self.data[i * self.cols..(i + 1) * self.cols].to_vec()
    }

    pub fn col(&self, j: usize) -> Vec<f32> {
        (0..self.rows).map(|i| self[i][j]).collect()
    }

    fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn identity(rows: usize, cols: usize) -> Self {
        let mut m = Matrix::zeroes(rows, cols);
        for i in 0..rows {
            m[i][i] = 1.0;
        }
        m
    }

    pub fn from_vec(data: Vec<Vec<f32>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let mut m = Matrix::zeroes(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                m[i][j] = data[i][j];
            }
        }
        m
    }

    pub fn from_vector(data: &Vector) -> Self {
        Matrix {
            data: data.as_slice().to_vec(),
            rows: data.len(),
            cols: 1,
        }
    }

    pub fn from_slice(slice: &[&[f32]]) -> Self {
        Self::from_vec(slice.into_iter().map(|r| r.to_vec()).collect())
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.data[..]
    }

    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        &mut self.data[..]
    }

    pub fn reshape(&self, new_rows: usize, new_cols: usize) -> Self {
        assert_eq!(self.rows * self.cols, new_rows * new_cols);
        Matrix::new(new_rows, new_cols, self.data.clone())
    }

    pub fn transpose(&self) -> Matrix {
        let mut data = vec![0.0; self.rows * self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[j * self.rows + i] = self[i][j];
            }
        }
        Matrix::new(self.cols, self.rows, data)
    }

    pub fn remove_row(&mut self, i: usize) {
        assert!(i < self.rows);
        self.data.drain(i * self.cols..(i + 1) * self.cols);
        self.rows -= 1;
    }

    pub fn remove_col(&mut self, j: usize) {
        assert!(j < self.cols);
        for i in (0..self.rows).rev() {
            self.data.remove(i * self.cols + j);
        }
        self.cols -= 1;
    }

    pub fn minor(&self, i: usize, j: usize) -> f32 {
        let mut m = self.clone();
        m.remove_row(i);
        m.remove_col(j);
        m.det()
    }

    pub fn cofactor(&self, i: usize, j: usize) -> f32 {
        (-1_i32).pow((i + j) as u32 % 2) as f32 * self.minor(i, j)
    }

    pub fn det(&self) -> f32 {
        assert!(self.is_square());
        if self.rows() == 1 {
            self[0][0]
        } else {
            (0..self.cols())
                .map(|j| self[0][j] * self.cofactor(0, j))
                .sum()
        }
    }

    pub fn as_vector(&self) -> Vector {
        assert_eq!(self.cols(), 1);
        Vector::from_vec(self.data.clone())
    }
}

// Unary Ops

impl_op_ex!(-|matrix: &Matrix| -> Matrix {
    Matrix::from_vec(
        (0..matrix.rows())
            .map(|i| (0..matrix.cols()).map(|j| -matrix[i][j]).collect())
            .collect(),
    )
});

// Scalar Ops

impl_op_ex_commutative!(+ |matrix: &Matrix, scalar: &f32| -> Matrix {
    Matrix::from_vec(
        (0..matrix.rows())
            .map(|i| (0..matrix.cols()).map(|j| matrix[i][j] + *scalar).collect())
            .collect(),
    )
});

impl_op_ex_commutative!(+ |matrix: &Matrix, scalar: &i32| -> Matrix {
    Matrix::from_vec(
        (0..matrix.rows())
            .map(|i| (0..matrix.cols()).map(|j| matrix[i][j] + *scalar as f32).collect())
            .collect(),
    )
});

impl_op_ex!(-|matrix: &Matrix, scalar: &f32| -> Matrix { matrix + (-scalar) });

impl_op_ex!(-|matrix: &Matrix, scalar: &i32| -> Matrix { matrix + (-scalar as f32) });

impl_op_ex!(-|scalar: &f32, matrix: &Matrix| -> Matrix { scalar + (-matrix) });

impl_op_ex!(-|scalar: &i32, matrix: &Matrix| -> Matrix { *scalar as f32 + (-matrix) });

impl_op_ex_commutative!(*|matrix: &Matrix, scalar: &f32| -> Matrix {
    Matrix::from_vec(
        (0..matrix.rows())
            .map(|i| (0..matrix.cols()).map(|j| matrix[i][j] * *scalar).collect())
            .collect(),
    )
});

impl_op_ex_commutative!(*|matrix: &Matrix, scalar: &i32| -> Matrix {
    Matrix::from_vec(
        (0..matrix.rows())
            .map(|i| {
                (0..matrix.cols())
                    .map(|j| matrix[i][j] * *scalar as f32)
                    .collect()
            })
            .collect(),
    )
});

impl_op_ex!(/ |matrix: &Matrix, scalar: &f32| -> Matrix {
    Matrix::from_vec(
        (0..matrix.rows())
            .map(|i| (0..matrix.cols()).map(|j| matrix[i][j] / *scalar).collect())
            .collect(),
    )
});

impl_op_ex!(/ |matrix: &Matrix, scalar: &i32| -> Matrix {
    Matrix::from_vec(
        (0..matrix.rows())
            .map(|i| {
                (0..matrix.cols())
                    .map(|j| matrix[i][j] / *scalar as f32)
                    .collect()
            })
            .collect(),
    )
});

// Matrix Ops

impl_op_ex!(+ |lhs: &Matrix, rhs: &Matrix| -> Matrix {
    assert_eq!(lhs.rows(),rhs.rows());
    assert_eq!(lhs.cols(),rhs.cols());
    Matrix::from_vec(
        (0..lhs.rows())
            .map(|i| {
                (0..lhs.cols())
                    .map(|j| lhs[i][j] + rhs[i][j])
                    .collect()
            })
            .collect(),
    )
});

impl_op_ex!(-|lhs: &Matrix, rhs: &Matrix| -> Matrix { lhs + (-rhs) });

impl_op_ex!(*|lhs: &Matrix, rhs: &Matrix| -> Matrix {
    assert_eq!(lhs.cols(), rhs.rows());
    Matrix::from_vec(
        (0..lhs.rows())
            .map(|i| {
                (0..rhs.cols())
                    .map(|j| -> f32 { (0..lhs.cols()).map(|k| lhs[i][k] * rhs[k][j]).sum() })
                    .collect()
            })
            .collect(),
    )
});

// Vector Ops

impl_op_ex!(*|matrix: &Matrix, vector: &Vector| -> Matrix {
    assert_eq!(matrix.cols(), vector.len());
    matrix * Matrix::from_vector(vector)
});

// Scalar Ops Assignment

impl<T: ToPrimitive> AddAssign<T> for Matrix {
    fn add_assign(&mut self, scalar: T) {
        let scalar_f32 = scalar.to_f32().unwrap();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self[i][j] += scalar_f32;
            }
        }
    }
}

impl<T: ToPrimitive> SubAssign<T> for Matrix {
    fn sub_assign(&mut self, scalar: T) {
        let scalar_f32 = scalar.to_f32().unwrap();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self[i][j] -= scalar_f32;
            }
        }
    }
}

impl<T: ToPrimitive> MulAssign<T> for Matrix {
    fn mul_assign(&mut self, scalar: T) {
        let scalar_f32 = scalar.to_f32().unwrap();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self[i][j] *= scalar_f32;
            }
        }
    }
}

impl<T: ToPrimitive> DivAssign<T> for Matrix {
    fn div_assign(&mut self, scalar: T) {
        let scalar_f32 = scalar.to_f32().unwrap();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self[i][j] /= scalar_f32;
            }
        }
    }
}

// Vector Ops Assignment

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.rows(), rhs.rows());
        assert_eq!(self.cols(), rhs.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self[i][j] += rhs[i][j];
            }
        }
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.rows(), rhs.rows());
        assert_eq!(self.cols(), rhs.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self[i][j] -= rhs[i][j];
            }
        }
    }
}

// Indexing

impl Index<usize> for Matrix {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.cols..(index + 1) * self.cols]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.cols..(index + 1) * self.cols]
    }
}
