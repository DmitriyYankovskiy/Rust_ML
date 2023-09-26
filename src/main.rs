use lib::network::Network;
use lib::activation::SIGMOID;

pub mod lib;

fn main() {

    let mut network = Network::new(vec![1, 2, 1], 0.2, SIGMOID);
    
    let input = vec![
        vec![1.0],
        vec![0.0],        
    ];

    let target = vec![
        vec![1.0],
        vec![0.0],
    ];

    let epoch = 1000;

    network.train(input.clone(), target.clone(), epoch);

    for i in 0..4 {
        println!("{:?}", network.feed_forward(input[i].clone()));
    }
}
