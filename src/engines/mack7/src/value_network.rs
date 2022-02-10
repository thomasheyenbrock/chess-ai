use ndarray::Ix2;
use neuronika::{
    nn::{Learnable, Linear, ModelStatus},
    Backward, Data, Forward, Gradient, MatMatMulT, Overwrite, Param, VarDiff,
};
use std::fs::File;
use std::io::{Read, Write};

const FILENAME: &str = "nn-value.json";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ValueNetwork {
    lin1: Linear,
    lin2: Linear,
    lin3: Linear,
    lin4: Linear,
    #[serde(skip)]
    status: ModelStatus,
}

impl ValueNetwork {
    fn new() -> Self {
        let mut status = ModelStatus::default();

        Self {
            lin1: status.register(Linear::new(837, 628)),
            lin2: status.register(Linear::new(628, 419)),
            lin3: status.register(Linear::new(419, 210)),
            lin4: status.register(Linear::new(210, 1)),
            status,
        }
    }

    pub fn parameters(&self) -> Vec<Param> {
        self.status.parameters()
    }

    pub fn forward<I, T, U>(
        &self,
        input: I,
    ) -> VarDiff<impl Data<Dim = Ix2> + Forward, impl Gradient<Dim = Ix2> + Overwrite + Backward>
    where
        I: MatMatMulT<Learnable<Ix2>>,
        I::Output: Into<VarDiff<T, U>>,
        T: Data<Dim = Ix2> + Forward,
        U: Gradient<Dim = Ix2> + Backward + Overwrite,
    {
        let out1 = self.lin1.forward(input).relu();
        let out2 = self.lin2.forward(out1).relu();
        let out3 = self.lin3.forward(out2).relu();
        self.lin4.forward(out3).sigmoid()
    }

    pub fn train(&self) {
        self.status.train();
    }

    pub fn eval(&self) {
        self.status.eval();
    }
}

// ##################################################################
// Storing weights and biases

pub fn save(network: &ValueNetwork) -> std::io::Result<()> {
    let mut file = File::create(FILENAME)?;
    file.write(serde_json::to_string(network).unwrap().as_bytes())?;
    Ok(())
}

// ##################################################################
// Restore weights and biases

pub fn load() -> std::io::Result<ValueNetwork> {
    match File::open(FILENAME) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let ValueNetwork {
                lin1,
                lin2,
                lin3,
                lin4,
                mut status,
            } = serde_json::from_str(&contents).unwrap();

            let nn = ValueNetwork {
                lin1: status.register(lin1),
                lin2: status.register(lin2),
                lin3: status.register(lin3),
                lin4: status.register(lin4),
                status,
            };
            println!("Loaded value network");
            Ok(nn)
        }
        Err(_) => {
            println!("No value network saved, saving the current one");
            let model = ValueNetwork::new();
            let _saved = save(&model);
            Ok(model)
        }
    }
}
