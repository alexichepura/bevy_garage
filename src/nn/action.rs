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
