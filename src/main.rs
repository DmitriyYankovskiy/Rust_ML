use lib::network::Network;
use lib::activation::SIGMOID;

pub mod lib;

fn main() {

    Network::new(vec![], SIGMOID);

    println!("Hello, World");
}
