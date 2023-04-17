use crate::error::ResourceError;
use crate::ext::Convert;
use crate::world::chunk::ChunkStore;
use ahash::{HashMap, HashMapExt};
use anyhow::Error;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use dot_vox::DotVoxData;

use super::material::Voxel;

pub struct VoxLoader;

impl AssetLoader for VoxLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        let name = load_context
            .path()
            .file_name()
            .expect("no file name")
            .to_str()
            .expect("bad file name")
            .to_string();
        Box::pin(async move { Ok(load_vox(bytes, load_context, name).await?) })
    }

    fn extensions(&self) -> &[&str] {
        &["vox"]
    }
}

async fn load_vox<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
    name: impl AsRef<str>,
) -> Result<(), ResourceError> {
    let _name = name.as_ref().replace(' ', "_").to_lowercase();

    let data: DotVoxData = match dot_vox::load_bytes(bytes) {
        Ok(d) => d,
        Err(e) => {
            return Err(ResourceError::Vox(e));
        }
    };

    let mut color_use: Vec<usize> = Vec::new();

    for model in data.models.iter() {
        for vox in model.voxels.iter() {
            let index = vox.i as usize;
            if !color_use.contains(&index) {
                color_use.push(index);
            }
        }
    }

    let colors: HashMap<usize, Color> = {
        let mut m = HashMap::with_capacity(16);
        for (index, color) in data.palette.into_iter().enumerate() {
            if color_use.contains(&index) {
                m.insert(index, color.convert());
            }
        }
        m
    };
    tracing::info!("{:?}", colors);

    for (i, model) in data.models.iter().enumerate() {
        // let id = BlockID(format!("runtime:{}_model_{}", name, i));

        let mut blocks = ChunkStore::new(UVec3::new(model.size.x, model.size.y, model.size.z));
        for vox in model.voxels.iter() {
            // let material_id = BlockID(format!("{}_material_{}", &id.0, vox.i));

            blocks.set_or_clone_value(
                UVec3::new(vox.x as u32, vox.y as u32, vox.z as u32),
                Some(&Voxel::Color(colors[&(vox.i as usize)].clone())),
            );
        }
        let model = VoxelData::new(blocks);

        let model_name = "model_".to_owned() + &i.to_string();
        tracing::info!("Loaded vox model: {}", &model_name);
        load_context.set_default_asset(LoadedAsset::new(model));
    }

    Ok(())
}

#[derive(Debug, Clone, TypeUuid, Bundle)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct VoxelData {
    pub value: ChunkStore<Voxel>,
}

impl VoxelData {
    pub fn new(blocks: ChunkStore<Voxel>) -> Self {
        VoxelData { value: blocks }
    }
}
