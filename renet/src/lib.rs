use bevy::prelude::*;
use bevy_garage_car::STATIC_GROUP;
use bevy_rapier3d::prelude::*;
use bevy_renet::renet::{transport::NETCODE_KEY_BYTES, ChannelConfig, ConnectionConfig, SendType};
use serde::{Deserialize, Serialize};
use std::{f32::consts::PI, time::Duration};

pub fn rapier_config_start_system(mut c: ResMut<RapierContext>) {
    c.integration_parameters.max_velocity_iterations = 64;
    c.integration_parameters.max_velocity_friction_iterations = 64;
    c.integration_parameters.max_stabilization_iterations = 16;
    c.integration_parameters.erp = 0.99;
}

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug, Component)]
pub struct Player {
    pub id: u64,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Serialize, Deserialize, Component, Event)]
pub enum PlayerCommand {
    BasicAttack { cast_at: Vec3 },
}

pub enum ClientChannel {
    Input,
    Command,
}

pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate {
        entity: Entity,
        id: u64,
        translation: [f32; 3],
    },
    PlayerRemove {
        id: u64,
    },
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkedEntities {
    pub entities: Vec<Entity>,
    pub translations: Vec<[f32; 3]>,
    pub rotations: Vec<[f32; 4]>,
    pub wheels_translations: Vec<[[f32; 3]; 4]>,
    pub wheels_rotations: Vec<[[f32; 4]; 4]>,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Command => 0,
            ClientChannel::Input => 1,
        }
    }
}

impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::Input.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
            ChannelConfig {
                channel_id: Self::Command.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
        ]
    }
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::NetworkedEntities => 0,
            ServerChannel::ServerMessages => 1,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::NetworkedEntities.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
            ChannelConfig {
                channel_id: Self::ServerMessages.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(200),
                },
            },
        ]
    }
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channels_config(),
        server_channels_config: ServerChannel::channels_config(),
    }
}

pub fn setup_level(
    mut cmd: Commands,
    #[cfg(feature = "graphics")] mut meshes: ResMut<Assets<Mesh>>,
    #[cfg(feature = "graphics")] mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 1000.;
    let cuboid = Collider::cuboid(size / 2., 0.5, size / 2.);
    let transform = Transform::from_xyz(0.0, -1.0, 0.0);

    let mut cuboid_cmd = cmd.spawn((
        cuboid,
        RigidBody::Fixed,
        ColliderScale::Absolute(Vec3::ONE),
        CollisionGroups::new(STATIC_GROUP, Group::ALL),
        Friction::coefficient(3.),
        Restitution::coefficient(0.),
    ));
    #[cfg(feature = "graphics")]
    cuboid_cmd.insert(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(size, 1., size))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform,
        ..Default::default()
    });
    #[cfg(not(feature = "graphics"))]
    cuboid_cmd.insert(TransformBundle::from_transform(transform));

    #[cfg(feature = "graphics")]
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}
