use rand :: Rng;

use std::ops:: {
    SubAssign, 
    Mul, 
    RemAssign,
    Not,
    AddAssign,
};

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
                println!("rnd - {}", matrix.data[i][j]);
            }
        }

        matrix
    }

    pub fn map(&mut self, function: &dyn Fn(f64) -> f64) -> Matrix {
        let mut matrix = self.clone();
        
        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                matrix.data[i][j] = function(matrix.data[i][j]);
            }
        }

        matrix
    }
}

impl<'a> AddAssign<&'a Matrix> for Matrix {
    fn add_assign(&mut self, other: &'a Matrix) {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add by matrix of incorrect dimensions");
        }
        for i in 0..self.rows {
            for j in 0..self.cols {                
               self.data[i][j] += other.data[i][j];                
            }
        }
    }
}

impl<'a, 'b> Mul<&'b Matrix> for &'b Matrix {
    type Output = Matrix;
    fn mul (self, other: &'b Matrix) -> Matrix{
        if self.cols != other.rows {
            panic!("Attempted to multiply by matrix of incorrect dimensions {}:{}", self.cols, other.rows);
        }

        let mut matrix = Matrix::new(self.rows, other.cols);

        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                for k in 0..self.cols {
                    matrix.data[i][j] = self.data[i][k] * other.data[k][j];
                }
            }
        }

        matrix
    }
}

impl<'a> SubAssign<&'a Matrix> for Matrix {
    fn sub_assign (&mut self, other: &'a Matrix) {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to subtrct by matrix of incorrect dimensions");
        }
        for i in 0..self.rows {
            for j in 0..self.cols {                
               self.data[i][j] -= other.data[i][j];                
            }
        }
    }
}

impl<'a> RemAssign<&'a Matrix> for Matrix {
    fn rem_assign (&mut self, other: &'a Matrix) {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to dot multiply by matrix of incorrect dimensions");
        }

        for i in 0..self.rows {
            for j in 0..self.cols {                
                self.data[i][j] *= other.data[i][j];                
            }
        }
    }
}

impl<'a> Not for &'a Matrix {
    type Output = Matrix;
    fn not (self) -> Matrix {
        let mut matrix = Matrix::new(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {                
                matrix.data[j][i] = self.data[i][j];                
            }
        }

        matrix
    }
}