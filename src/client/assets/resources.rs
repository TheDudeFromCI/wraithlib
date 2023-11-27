use bevy::asset::{AssetPath, RecursiveDependencyLoadState, UntypedAssetId};
use bevy::prelude::*;

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

    pub fn add_many_to_queue(&mut self, assets: &[UntypedHandle]) {
        for asset in assets {
            self.assets.push(asset.clone().id());
        }
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

    pub fn load_string<A: Asset>(&mut self, path: String) -> Handle<A> {
        let handle = self.asset_server.load(path);
        self.asset_queue.add_to_queue(&handle);
        handle
    }
}
