use std::path::PathBuf;
use std::{env, fs};

use bevy::prelude::*;
use rusqlite::Connection;

#[derive(Debug, Resource)]
pub struct Files {
    pub data_folder: String,
    pub file_ext: String,
}

impl Default for Files {
    fn default() -> Self {
        Self {
            data_folder: "saves".into(),
            file_ext: "whlib".into(),
        }
    }
}

impl Files {
    pub(super) fn init_data_folder(&self) {
        let folder = self.get_data_folder();
        if !folder.exists() {
            fs::create_dir_all(folder).unwrap();
        }
    }

    pub(super) fn get_data_folder(&self) -> PathBuf {
        let dir = env::current_dir().unwrap();
        dir.join(&self.data_folder)
    }

    pub fn get_all_saves(&self) -> Vec<SaveFile> {
        let folder = self.get_data_folder();
        let Ok(dir) = fs::read_dir(folder) else {
            return vec![];
        };

        dir.flatten()
            .flat_map(|entry| {
                let path = entry.path();
                if Some(Some(self.file_ext.as_str())) == path.extension().map(|ext| ext.to_str()) {
                    Some(SaveFile::from_path(path))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn get_save(&self, name: &str) -> SaveFile {
        let folder = self.get_data_folder();
        let name = format!("{}.{}", name, &self.file_ext);
        let file = folder.join(name);
        SaveFile::from_path(file)
    }
}

pub struct SaveFile {
    path: PathBuf,
}

impl SaveFile {
    pub(super) fn from_path(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn open(self) -> Connection {
        Connection::open(self.path).unwrap()
    }
}
