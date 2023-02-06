use bevy::prelude::*;

#[bevy_main]
fn main() {
    bevy_garage_xr::game_main();
}

#[cfg(target_os = "android")]
compile_error!("Use the `--example android` flag to compile for quest.");
