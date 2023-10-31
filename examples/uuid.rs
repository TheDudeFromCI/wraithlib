use bevy::prelude::*;
use wraithlib::common::uuid::{EntityPointer, Uuid, UuidQuery};
use wraithlib::common::WraithLibPlugins;

pub const UUID_POINTER: &str = "0f9a0026-6700-5521-82c3-cf9921184e5c";

/// Showcase use of the Uuid plugin.
fn main() {
    App::new()
        .init_resource::<EntityPointerCache>()
        .add_plugins(DefaultPlugins)
        .add_plugins(WraithLibPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, cache.before(move_entity))
        .add_systems(Update, move_entity)
        .run();
}

#[derive(Debug, Default, Resource)]
struct EntityPointerCache {
    pointer: Option<EntityPointer>,
}

fn init(mut commands: Commands) {
    commands.spawn((
        Uuid::from_string(UUID_POINTER),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}

fn cache(mut cache: ResMut<EntityPointerCache>, query: UuidQuery<()>) {
    if cache.pointer.is_some() {
        return;
    }

    let pointer = query.get_pointer(&Uuid::from_string(UUID_POINTER)).unwrap();
    cache.pointer = Some(pointer);
}

fn move_entity(
    time: Res<Time>,
    cache: Res<EntityPointerCache>,
    mut query: UuidQuery<&mut Transform>,
) {
    // We can create an entity pointer using either a Uuid or an Entity. (Or both)
    //
    // A UuidQuery can any entity pointer as input, allowing for access if we only
    // have one or the other. This is useful for networking, where Uuids are shared,
    // but entities are not.
    //
    // Note: Looking up an entity using a Uuid alone is slow. It is always
    // recommended to cache the entity id within the pointer where possible.
    // This can be done by using the get_pointer() method on a UuidQuery.

    if let Some(pointer) = &cache.pointer {
        if let Ok(mut transform) = query.get_mut(pointer) {
            transform.translation.x += time.delta_seconds();
        }
    }
}
