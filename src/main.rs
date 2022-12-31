use bevy::prelude::App;
use bevy_rapier_car_sim::car_app;

fn main() {
    let mut app = App::new();
    car_app(&mut app);
    app.run();
}
