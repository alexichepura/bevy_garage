#![feature(slice_flatten)]

#[cfg(feature = "api")]
pub mod api_client;

pub mod dash;
pub mod dqn;
pub mod dqn_bevy;
pub mod gradient;
pub mod params;
pub mod replay;
pub mod spawn;
pub mod util;

use crate::{dqn::dqn_system, dqn_bevy::*, spawn::*};
use bevy::prelude::{App, IntoSystemConfig, Plugin};
use bevy_garage_car::CarSet;

pub struct BrainPlugin;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DqnResource::default())
            .add_event::<DqnEvent>()
            .add_startup_systems((dqn_start_system, dqn_x_start_system))
            .add_systems((
                add_dqn_on_spawned_car_system,
                dqn_rx_to_bevy_event_system,
                dqn_event_reader_system,
                bevy_garage_car::sensor::sensor_system.in_set(CarSet::Input),
                dqn_system.in_set(CarSet::Brain).after(CarSet::Input),
                dqn_dash_update_system,
            ));

        #[cfg(feature = "api")]
        {
            use crate::api_client::*;
            app.add_event::<StreamEvent>()
                .add_startup_system(api_start_system)
                .add_systems((api_read_stream_event_writer_system, api_event_reader_system));
        }
    }
}
