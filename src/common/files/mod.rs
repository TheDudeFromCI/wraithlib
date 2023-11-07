use bevy::prelude::*;

mod file_system;
mod systems;

pub use file_system::*;
pub use include_sqlite_sql::*;
pub use rusqlite::Connection;

pub struct FilesPlugin;
impl Plugin for FilesPlugin {
    fn build(&self, app_: &mut App) {
        app_.init_resource::<Files>()
            .add_systems(Startup, systems::init_data_folder);
    }
}
