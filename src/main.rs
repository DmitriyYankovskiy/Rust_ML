pub mod lib;

use lib::activation::SIGMOID;
use lib::network::Network;

fn main() {
    let saves_file: &String = &"save.dat".to_string();

    let mut network = Network::new(vec![2, 10, 10, 1], 0.2, SIGMOID);

    let inputs = vec![vec![1.0, 0.0], vec![0.0, 0.0], vec![0.0, 1.0]];

    let targets = vec![vec![1.0], vec![0.0], vec![1.0]];

    let epoch = 10000;

    network.load(saves_file);

    print!("{:?}", network.feed_forward(inputs[0].clone()));
    network.train(inputs.clone(), targets.clone(), epoch);

    network.save(saves_file);
}
