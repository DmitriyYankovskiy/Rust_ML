pub mod lib;

use lib::network::Network;
use lib::activation::SIGMOID;

fn main() {

    let mut network = Network::new(vec![2, 10, 10, 1], 0.2, SIGMOID);
    
    let inputs = vec![
        vec![1.0, 0.0],
        vec![0.0, 0.0],
        vec![0.0, 1.0],
    ];

    let targets = vec![
        vec![1.0],
        vec![0.0],
        vec![1.0],
    ];

    let epoch = 10000;

    network.train(inputs.clone(), targets.clone(), epoch);

    for i in 0..inputs.len() {
        println!("{:?}", network.feed_forward(inputs[i].clone()));
    }

    println!("{:?}", network.feed_forward(vec![1.0, 1.0]));
}
