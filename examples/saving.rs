use bevy::prelude::*;
use wraithlib::common::files::*;
use wraithlib::common::WraithLibPlugins;

include_sql!("examples/sql/save_counter.sql");

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
    update_counter(&conn).unwrap();
}

fn update_counter(save: &Connection) -> rusqlite::Result<()> {
    save.init_save_counter()?;

    let mut count = save.get_save_count(|row| {
        let r: i32 = row.get(0)?;
        Ok(r)
    })?;

    count += 1;
    println!("Save Count: {}", count);

    save.set_save_count(count)?;

    Ok(())
}
