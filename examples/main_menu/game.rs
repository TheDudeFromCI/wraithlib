use bevy::prelude::*;
use bevy_wh_net::client::{DoDisconnectFromServer, OnReceivePacketFromServer};
use bevy_wh_net::common::{packet_to_client, Packet};
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct GameObject;

#[derive(Component)]
pub struct Cube;

#[derive(Debug, Serialize, Deserialize, Packet)]
#[packet_to_client]
pub struct RotatePacket {
    pub rotation: Quat,
}

pub fn init(
    mut query_camera: Query<&mut Transform, With<Camera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        GameObject,
        PbrBundle {
            mesh: meshes.add(shape::Circle::new(4.0).into()),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        },
    ));

    commands.spawn((
        GameObject,
        Cube,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        GameObject,
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
    ));

    *query_camera.single_mut() =
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y);
}

pub fn cleanup(query_game_objects: Query<Entity, With<GameObject>>, mut commands: Commands) {
    for entity in query_game_objects.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update(
    mut query_cubes: Query<&mut Transform, With<Cube>>,
    mut on_packet_evs: EventReader<OnReceivePacketFromServer>,
) {
    for ev in on_packet_evs.read() {
        let Some(packet) = ev.as_packet::<RotatePacket>() else {
            continue;
        };

        for mut transform in query_cubes.iter_mut() {
            transform.rotation = packet.rotation;
        }
    }
}

pub fn logout(
    keyboard_input: Res<Input<KeyCode>>,
    mut do_quit_server: EventWriter<DoDisconnectFromServer>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        do_quit_server.send(DoDisconnectFromServer);
    }
}
