use crate::{convert::Convert, world::chunk::SizedGridMut};
use crate::world::chunk::ChunkStore;
use crate::{error::ResourceError, world::chunk::SizedGrid as _};
use ahash::{HashMap, HashMapExt};
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::*;
use dot_vox::DotVoxData;

use super::material::Voxel;

pub struct VoxLoader;

impl AssetLoader for VoxLoader {
    type Asset = Vox;
    type Settings = ();
    type Error = ResourceError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        let name = load_context
            .path()
            .file_name()
            .expect("no file name")
            .to_str()
            .expect("bad file name")
            .to_string();
        Box::pin(async move { load_vox(reader, load_context, name).await })
    }

    fn extensions(&self) -> &[&str] {
        &["vox"]
    }
}

async fn load_vox<'a, 'b>(
    bytes: &'a mut bevy::asset::io::Reader<'a>,
    _load_context: &'a mut LoadContext<'b>,
    name: impl AsRef<str>,
) -> Result<Vox, ResourceError> {
    let _name = name.as_ref().replace(' ', "_").to_lowercase();

    let mut buffer: Vec<u8> = Vec::new();
    bytes.read_to_end(&mut buffer).await?;
    let data: DotVoxData = match dot_vox::load_bytes(&buffer) {
        Ok(d) => d,
        Err(e) => {
            return Err(ResourceError::Vox(e));
        }
    };

    let mut models = Vec::with_capacity(data.models.len());

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

    for (_i, model) in data.models.iter().enumerate() {
        // let id = BlockID(format!("runtime:{}_model_{}", name, i));

        let mut blocks = ChunkStore::new(UVec3::new(model.size.x, model.size.y, model.size.z));
        for vox in model.voxels.iter() {
            // let material_id = BlockID(format!("{}_material_{}", &id.0, vox.i));

            blocks.set_or_clone_pos_value(
                UVec3::new(vox.x as u32, vox.y as u32, vox.z as u32),
                Some(&Voxel::Color(colors[&(vox.i as usize)])),
            );
        }

        models.push(blocks);

        // let model_name = format!("{}_{}", name, i);
        // tracing::info!("Loaded vox model: {}", &model_name);
        //load_context.add_labeled_asset(model_name, blocks);
    }

    Ok(Vox { models })
}

#[derive(Debug, Clone, TypePath, Asset)]
pub struct Vox {
    pub models: Vec<ChunkStore<Voxel>>,
}
