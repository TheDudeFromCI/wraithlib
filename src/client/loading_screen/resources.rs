use bevy::asset::{AssetPath, RecursiveDependencyLoadState, UntypedAssetId};
use bevy::prelude::*;

use crate::client::gamestates::ClientGameState;

#[derive(Debug, Resource, Clone)]
pub struct LoadingScreenProperties {
    pub path: Option<String>,
    pub fade_in_time: f32,
    pub fade_out_time: f32,
}

impl Default for LoadingScreenProperties {
    fn default() -> Self {
        Self {
            path: None,
            fade_in_time: 0.5,
            fade_out_time: 0.5,
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct ActiveLoadingScreen {
    pub state: ClientGameState,
    pub loading_img: Option<UiImage>,
    pub fade_time_end: f32,
}

#[derive(Debug, Default, States, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoadingState {
    #[default]
    None,
    StartingLoad,
    Loading,
    FinishingLoad,
}

#[derive(Debug, Default, Resource)]
pub struct AssetsWaitForLoad {
    assets: Vec<UntypedAssetId>,
}

impl AssetsWaitForLoad {
    pub fn new() -> Self {
        Self { assets: Vec::new() }
    }

    pub fn load<'a, A: Asset>(
        &mut self,
        asset_server: &AssetServer,
        path: impl Into<AssetPath<'a>>,
    ) -> Handle<A> {
        let handle = asset_server.load(path);
        self.add_to_queue(&handle);
        handle
    }

    pub fn add_to_queue<A: Asset>(&mut self, asset: &Handle<A>) {
        self.assets.push(asset.clone().untyped().id());
    }

    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }

    pub fn remove_finished(&mut self, asset_server: &AssetServer) {
        self.assets.retain(|asset| {
            let Some(state) = asset_server.get_recursive_dependency_load_state(*asset) else {
                return false;
            };

            state == RecursiveDependencyLoadState::NotLoaded
                || state == RecursiveDependencyLoadState::Loading
        });
    }

    pub fn with_server<'a>(&'a mut self, asset_server: &'a AssetServer) -> AssetLoader<'a> {
        AssetLoader {
            asset_server,
            asset_queue: self,
        }
    }
}

pub struct AssetLoader<'a> {
    asset_server: &'a AssetServer,
    asset_queue: &'a mut AssetsWaitForLoad,
}

impl<'a> AssetLoader<'a> {
    pub fn load<A: Asset>(&mut self, path: impl Into<AssetPath<'a>>) -> Handle<A> {
        let handle = self.asset_server.load(path);
        self.asset_queue.add_to_queue(&handle);
        handle
    }
}
