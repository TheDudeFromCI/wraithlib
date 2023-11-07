use bevy::prelude::*;

use super::Files;

pub(super) fn init_data_folder(files: Res<Files>) {
    files.init_data_folder();
}
