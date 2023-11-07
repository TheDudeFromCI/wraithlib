use bevy::prelude::*;
use rusqlite::params;
use wraithlib::common::files::Files;
use wraithlib::common::WraithLibPlugins;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(WraithLibPlugins)
        .add_systems(Update, save_game)
        .run();
}

fn save_game(time: Res<Time>, files: Res<Files>, mut last_save: Local<f32>) {
    let time = time.elapsed_seconds();
    if time - *last_save < 1.0 {
        return;
    }
    *last_save = time;

    let conn = files.get_save("save_counter").open();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS save_count (value INTEGER)",
        params![],
    )
    .unwrap();

    let mut count: i32 = conn
        .query_row(
            "SELECT value FROM save_count ORDER BY value DESC LIMIT 1",
            params![],
            |row| row.get(0),
        )
        .unwrap_or(0);

    count += 1;
    println!("Save Count: {}", count);

    conn.execute(
        "INSERT INTO save_count (value) VALUES (?1)",
        rusqlite::params![count],
    )
    .unwrap();
}
