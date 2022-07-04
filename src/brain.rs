// use bevy::prelude::*;
use rand::thread_rng;
use rand::{distributions::Standard, Rng};

struct Net {
    levels: Vec<Level>,
}
impl Net {
    pub fn new() -> Net {
        // let levels: Vec<Level> = [(); 3].map(|_| Level::new(10)).to_vec();
        Net {
            levels: [Level::new(5, 6), Level::new(6, 4), Level::new(4, 0)].to_vec(),
        }
    }
}
#[derive(Clone)]
struct Level {
    inputs: Vec<f64>,
    outputs: Vec<f64>,
    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
}

impl Level {
    pub fn new(n_inputs: usize, n_outputs: usize) -> Level {
        let weights: Vec<f64> = thread_rng().sample_iter(Standard).take(n_inputs).collect();
        let biases: Vec<f64> = thread_rng().sample_iter(Standard).take(n_inputs).collect();
        let inputs: Vec<f64> = vec![0.; n_inputs];
        let outputs: Vec<f64> = vec![0.; n_outputs];

        Level {
            weights,
            biases,
            inputs,
            outputs,
        }
    }
}
pub fn brain_system() {
    let mut net = Net::new(20);
    // net.train(&training_data, 100000, 100, 0.000001);
    // brain=new NeuralNetwork(
    //     [this.sensor.rayCount,6,4]
    // )
}
