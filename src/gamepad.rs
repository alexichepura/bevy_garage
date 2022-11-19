use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Resource, Default)]
pub struct GamepadLobby {
    pub gamepads: HashSet<Gamepad>,
}

pub fn gamepad_stage_preupdate_system(
    mut lobby: ResMut<GamepadLobby>,
    mut gamepad_event: EventReader<GamepadEvent>,
) {
    for event in gamepad_event.iter() {
        match &event.event_type {
            GamepadEventType::Connected => {
                info!("{:?} Connected", event.gamepad);
                lobby.gamepads.insert(event.gamepad);
            }
            GamepadEventType::Disconnected => {
                info!("{:?} Disconnected", event.gamepad);
                lobby.gamepads.remove(&event.gamepad);
            }
            GamepadEventType::ButtonChanged(button_type, value) => {
                info!(
                    "{:?} of {:?} is changed to {}",
                    button_type, event.gamepad, value
                );
            }
            GamepadEventType::AxisChanged(axis_type, value) => {
                info!(
                    "{:?} of {:?} is changed to {}",
                    axis_type, event.gamepad, value
                );
            }
        }
    }
}
