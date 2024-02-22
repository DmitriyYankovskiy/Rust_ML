use rand::Rng;

use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn from(vec: Vec<Vec<f64>>) -> Matrix {
        Matrix {
            rows: vec.len(),
            cols: vec[0].len(),
            data: vec,
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut matrix = Matrix::new(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                matrix.data[i][j] = rand::thread_rng().gen::<f64>() * 2.0 - 1.0;
            }
        }

        matrix
    }

    pub fn map(&mut self, function: &dyn Fn(f64) -> f64) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = function(self.data[i][j]);
            }
        }
    }

    pub fn add(&mut self, other: &Matrix) {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add by matrix of incorrect dimensions")
        }

        let rows = self.rows;
        let cols = self.cols;

        for i in 0..rows {
            for j in 0..cols {
                self.data[i][j] += other.data[i][j];
            }
        }
    }

    pub fn sub(&mut self, other: &Matrix) {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to subtrct by matrix of incorrect dimensions");
        }

        let rows = self.rows;
        let cols = self.cols;

        for i in 0..rows {
            for j in 0..cols {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }

    pub fn mul(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!(
                "Attempted to multiply by matrix of incorrect dimensions {}:{}",
                self.cols, other.rows
            );
        }

        let rows = self.rows;
        let cols = other.cols;
        let mut matrix = Matrix::new(self.rows, other.cols);

        for i in 0..rows {
            for j in 0..cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }

                matrix.data[i][j] = sum;
            }
        }

        matrix
    }

    pub fn dot(&mut self, other: &Matrix) {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to dot multiply by matrix of incorrect dimensions");
        }

        let rows = self.rows;
        let cols = other.cols;

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] *= other.data[i][j];
            }
        }
    }

    pub fn rev(&self) -> Matrix {
        let mut matrix = Matrix::new(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                matrix.data[j][i] = self.data[i][j];
            }
        }

        matrix
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}