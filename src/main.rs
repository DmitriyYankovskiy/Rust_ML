use lib::network::Network;
use lib::matrix::Matrix;
use lib::activation::SIGMOID;

pub mod lib;

fn main() {

    let mut network = Network::new(vec![1, 2, 2, 1], 0.2, SIGMOID);
    
    let inputs = vec![
        vec![1.0],
        vec![0.0],        
    ];

    let targets = vec![
        vec![1.0],
        vec![0.0],
    ];

    let epoch = 10000;

    network.train(inputs.clone(), targets.clone(), epoch);

    for i in 0..inputs.len() {
        println!("{:?}", network.feed_forward(inputs[i].clone()));
    }

    let mut m = Matrix::from(vec![vec![0.0, 1.0]]);
    
    m.map(&|x| x + 1.0);

    println!("{}", m.data[0][0]);
}
