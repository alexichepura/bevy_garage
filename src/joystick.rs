use bevy::prelude::*;
use virtual_joystick::*;

use crate::{
    car::{Car, HID},
    CarSimLabel,
};

#[derive(Component, Default)]
pub struct CarJoystick {
    last_x: f32,
    last_y: f32,
}

pub struct CarJoystickPlugin;
impl Plugin for CarJoystickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(VirtualJoystickPlugin)
            .add_startup_system(joystick_start_system)
            .add_system(update_joystick.in_set(CarSimLabel::Input));
    }
}

pub fn joystick_start_system(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("joystick/Outline_Arrows.png"),
            knob_image: asset_server.load("joystick/Outline.png"),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.,
        })
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            size: Size::all(Val::Px(100.)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(20.),
                bottom: Val::Px(20.),
                ..default()
            },
            ..default()
        }),
    )
    .insert(BackgroundColor(Color::ORANGE_RED.with_a(0.2)))
    .insert(CarJoystick::default())
    .insert(VirtualJoystickInteractionArea);
}

fn update_joystick(
    mut virtual_joystick_events: EventReader<VirtualJoystickEvent>,
    mut cars: Query<&mut Car, With<HID>>,
    mut car_joysticks: Query<&mut CarJoystick, With<CarJoystick>>,
) {
    for mut car in cars.iter_mut() {
        let mut last_x = 0.;
        let mut last_y = 0.;
        for j in virtual_joystick_events.iter() {
            let Vec2 { x, y } = j.axis();
            println!("x{x}, y{y}");
            last_x = x;
            last_y = y;

            car.steering = x;
            if y < 0. {
                car.brake = -y / 0.75;
                car.gas = 0.;
            } else {
                car.gas = y / 0.75;
                car.brake = 0.;
            }
        }
        for mut cj in car_joysticks.iter_mut() {
            // workaround to reset action
            if last_x == 0. && cj.last_x != 0. && last_y == 0. && cj.last_y != 0. {
                println!("last_reset");
                cj.last_x = 0.;
                cj.last_y = 0.;
                car.steering = 0.;
                car.gas = 0.;
                car.brake = 0.;
            } else {
                cj.last_x = last_x;
                cj.last_y = last_y;
            }
        }
    }
}
