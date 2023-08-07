use crate::CarSet;
use bevy::prelude::*;
use bevy_garage_car::{Car, Player};
use virtual_joystick::*;

#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
enum JoystickTypeAxis {
    #[default]
    X,
    Y,
}

pub struct CarJoystickPlugin;
impl Plugin for CarJoystickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VirtualJoystickPlugin::<JoystickTypeAxis>::default())
            .add_systems(Startup, joystick_start_system)
            .add_systems(Update, update_joystick.in_set(CarSet::Input));
    }
}

const MARGIN: Val = Val::Px(35.);
const KNOB_SIZE: Vec2 = Vec2::new(70., 70.);
const AREA_SIZE: Val = Val::Px(150.);
const BG: BackgroundColor = BackgroundColor(Color::rgba(1.0, 0.27, 0.0, 0.1));

pub fn joystick_start_system(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn((
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("joystick/Horizontal_Outline_Arrows.png"),
            knob_image: asset_server.load("joystick/Outline.png"),
            knob_size: KNOB_SIZE,
            dead_zone: 0.,
            id: JoystickTypeAxis::X,
            axis: VirtualJoystickAxis::Horizontal,
            behaviour: VirtualJoystickType::Fixed,
        })
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            width: AREA_SIZE,
            height: AREA_SIZE,
            position_type: PositionType::Absolute,
            left: MARGIN,
            bottom: MARGIN,
            ..default()
        }),
        BG,
        VirtualJoystickInteractionArea,
    ));

    cmd.spawn((
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("joystick/Vertical_Outline_Arrows.png"),
            knob_image: asset_server.load("joystick/Outline.png"),
            knob_size: KNOB_SIZE,
            dead_zone: 0.,
            id: JoystickTypeAxis::Y,
            axis: VirtualJoystickAxis::Vertical,
            behaviour: VirtualJoystickType::Fixed,
        })
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            width: AREA_SIZE,
            height: AREA_SIZE,
            position_type: PositionType::Absolute,
            right: MARGIN,
            bottom: MARGIN,
            ..default()
        }),
        BG,
        VirtualJoystickInteractionArea,
    ));
}

fn update_joystick(
    mut virtual_joystick_events: EventReader<VirtualJoystickEvent<JoystickTypeAxis>>,
    mut cars: Query<&mut Car, With<Player>>,
) {
    for mut car in cars.iter_mut() {
        for j in virtual_joystick_events.iter() {
            let Vec2 { x, y } = j.axis();
            // println!("x{x}, y{y}");
            match j.id() {
                JoystickTypeAxis::X => {
                    car.steering = x;
                }
                JoystickTypeAxis::Y => {
                    if y < 0. {
                        car.brake = -y;
                        car.gas = 0.;
                    } else {
                        car.gas = y;
                        car.brake = 0.;
                    }
                }
            }
        }
    }
}
