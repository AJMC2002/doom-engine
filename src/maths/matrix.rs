use std::ops::{self, AddAssign, DivAssign, MulAssign, SubAssign};
use std::ops::{Index, IndexMut};
use std::usize;

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
        $crate::maths::Matrix::from_vec(vec![ $( $arr.to_vec() ),* ])
    };
}

//Usual methods

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f32>) -> Self {
        assert_eq!(data.len(), rows * cols);
        Self { data, rows, cols }
    }

    pub fn identity(size: usize) -> Self {
        let mut m = Self::zeroes(size, size);
        for i in 0..size {
            m[i][i] = 1.0;
        }
        m
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

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn from_vec(data: Vec<Vec<f32>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let mut m = Self::zeroes(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                m[i][j] = data[i][j];
            }
        }
        m
    }

    pub fn from_vector(data: &Vector) -> Self {
        Self {
            data: data.as_slice().to_vec(),
            rows: data.len(),
            cols: 1,
        }
    }

    pub fn from_slice(slice: &[&[f32]]) -> Self {
        Self::from_vec(slice.iter().map(|r| r.to_vec()).collect())
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.data[..]
    }

    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        &mut self.data[..]
    }

    pub fn reshape(&self, new_rows: usize, new_cols: usize) -> Self {
        assert_eq!(self.rows * self.cols, new_rows * new_cols);
        Self::new(new_rows, new_cols, self.data.clone())
    }

    pub fn transpose(&self) -> Self {
        let mut data = vec![0.0; self.rows * self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[j * self.rows + i] = self[i][j];
            }
        }
        Self::new(self.cols, self.rows, data)
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

    pub fn submatrix(&self, i: usize, j: usize) -> Self {
        let mut m = self.clone();
        m.remove_row(i);
        m.remove_col(j);
        m
    }

    pub fn minor(&self, i: usize, j: usize) -> f32 {
        self.submatrix(i, j).det()
    }

    pub fn cofactor(&self, i: usize, j: usize) -> f32 {
        (-1_i32).pow((i + j) as u32 % 2) as f32 * self.minor(i, j)
    }

    pub fn comatrix(&self) -> Self {
        Self {
            data: (0..self.rows() * self.cols())
                .map(|lin_i| self.cofactor(lin_i / self.rows(), lin_i % self.cols()))
                .collect::<Vec<f32>>(),
            rows: self.rows(),
            cols: self.cols(),
        }
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

    pub fn inverse(&self) -> Self {
        self.comatrix().transpose() / self.det()
    }

    ///Takes a column matrix and turns it into a vector
    pub fn as_vector(&self) -> Vector {
        assert_eq!(self.cols(), 1);
        Vector::from(self.data.clone())
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }
}

//Custom matrices

impl Matrix {
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

    pub fn scaling(values: Vector) -> Self {
        assert_eq!(values.len(), 3);
        Self {
            data: vec![
                values[0], 0., 0., 0., //row 1
                0., values[1], 0., 0., //row 2
                0., 0., values[2], 0., //row 3
                0., 0., 0., 1., //row 4
            ],
            rows: 4,
            cols: 4,
        }
    }

    pub fn translation(values: Vector) -> Self {
        assert_eq!(values.len(), 3);
        Self {
            data: vec![
                1., 0., 0., values[0], //row 1
                0., 1., 0., values[1], //row 2
                0., 0., 1., values[2], //row 3
                0., 0., 0., 1., //row 4
            ],
            rows: 4,
            cols: 4,
        }
    }

    pub fn rotation(values: Vector) -> Self {
        assert_eq!(values.len(), 3);
        Self::rotation_z(values[2]) * Self::rotation_y(values[1]) * Self::rotation_x(values[0])
    }

    pub fn rotation_x(angle: f32) -> Self {
        Self {
            data: vec![
                1.,
                0.,
                0.,
                0., //row 1
                0.,
                angle.cos(),
                -angle.sin(),
                0., //row 2
                0.,
                angle.sin(),
                angle.cos(),
                0., //row 3
                0.,
                0.,
                0.,
                1., //row 4
            ],
            rows: 4,
            cols: 4,
        }
    }

