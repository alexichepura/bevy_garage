use bevy::{
    app::AppExit,
    prelude::*,
    xr::{XrSessionMode, XrSystem},
};
use bevy_rapier_car_sim::car_app;

pub fn game_main() {
    let mut app = App::new();
    app.add_startup_system(xr_startup);
    car_app(&mut app).run();
}

fn xr_startup(mut xr_system: ResMut<XrSystem>, mut app_exit_events: EventWriter<AppExit>) {
    if xr_system.is_session_mode_supported(XrSessionMode::ImmersiveVR) {
        xr_system.request_session_mode(XrSessionMode::ImmersiveVR);
    } else {
        bevy::log::error!("The XR device does not support immersive VR mode");
        app_exit_events.send(AppExit)
    }

    println!("startup done");
}
