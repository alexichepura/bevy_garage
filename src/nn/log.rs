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
