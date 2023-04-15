use bevy::prelude::*;
use virtual_joystick::*;

use crate::car::{Car, HID};

pub struct CarJoystickPlugin;
impl Plugin for CarJoystickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(VirtualJoystickPlugin)
            .add_startup_system(joystick_start_system)
            .add_system(update_joystick);
    }
}

pub fn joystick_start_system(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("joystick/Horizontal_Outline_Arrows.png"),
            knob_image: asset_server.load("joystick/Outline.png"),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.,
        })
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            size: Size::all(Val::Px(150.)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(50.),
                bottom: Val::Px(50.),
                ..default()
            },
            ..default()
        }),
    )
    .insert(BackgroundColor(Color::ORANGE_RED.with_a(0.3)))
    .insert(VirtualJoystickInteractionArea);
    cmd.spawn(
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("joystick/Vertical_Outline_Arrows.png"),
            knob_image: asset_server.load("joystick/Outline.png"),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.,
        })
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            size: Size::all(Val::Px(150.)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(50.),
                bottom: Val::Px(50.),
                ..default()
            },
            ..default()
        }),
    )
    .insert(BackgroundColor(Color::ORANGE_RED.with_a(0.3)))
    .insert(VirtualJoystickInteractionArea);
}

fn update_joystick(
    mut joystick: EventReader<VirtualJoystickEvent>,
    mut cars: Query<&mut Car, With<HID>>,
) {
    for j in joystick.iter() {
        let Vec2 { x, y } = j.axis();
        println!("x{x}, y{y}");
        for mut car in cars.iter_mut() {
            car.steering = x;
        }
    }
}
