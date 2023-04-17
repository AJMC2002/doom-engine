use std::ops::{Index, IndexMut};

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
    pub fn empty(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
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

    pub fn identity(rows: usize, cols: usize) -> Self {
        let mut m = Matrix::empty(rows, cols);
        for i in 0..rows {
            m[i][i] = 1.0;
        }
        m
    }

    pub fn from_vec(data: Vec<Vec<f32>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let mut m = Matrix::empty(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                m[i][j] = data[i][j];
            }
        }
        m
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
