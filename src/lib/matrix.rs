use rand :: Rng;

use std::ops:: {
    Add, 
    Sub, 
    Mul, 
    Index,
    Rem,
    Not
};

#[derive(Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
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
                matrix[i][j] = rand::thread_rng().gen::<f64>() * 2.0 - 1.0;
            }
        }

        matrix
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, other: Matrix) -> Matrix{
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add by matrix of incorrect dimensions");
        }

        let mut matrix = Matrix::new(self.rows, other.cols);

        for i in 0..matrix.rows {
            for j in 0..matrix.cols {                
                matrix[i][j] = self[i][j] + other[i][j];                
            }
        }

        matrix
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul (self, other: Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Attempted to multiply by matrix of incorrect dimensions");
        }

        let mut matrix = Matrix::new(self.rows, other.cols);

        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                for k in 0..self.cols {
                    matrix[i][j] += self[i][k] * other[k][j];
                }
            }
        }

        matrix
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, other: Matrix) -> Matrix{
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to subtrct by matrix of incorrect dimensions");
        }

        let mut matrix = Matrix::new(self.rows, other.cols);

        for i in 0..matrix.rows {
            for j in 0..matrix.cols {                
                matrix[i][j] = self[i][j] - other[i][j];                
            }
        }

        matrix
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;
    fn index(&self, index: usize) -> &Vec<f64> {
        &self.data[index]
    }
}

impl Rem for Matrix {
    type Output = Matrix;
    fn rem(self, other: Matrix) -> Matrix{
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to dot multiply by matrix of incorrect dimensions");
        }

        let mut matrix = Matrix::new(self.rows, other.cols);

        for i in 0..matrix.rows {
            for j in 0..matrix.cols {                
                matrix[i][j] = self[i][j] * other[i][j];                
            }
        }

        matrix
    }
}

impl Not for Matrix {
    type Output = Matrix;
    fn not(self) -> Matrix{
        let mut matrix = Matrix::new(self.cols, self.rows);

        for i in 0..matrix.rows {
            for j in 0..matrix.cols {                
                matrix[j][i] = self[i][j];                
            }
        }

        matrix
    }
}