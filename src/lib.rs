use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
use serde::Deserialize;

pub mod register_bevy_math;

pub use rhai::*;


#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "c65283c9-420f-49b4-a99a-56d054160294"]
pub struct RhaiScript {
    pub content: String,
    pub file_name: String,
}

#[derive(Default)]
pub struct RhaiScriptLoader;
impl AssetLoader for RhaiScriptLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = RhaiScript {
                content: std::str::from_utf8(bytes)?.to_string(),
                file_name: load_context
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            };
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["rhai"]
    }
}

pub struct BevyRhaiPlugin;

impl Plugin for BevyRhaiPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<RhaiScript>()
            .init_asset_loader::<RhaiScriptLoader>();
    }
}
