// pub fn log_training(use_random: bool, action: usize, reward: f32) {
//     let log = [
//         "train".to_string(),
//         String::from(if use_random { "?" } else { " " }),
//         action.to_string(),
//         " ".to_string(),
//         String::from(if reward > 0. { "+" } else { "-" }),
//         format!("{:.2}", reward.abs()),
//     ]
//     .join("");
//     println!("{log:?}");
// }
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
    let gas = match a {
        0 | 4 | 5 => ONE,
        _ => ZERO,
    };
    let brake = match a {
        1 | 6 | 7 => ONE,
        _ => ZERO,
    };
    let left = match a {
        2 | 4 | 6 => ONE,
        _ => ZERO,
    };
    let right = match a {
        3 | 5 | 7 => ONE,
        _ => ZERO,
    };
    (gas, brake, left, right)
}
