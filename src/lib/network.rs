use super::matrix::Matrix;

pub struct Network {
    layers: Vec<u32>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Vec<f64>>,    
}

impl Network {
    pub fn new(layers: Vec<u32>) -> Network{
        let mut weights: Vec<Matrix> = vec![];
        let mut biases: Vec<Matrix> = vec![];

        weights[0] + biases[0] ;
    }
}