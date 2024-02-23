use crate::{learning::Data, DataSet};

use super::activation::Activation;
use super::matrix::Matrix;
use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};


#[derive(Clone)]
pub struct Network<'a> {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    edge_vel: Vec<Matrix>,
    neuron_vel: Vec<Matrix>,
    learning_rate: f64,
    impulse_rate: f64,
    activation: Activation<'a>,
}


#[derive(Serialize, Deserialize, Clone)]
struct SaveData {
    weights: Vec<Vec<Vec<f64>>>,
    biases: Vec<Vec<Vec<f64>>>,
    edge_vel: Vec<Vec<Vec<f64>>>,
    neuron_vel: Vec<Vec<Vec<f64>>>,
} 

impl<'a> From<Network<'a>> for SaveData {
    fn from(network: Network<'a>) -> Self {
        Self {
            weights: network.weights.iter().map(|x| x.clone().into()).collect(),
            biases: network.biases.iter().map(|x| x.clone().into()).collect(),
            edge_vel: network.edge_vel.iter().map(|x| x.clone().into()).collect(),
            neuron_vel: network.neuron_vel.iter().map(|x| x.clone().into()).collect(),
        }
    }
}

impl<'a> Network<'a> {
    pub fn new(layers: Vec<usize>, learning_rate: f64, impulse_rate: f64, activation: Activation<'a>) -> Network<'a> {
        let mut weights: Vec<Matrix> = vec![];
        let mut biases: Vec<Matrix> = vec![];
        let mut edge_vel: Vec<Matrix> = vec![];
        let mut neuron_vel: Vec<Matrix> = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], 1));
            neuron_vel.push(Matrix::new(layers[i + 1], 1));
            edge_vel.push(Matrix::new(layers[i + 1], layers[i]));
        }

        Network {
            layers,
            weights,
            biases,
            edge_vel,
            neuron_vel,
            data: vec![],
            learning_rate,
            impulse_rate,
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
            let edge_vel: &mut Matrix;
            let neuron_vel: &mut Matrix;
            let front: &mut Matrix;

            {
                edge_vel = &mut self.edge_vel[i];
                neuron_vel = &mut self.neuron_vel[i];
                let (left, right) = self.data.split_at_mut(i + 1);
                gradient = right.first_mut().unwrap();
                front = left.last_mut().unwrap();
            }

            gradient.map(&self.activation.derivative);
            gradient.dot(&errors);

            edge_vel.map(&|x| x * self.impulse_rate);
            edge_vel.add(&gradient.mul(&front.rev()));
            edge_vel.map(&|x| x * self.learning_rate);

            neuron_vel.map(&|x| x * self.impulse_rate);
            neuron_vel.add(gradient);
            neuron_vel.map(&|x| x * self.learning_rate);

            self.weights[i].add(edge_vel);
            self.biases[i].add(neuron_vel);

            errors = self.weights[i].rev().mul(&errors);
        }
    }

    pub fn train(&mut self, DataSet(tests): &DataSet, epoch: usize, is_log: bool) {
        for i in 1..=epoch {
            if (i % 100 == 0) && is_log {
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
			serde_json::to_string(&SaveData::from(self.clone())).unwrap().as_bytes(),
		).expect("Unable to write to save file");
    }

    pub fn load(&mut self, file: &String) {
        let mut file = File::open(file).expect("Unable to open save file");
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .expect("Unable to read save file");

        let save_data: SaveData = from_str(&buffer).expect("Unable to serialize save data");

        self.weights = save_data.weights.into_iter().map(|x| Matrix::from(x)).collect();
        self.biases = save_data.biases.into_iter().map(|x| Matrix::from(x)).collect();
        self.edge_vel = save_data.edge_vel.into_iter().map(|x| Matrix::from(x)).collect();
        self.neuron_vel = save_data.neuron_vel.into_iter().map(|x| Matrix::from(x)).collect();
    }
}
