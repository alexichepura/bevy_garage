use std::time::Instant;

pub fn log_training(
    use_random: bool,
    action: usize,
    reward: f32,
    loss_string: &String,
    start: Instant,
) {
    let log = [
        String::from(if use_random { "?" } else { " " }),
        action.to_string(),
        " ".to_string(),
        String::from(if reward > 0. { "+" } else { "-" }),
        format!("{:.2}", reward.abs()),
        " ".to_string(),
        start.elapsed().as_micros().to_string() + "μs",
        " ".to_string(),
        loss_string.to_string(),
    ]
    .join("");
    println!("{log:?}");
}
pub fn log_action_reward(action: usize, reward: f32) {
    let log = [
        action.to_string(),
        " ".to_string(),
        String::from(if reward > 0. { "+" } else { "-" }),
        format!("{:.2}", reward.abs()),
    ]
    .join("");
    println!("{log:?}");
}

const ONE: f32 = 1.;
const ZERO: f32 = 0.;
pub fn map_action_to_car(a: usize) -> (f32, f32, f32, f32) {
    let gas = if a == 0 || a == 4 || a == 5 {
        ONE
    } else {
        ZERO
    };
    let brake = if a == 1 || a == 6 || a == 7 {
        ONE
    } else {
        ZERO
    };
    let left = if a == 2 || a == 4 || a == 6 {
        ONE
    } else {
        ZERO
    };
    let right = if a == 3 || a == 5 || a == 7 {
        ONE
    } else {
        ZERO
    };
    (gas, brake, left, right)
}
