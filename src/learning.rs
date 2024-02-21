#[derive(Debug, Clone)]
pub struct Data(pub Vec<f64>);

impl Data {
    pub fn new<T: Into<f64>>(data: Vec<T>)-> Data {
        Data(data.into_iter().map(|x| x.into()).collect())
    }
}

pub struct Example {
    pub input: Data,
    pub target: Data,
}

impl Example {
    pub fn new(input: Data, target: Data) -> Example {
        Example {
            input,
            target,
        }
    }
}
pub struct DataSet(pub Vec<Example>);
