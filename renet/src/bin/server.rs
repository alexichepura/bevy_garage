use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResolution,
};
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_garage::esp::esp_system;
use bevy_garage_car::{car_start_system, spawn_car, Car, CarRes, CarWheels, Wheel};
use bevy_garage_renet::{
    connection_config, rapier_config_start_system, setup_level, ClientChannel, NetworkedEntities,
    Player, PlayerCommand, PlayerInput, ServerChannel, ServerMessages, PROTOCOL_ID,
};
use bevy_rapier3d::prelude::*;
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use renet_visualizer::RenetServerVisualizer;
use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<u64, Entity>,
}

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(connection_config());

    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time: std::time::Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    (server, transport)
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Garage renet server".to_string(),
                resolution: WindowResolution::new(640., 240.),
                canvas: Some("#bevy-garage".to_string()),
                ..default()
            }),
            ..default()
        }),
        RenetServerPlugin,
        NetcodeServerPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin::default(),
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin::default(),
        EguiPlugin,
    ));

    app.insert_resource(CarRes {
        show_rays: true,
        ..default()
    })
    .insert_resource(RapierConfiguration {
        timestep_mode: TimestepMode::Variable {
            max_dt: 1. / 60.,
            time_scale: 1.,
            substeps: 10,
        },
        ..default()
    });
    app.insert_resource(ServerLobby::default());

    let (server, transport) = new_renet_server();
    app.insert_resource(server).insert_resource(transport);

    app.insert_resource(RenetServerVisualizer::<200>::default());

    app.add_systems(
        Update,
        (
            server_update_system,
            server_network_sync,
            move_players_system,
            update_visulizer_system,
            esp_system.after(move_players_system),
        ),
    );

    app.add_systems(
        Startup,
        (
            rapier_config_start_system,
            setup_level,
            setup_simple_camera,
            car_start_system,
        ),
    );

    app.run();
}

#[allow(clippy::too_many_arguments)]
fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut cmd: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
    players: Query<(Entity, &Player, &Transform)>,
    car_res: Res<CarRes>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);
                visualizer.add_client(*client_id);

                // Initialize other players for this new client
                for (entity, player, transform) in players.iter() {
                    let translation: [f32; 3] = transform.translation.into();
                    let message = bincode::serialize(&ServerMessages::PlayerCreate {
                        id: player.id,
                        entity,
                        translation,
                    })
                    .unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessages, message);
                }

                // Spawn new player
                let transform = Transform::from_xyz(
                    (fastrand::f32() - 0.5) * 40.,
                    0.51,
                    (fastrand::f32() - 0.5) * 40.,
                );
                let player_entity = spawn_car(
                    &mut cmd,
                    &car_res.car_scene.as_ref().unwrap(),
                    &car_res.wheel_scene.as_ref().unwrap(),
                    false,
                    transform,
                );
                cmd.entity(player_entity)
                    .insert(Player { id: *client_id })
                    .insert(PlayerInput::default());

                lobby.players.insert(*client_id, player_entity);

                let translation: [f32; 3] = transform.translation.into();
                let message = bincode::serialize(&ServerMessages::PlayerCreate {
                    id: *client_id,
                    entity: player_entity,
                    translation,
                })
                .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
                visualizer.remove_client(*client_id);
                if let Some(player_entity) = lobby.players.remove(client_id) {
                    cmd.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessages::PlayerRemove { id: *client_id }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }
    }

    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
            let command: PlayerCommand = bincode::deserialize(&message).unwrap();
            match command {
                PlayerCommand::BasicAttack { cast_at } => {
                    println!(
                        "Received basic attack from client {}: {:?}",
                        client_id, cast_at
                    );
                }
            }
        }
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            let input: PlayerInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                cmd.entity(*player_entity).insert(input);
            }
        }
    }
}

fn update_visulizer_system(
    mut egui_contexts: EguiContexts,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
    server: Res<RenetServer>,
) {
    visualizer.update(&server);
    visualizer.show_window(egui_contexts.ctx_mut());
}

#[allow(clippy::type_complexity)]
fn server_network_sync(
    mut server: ResMut<RenetServer>,
    mut tr_set: ParamSet<(
        Query<(Entity, &Transform, &CarWheels), With<Player>>,
        Query<&Transform, With<Wheel>>,
    )>,
) {
    let mut networked_entities = NetworkedEntities::default();
    let mut wheels_all: Vec<[Entity; 4]> = vec![];
    for (entity, transform, wheels) in tr_set.p0().iter() {
        networked_entities.entities.push(entity);
        networked_entities
            .translations
            .push(transform.translation.into());
        networked_entities.rotations.push(transform.rotation.into());

        wheels_all.push(wheels.entities);
    }

    for wheels in wheels_all {
        networked_entities.wheels_translations.push([
            tr_set.p1().get(wheels[0]).unwrap().translation.into(),
            tr_set.p1().get(wheels[1]).unwrap().translation.into(),
            tr_set.p1().get(wheels[2]).unwrap().translation.into(),
            tr_set.p1().get(wheels[3]).unwrap().translation.into(),
        ]);
        networked_entities.wheels_rotations.push([
            tr_set.p1().get(wheels[0]).unwrap().rotation.into(),
            tr_set.p1().get(wheels[1]).unwrap().rotation.into(),
            tr_set.p1().get(wheels[2]).unwrap().rotation.into(),
            tr_set.p1().get(wheels[3]).unwrap().rotation.into(),
        ]);
    }

    let sync_message = bincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
}

fn move_players_system(mut query: Query<(&PlayerInput, &mut Car)>) {
    for (input, mut car) in query.iter_mut() {
        if input.up {
            car.gas = 1.;
        } else {
            car.gas = 0.;
        }
        if input.down {
            car.brake = 1.;
        } else {
            car.brake = 0.;
        }
        if input.left {
            car.steering = -1.;
        }
        if input.right {
            car.steering = 1.;
        }
        if !input.left && !input.right {
            car.steering = 0.;
        }
    }
}

pub fn setup_simple_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-20.5, 30.0, 20.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
