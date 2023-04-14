use std::iter::Sum;
use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;

#[derive(Clone, PartialEq, Eq, Debug, Default, PartialOrd, Ord)]
pub struct Vector<T> {
    data: Vec<T>,
}

#[macro_export]
macro_rules! vector {
    ( $( $x:expr ),* ) => {
        Vector::from_vec(vec![ $( $x ),* ])
    };
}

impl<T> Vector<T> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        Self { data: v }
    }
}

// Vector Sum

impl<T: Add<Output = T> + Copy> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Vector<T> {
        Vector::from_iter((0..self.data.len()).map(|i| match i < other.data.len() {
            true => self[i] + other[i],
            false => self[i],
        }))
    }
}

impl<'a, T: Add<Output = T> + Copy> Add<&'a Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: &'a Vector<T>) -> Vector<T> {
        self + other.clone()
    }
}

impl<'a, T: Add<Output = T> + Copy> Add<Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Vector<T> {
        self.clone() + other
    }
}

impl<'a, 'b, T: Add<Output = T> + Copy> Add<&'a Vector<T>> for &'b Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: &'a Vector<T>) -> Vector<T> {
        self.clone() + other.clone()
    }
}

// Scalar Product

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, scalar: T) -> Vector<T> {
        Vector::from_iter(self.data.iter().map(|&x| x * scalar))
    }
}

impl<'a, T: Mul<Output = T> + Copy> Mul<&'a T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, scalar: &'a T) -> Vector<T> {
        self * *scalar
    }
}

impl<'a, T: Mul<Output = T> + Copy> Mul<T> for &'a Vector<T> {
    type Output = Vector<T>;

    fn mul(self, scalar: T) -> Vector<T> {
        self.clone() * scalar
    }
}

impl<'a, 'b, T: Mul<Output = T> + Copy> Mul<&'a T> for &'b Vector<T> {
    type Output = Vector<T>;

    fn mul(self, scalar: &'a T) -> Vector<T> {
        self.clone() * *scalar
    }
}

// Dot Product

impl<T: Mul<Output = T> + Sum + Copy> Mul<Vector<T>> for Vector<T> {
    type Output = T;

    fn mul(self, other: Vector<T>) -> T {
        (0..self.len()).map(|i| self[i] * other[i]).sum()
    }
}

impl<'a, T: Mul<Output = T> + Sum + Copy> Mul<&'a Vector<T>> for Vector<T> {
    type Output = T;

    fn mul(self, other: &'a Vector<T>) -> T {
        self * other.clone()
    }
}

impl<'a, T: Mul<Output = T> + Sum + Copy> Mul<Vector<T>> for &'a Vector<T> {
    type Output = T;

    fn mul(self, other: Vector<T>) -> T {
        self.clone() * other
    }
}

impl<'a, 'b, T: Mul<Output = T> + Sum + Copy> Mul<&'a Vector<T>> for &'b Vector<T> {
    type Output = T;

    fn mul(self, other: &'a Vector<T>) -> T {
        self.clone() * other.clone()
    }
}

// Cross product

//...

// Indexing

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }
}

// Iterators

impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vector<T> {
        let data: Vec<T> = iter.into_iter().collect();
        Vector { data }
    }
}
