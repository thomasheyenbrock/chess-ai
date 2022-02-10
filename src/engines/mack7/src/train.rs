use neuronika::{data::DataLoader, nn::loss, optim};
use std::fs::File;
use std::io::Read;

use crate::{policy_network, value_network};

pub fn run(run_indices: Vec<&str>) -> std::io::Result<()> {
    println!("Loading training data");

    let mut value_csv: String = String::from("");
    let mut policy_csv: String = String::from("");

    for run_index in run_indices {
        let mut value_file = File::open(format!("value.{}.csv", run_index))?;
        let mut value_contents = String::new();
        value_file.read_to_string(&mut value_contents)?;
        value_csv.push_str(&value_contents);

        let mut policy_file = File::open(format!("policy.{}.csv", run_index))?;
        let mut policy_contents = String::new();
        policy_file.read_to_string(&mut policy_contents)?;
        policy_csv.push_str(&policy_contents);
    }

    // Creates data loader.
    let mut value_dataset = DataLoader::default()
        .with_labels(&[837])
        .without_headers()
        .from_reader(value_csv.as_bytes(), 837, 1);
    let mut policy_dataset = DataLoader::default()
        .with_labels(&(837..2809).collect::<Vec<usize>>())
        .without_headers()
        .from_reader(policy_csv.as_bytes(), 837, 1972);

    let value_nn = value_network::load()?;
    value_nn.train();

    // Creates the optimizer.
    let value_optimizer = optim::SGD::new(value_nn.parameters(), 0.01, optim::L2::new(0.0));

    // Trains the model.
    for epoch in 0..100 {
        let batched_data = value_dataset.shuffle().batch(100).drop_last().take(10);
        let mut total_loss = 0.;

        for (input_array, target_array) in batched_data {
            let input = neuronika::from_ndarray(input_array.to_owned());
            let target = neuronika::from_ndarray(target_array.to_owned());

            let result = value_nn.forward(input);

            let loss = loss::mse_loss(result.clone(), target.clone(), loss::Reduction::Mean);
            loss.forward();
            total_loss += loss.data()[()];
            loss.backward(1.0);
            value_optimizer.step();
        }

        println!("Value network loss for epoch {} : {} ", epoch, total_loss);
    }
    value_network::save(&value_nn)?;

    let policy_nn = policy_network::load()?;
    policy_nn.train();

    // Creates the optimizer.
    let policy_optimizer = optim::SGD::new(policy_nn.parameters(), 0.01, optim::L2::new(0.0));

    // Trains the model.
    for epoch in 0..100 {
        let batched_data = policy_dataset.shuffle().batch(100).drop_last().take(10);
        let mut total_loss = 0.;

        for (input_array, target_array) in batched_data {
            let input = neuronika::from_ndarray(input_array.to_owned());
            let target = neuronika::from_ndarray(target_array.to_owned());

            let result = policy_nn.forward(input);

            let loss = loss::mse_loss(result.clone(), target.clone(), loss::Reduction::Mean);
            loss.forward();
            total_loss += loss.data()[()];
            loss.backward(1.0);
            policy_optimizer.step();
        }

        println!("Policy network loss for epoch {} : {} ", epoch, total_loss);
    }
    policy_network::save(&policy_nn)?;

    Ok(())
}
