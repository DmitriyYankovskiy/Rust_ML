pub struct Test {
    pub input: Vec<f64>,
    pub target: Vec<f64>,
}

impl Test {
    pub fn new(input: Vec<f64>, target: Vec<f64>) -> Test {
        Test {
            input,
            target,
        }
    }
}
