// use bevy::prelude::*;
use rand::thread_rng;
use rand::{distributions::Standard, Rng};

#[derive(Debug)]
struct CarBrain {
    levels: Vec<Level>,
}
impl CarBrain {
    pub fn new() -> CarBrain {
        // let levels: Vec<Level> = [(); 3].map(|_| Level::new(10)).to_vec();
        CarBrain {
            levels: [Level::new(5, 6), Level::new(6, 4), Level::new(4, 0)].to_vec(),
        }
    }
}
#[derive(Debug, Clone)]
struct Level {
    inputs: Vec<f64>,
    outputs: Vec<f64>,
    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
}

impl Level {
    pub fn new(n_in: usize, n_out: usize) -> Level {
        let inputs: Vec<f64> = vec![0.; n_in];
        let outputs: Vec<f64> = vec![0.; n_out];
        let weights: Vec<Vec<f64>> = (0..n_in)
            .map(|_| thread_rng().sample_iter(Standard).take(n_out).collect())
            .collect();
        let biases: Vec<f64> = thread_rng().sample_iter(Standard).take(n_out).collect();

        Level {
            weights,
            biases,
            inputs,
            outputs,
        }
    }
    pub fn feed_forward(mut self, new_inputs: Vec<f64>) {
        for (index, input) in self.inputs.iter_mut().enumerate() {
            *input = new_inputs[index];
        }
        for (index_out, output) in self.outputs.iter_mut().enumerate() {
            let mut sum: f64 = 0.;
            for (index_in, input) in self.inputs.iter_mut().enumerate() {
                sum = sum + *input * self.weights[index_in][index_out];
            }
            if sum > self.biases[index_out] {
                *output = 1.;
            } else {
                *output = 0.;
            }
        }
    }
}
pub fn brain_system() {
    let net = CarBrain::new();
    println!("{:?}", net);
}
