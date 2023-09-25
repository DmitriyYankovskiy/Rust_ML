use super::{matrix::Matrix, activation::Activation};

pub struct Network<'a> {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Vec<f64>>,    
    activation: Activation<'a>,
}

impl<'a> Network<'a> {
    pub fn new(layers: Vec<usize>, activation: Activation<'a>) -> Network<'a>{
        let mut weights: Vec<Matrix> = vec![];
        let mut biases: Vec<Matrix> = vec![];
        let mut data: Vec<Vec<f64>> = vec![vec![0.0; layers[0]]; 1];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], 1));
            data.push(vec![0.0; layers[i+1]]);
        }

        Network { 
            layers,
            weights,
            biases,
            data,
            activation,
        }
    }

    pub fn feed_forward(&mut self, input: Vec<f64>) -> Vec<f64> {
        if input.len() != self.data[0].len() {
            panic!("Attempted to feed forward input {}, but input layer has {} neuron", input.len(), self.data[0].len());
        }

        let mut current = !&Matrix::from(vec![input]); 

        for i in 0..self.layers.len() - 1 {
            current = &(&self.weights[i] * &current) + &self.biases[i];            
        }

        return (!&current).data[0].clone();
    }

    pub fn back_propogation(&mut self, output: Vec<f64>, target: Vec<f64>) {
        let error = &!&Matrix::from(vec![target]) - &!&Matrix::from(vec![output]);

        for i in (0..self.layers.len()).rev() {
            
        }
    }
}