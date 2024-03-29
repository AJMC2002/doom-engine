use cgmath::num_traits::ToPrimitive;
use std::ops;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::MulAssign;
use std::ops::SubAssign;

use super::Matrix;
use crate::matrix;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Vector {
    data: Vec<f32>,
}

#[macro_export]
macro_rules! vector {
    ( $( $x:expr ),* ) => {
        $crate::maths::Vector::from(vec![ $( $x ),* ])
    };
}

impl Vector {
    pub fn new(n: usize, val: f32) -> Self {
        Vector::from(vec![val; n])
    }

    pub fn zeroes(n: usize) -> Self {
        Vector::from(vec![0.; n])
    }

    pub fn ones(n: usize) -> Self {
        Vector::from(vec![1.; n])
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn abs(&self) -> f32 {
        self.clone().into_iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    pub fn angle(&self, other: &Vector) -> f32 {
        (self.unit() * other.unit()).acos()
    }

    pub fn unit(&self) -> Self {
        self / self.abs()
    }

    pub fn cross(&self, other: &Vector) -> Self {
        assert_eq!(self.len(), 3);
        assert_eq!(other.len(), 3);
        Vector::from(vec![
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }

    // Vec3 Ops

    pub fn scale(&mut self, v: Vector) {
        assert_eq!(self.len(), 4);
        assert_eq!(v.len(), 3);
        self[0] *= v[0];
        self[1] *= v[1];
        self[2] *= v[2];
    }

    pub fn translate(&mut self, v: Vector) {
        assert_eq!(self.len(), 4);
        assert_eq!(v.len(), 3);
        self[0] += v[0];
        self[1] += v[1];
        self[2] += v[2];
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let rot_m = matrix![
            [1., 0., 0., 0.],
            [0., angle.cos(), -angle.sin(), 0.],
            [0., angle.sin(), angle.cos(), 0.],
            [0., 0., 0., 1.]
        ];
        let ans = (rot_m * self.clone()).as_vector();
        for i in 0..self.len() {
            self[i] = ans[i];
        }
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let rot_m = matrix![
            [angle.cos(), 0., angle.sin(), 0.],
            [0., 1., 0., 0.],
            [-angle.sin(), 0., angle.cos(), 0.],
            [0., 0., 0., 1.]
        ];
        let ans = (rot_m * self.clone()).as_vector();
        for i in 0..self.len() {
            self[i] = ans[i];
        }
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let rot_m = matrix![
            [angle.cos(), -angle.sin(), 0., 0.],
            [angle.sin(), angle.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.]
        ];
        let ans = (rot_m * self.clone()).as_vector();
        for i in 0..self.len() {
            self[i] = ans[i];
        }
    }

    // Conversion

    pub fn as_slice(&self) -> &[f32] {
        &self.data
    }

    pub fn as_matrix(&self) -> Matrix {
        Matrix::from_vector(self)
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }
}

// Unary Ops

impl_op_ex!(-|vector: &Vector| -> Vector {
    Vector::from_iter((0..vector.len()).map(|i| -vector[i]))
});

// Scalar Ops

impl_op_ex_commutative!(+ |vector: &Vector, scalar: &f32| -> Vector {
    Vector::from_iter((0..vector.len()).map(|i| vector[i] + scalar))
});

impl_op_ex_commutative!(+ |vector: &Vector, scalar: &i32| -> Vector {
    Vector::from_iter((0..vector.len()).map(|i| vector[i] + *scalar as f32))
});

impl_op_ex!(-|vector: &Vector, scalar: &f32| -> Vector { vector + (-scalar) });

impl_op_ex!(-|vector: &Vector, scalar: &i32| -> Vector { vector + (-scalar as f32) });

impl_op_ex!(-|scalar: &f32, vector: &Vector| -> Vector { scalar + (-vector) });

impl_op_ex!(-|scalar: &i32, vector: &Vector| -> Vector { *scalar as f32 + (-vector) });

impl_op_ex_commutative!(*|vector: &Vector, scalar: &f32| -> Vector {
    Vector::from_iter((0..vector.len()).map(|i| vector[i] * scalar))
});

impl_op_ex_commutative!(*|vector: &Vector, scalar: &i32| -> Vector {
    Vector::from_iter((0..vector.len()).map(|i| vector[i] * *scalar as f32))
});

impl_op_ex!(/ |vector: &Vector, scalar: &f32| -> Vector {
    Vector::from_iter((0..vector.len()).map(|i| vector[i] / scalar))
});

impl_op_ex!(/ |vector: &Vector, scalar: &i32| -> Vector {
    Vector::from_iter((0..vector.len()).map(|i| vector[i] / *scalar as f32))
});

// Vector Ops

impl_op_ex!(+ |lhs: &Vector, rhs: &Vector| -> Vector {
    assert_eq!(lhs.len(), rhs.len());
    Vector::from_iter((0..lhs.len()).map(|i| lhs[i] + rhs[i]))
});

impl_op_ex!(-|lhs: &Vector, rhs: &Vector| -> Vector { lhs + (-rhs) });

impl_op_ex!(*|lhs: &Vector, rhs: &Vector| -> f32 {
    assert_eq!(lhs.len(), rhs.len());
    (0..lhs.len()).map(|i| lhs[i] * rhs[i]).sum()
});

// Scalar Ops Assignment

impl<T: ToPrimitive> AddAssign<T> for Vector {
    fn add_assign(&mut self, scalar: T) {
        let scalar_f32 = scalar.to_f32().unwrap();
        for i in 0..self.len() {
            self[i] += scalar_f32;
        }
    }
}

impl<T: ToPrimitive> SubAssign<T> for Vector {
    fn sub_assign(&mut self, scalar: T) {
        let scalar_f32 = scalar.to_f32().unwrap();
        for i in 0..self.len() {
            self[i] -= scalar_f32;
        }
    }
}

impl<T: ToPrimitive> MulAssign<T> for Vector {
    fn mul_assign(&mut self, scalar: T) {
        let scalar_f32 = scalar.to_f32().unwrap();
        for i in 0..self.len() {
            self[i] *= scalar_f32;
        }
    }
}

impl<T: ToPrimitive> DivAssign<T> for Vector {
    fn div_assign(&mut self, scalar: T) {
        let scalar_f32 = scalar.to_f32().unwrap();
        for i in 0..self.len() {
            self[i] /= scalar_f32;
        }
    }
}

// Vector Ops Assignment

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.len(), rhs.len());
        for i in 0..self.len() {
            self[i] += rhs[i];
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.len(), rhs.len());
        for i in 0..self.len() {
            self[i] -= rhs[i];
        }
    }
}

// Indexing

impl Index<usize> for Vector {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Conversions

// - From

impl From<&[f32]> for Vector {
    fn from(value: &[f32]) -> Self {
        Vector {
            data: value.to_vec(),
        }
    }
}

impl From<Vec<f32>> for Vector {
    fn from(value: Vec<f32>) -> Self {
        Vector { data: value }
    }
}

impl From<Matrix> for Vector {
    fn from(value: Matrix) -> Self {
        value.as_vector()
    }
}

impl From<&Matrix> for Vector {
    fn from(value: &Matrix) -> Self {
        value.as_vector()
    }
}

impl FromIterator<f32> for Vector {
    fn from_iter<I: IntoIterator<Item = f32>>(iter: I) -> Vector {
        Vector {
            data: iter.into_iter().collect(),
        }
    }
}

impl FromIterator<i32> for Vector {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Vector {
        Vector {
            data: iter.into_iter().map(|x| x as f32).collect(),
        }
    }
}

// - Into / Reverse From

impl From<Vector> for Vec<f32> {
    fn from(val: Vector) -> Self {
        val.data
    }
}

impl From<&Vector> for Vec<f32> {
    fn from(val: &Vector) -> Self {
        val.data.clone()
    }
}

impl IntoIterator for Vector {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl IntoIterator for &Vector {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.clone().into_iter()
    }
}
