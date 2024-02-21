#[derive(Debug, Clone)]
pub struct Data(pub Vec<f64>);

impl Data {
    pub fn new<T: Into<f64>>(data: Vec<T>)-> Self {
        Self(data.into_iter().map(|x| x.into()).collect())
    }

    pub fn view(&self) {
        for i in self.0.iter() {
            print!("{} ", i.round());
        }
        println!();
    }

    pub fn error(&self) {
        for i in self.0.iter() {
            print!("{} ", i);
        }
        println!();
    }
}

pub struct Example {
    pub input: Data,
    pub target: Data,
}

impl Example {
    pub fn new(input: Data, target: Data) -> Self {
        Self {
            input,
            target,
        }
    }
}
pub struct DataSet(pub Vec<Example>);

impl DataSet {
    pub fn from_table<T1: Into<f64>, T2: Into<f64>>(value: Vec<(Vec<T1>, Vec<T2>)>) -> Self {
        Self (
            value.into_iter().map(|(input, target)| Example::new(Data::new(input), Data::new(target))).collect()
        )
    }
}