    pub fn rotation_y(angle: f32) -> Self {
        Self {
            data: vec![
                angle.cos(),
                0.,
                angle.sin(),
                0., //row 1
                0.,
                1.,
                0.,
                0., //row 2
                -angle.sin(),
                0.,
                angle.cos(),
                0., //row 3
                0.,
                0.,
                0.,
                1., //row 4
            ],
            rows: 4,
            cols: 4,
        }
    }

    pub fn rotation_z(angle: f32) -> Self {
        Self {
            data: vec![
                angle.cos(),
                -angle.sin(),
                0.,
                0., //row 1
                angle.sin(),
                angle.cos(),
                0.,
                0., //row 2
                0.,
                0.,
                1.,
                0., //row 3
                0.,
                0.,
                0.,
                1., //row 4
            ],
            rows: 4,
            cols: 4,
        }
    }

    pub fn model(
        translation_values: Vector,
        rotation_values: Vector,
        scaling_values: Vector,
    ) -> Self {
        Self::translation(translation_values)
            * Self::rotation(rotation_values)
            * Self::scaling(scaling_values)
    }

    pub fn model_default() -> Self {
        Self::identity(4)
    }

    /// Returns the normal matrix from a model matrix.
    pub fn to_normal(&self) -> Self {
        assert!(self.is_square());
        assert_eq!(self.rows(), 4);
        let mut m = self.clone();
        m.remove_row(3);
        m.remove_col(3);
        m.inverse().transpose()
    }

    pub fn projection_orthographic(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            data: vec![
                2. / (right - left),
                0.,
                0.,
                -(right + left) / (right - left), //row 1
                0.,
                2. / (top - bottom),
                0.,
                -(top + bottom) / (top - bottom), //row 2
                0.,
                0.,
                -2. / (far - near),
                -(far + near) / (far - near), //row 3
                0.,
                0.,
                0.,
                1., //row 4
            ],
            rows: 4,
            cols: 4,
        }
    }

    pub fn projection_perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let tan_of_half_fov = (fov / 2.).tan();
        Self {
            data: vec![
                1. / (aspect * tan_of_half_fov),
                0.,
                0.,
                0., //row 1
                0.,
                1. / tan_of_half_fov,
                0.,
                0., //row 2
                0.,
                0.,
                -(far + near) / (far - near),
                -2. * far * near / (far - near), //row 3
                0.,
                0.,
                -1.,
                0., //row 4
            ],
            rows: 4,
            cols: 4,
        }
    }

    /// position: Camera position
    /// target: Target position
    /// up: Up vector in world space
    pub fn look_at(position: &Vector, target: &Vector, up: &Vector) -> Self {
        assert_eq!(position.len(), 3);
        assert_eq!(target.len(), 3);
        assert_eq!(up.len(), 3);
        let direction = (position - target).unit();
        let cam_right = up.unit().cross(&direction).unit();
        let cam_up = direction.cross(&cam_right);
        Self {
            data: vec![
                cam_right[0],
                cam_right[1],
                cam_right[2],
                0., //row 1
                cam_up[0],
                cam_up[1],
                cam_up[2],
                0., //row 2
                direction[0],
                direction[1],
                direction[2],
                0., //row 3
                0.,
                0.,
                0.,
                1., //row 4
            ],
            rows: 4,
            cols: 4,
        } * Self::translation(-position)
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

// Conversions

// - From

impl From<&[&[f32]]> for Matrix {
    fn from(value: &[&[f32]]) -> Self {
        Self::from(value.iter().map(|v| v.to_vec()).collect::<Vec<Vec<f32>>>())
    }
}

impl From<Vec<Vec<f32>>> for Matrix {
    fn from(vec: Vec<Vec<f32>>) -> Self {
        let rows = vec.len();
        let cols = vec[0].len();
        let mut m = Self::zeroes(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                m[i][j] = vec[i][j];
            }
        }
        m
    }
}

impl From<Vector> for Matrix {
    fn from(value: Vector) -> Self {
        Self {
            data: value.clone().into(),
            rows: value.len(),
            cols: 1,
        }
    }
}

impl From<&Vector> for Matrix {
    fn from(value: &Vector) -> Self {
        Self {
            data: value.into(),
            rows: value.len(),
            cols: 1,
        }
    }
}
