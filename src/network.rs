use crate::{learning::Data, DataSet};

use super::activation::Activation;
use super::matrix::Matrix;
use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

pub struct Network<'a> {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    learning_rate: f64,
    activation: Activation<'a>,
}

#[derive(Serialize, Deserialize)]
struct SaveData {
    weights: Vec<Vec<Vec<f64>>>,
    biases: Vec<Vec<Vec<f64>>>,
}

impl<'a> Network<'a> {
    pub fn new(layers: Vec<usize>, learning_rate: f64, activation: Activation<'a>) -> Network<'a> {
        let mut weights: Vec<Matrix> = vec![];
        let mut biases: Vec<Matrix> = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], 1));
        }

        Network {
            layers,
            weights,
            biases,
            data: vec![],
            learning_rate,
            activation,
        }
    }

    pub fn predict(&mut self, Data(input): Data) -> Data {
        if input.len() != self.layers[0] {
            panic!(
                "Attempted to feed forward input {}, but input layer has {} neuron",
                input.len(),
                self.layers[0]
            );
        }
        let mut current = Matrix::from(vec![input]).rev();
        self.data = vec![];

        for i in 0..self.layers.len() - 1 {
            self.data.push(current);
            
            current = self.weights[i].mul(&self.data.last().unwrap());
            current.add(&self.biases[i]);
            current.map(&self.activation.function);
        }
        
        self.data.push(current.clone());
        Data(current.rev()[0].to_owned())
    }

    pub fn back_propogate(&mut self, Data(targets): Data) {
        let mut errors = Matrix::from(vec![targets]).rev();

        {
            let output = &self.data[self.layers.len() - 1];
            errors.sub(output);
        }

        for i in (0..self.layers.len() - 1).rev() {
            let gradient: &mut Matrix;
            let front: &mut Matrix;

            {
                let (left, right) = self.data.split_at_mut(i);
                gradient = right.first_mut().unwrap();
                front = left.first_mut().unwrap();
            }

            gradient.map(&self.activation.derivative);
            gradient.dot(&errors);
            gradient.map(&|x| x * self.learning_rate);

            self.weights[i].add(&gradient.mul(&front.rev()));
            self.biases[i].add(gradient);

            errors = self.weights[i].rev().mul(&errors);
        }
    }

    pub fn train(&mut self, DataSet(tests): DataSet, epoch: usize) {
        for i in 1..=epoch {
            if i % 100 == 0 {
                println!("~~~ train {} ~~~", i);
            }

            for j in 0..tests.len() {
                let _ = self.predict(tests[j].input.clone());
                self.back_propogate(tests[j].target.clone());
            }
        }
    }

    pub fn save(&self, file: &String) {
        let mut file = File::create(file).expect("Unable to touch save file");

        file.write_all(
			json!({
				"weights": self.weights.clone().into_iter().map(|matrix| matrix.data).collect::<Vec<Vec<Vec<f64>>>>(),
				"biases": self.biases.clone().into_iter().map(|matrix| matrix.data).collect::<Vec<Vec<Vec<f64>>>>()
			}).to_string().as_bytes(),
		).expect("Unable to write to save file");
    }

    pub fn load(&mut self, file: &String) {
        let mut file = File::open(file).expect("Unable to open save file");
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .expect("Unable to read save file");

        let save_data: SaveData = from_str(&buffer).expect("Unable to serialize save data");

        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..self.layers.len() - 1 {
            weights.push(Matrix::from(save_data.weights[i].clone()));
            biases.push(Matrix::from(save_data.biases[i].clone()));
        }

        self.weights = weights;
        self.biases = biases;
    }
}
