use crate::error::ResourceError;
use crate::world::block::Side;
use crate::{BlockProperties, MaterialID, NAME};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::btree_map::BTreeMap;
use std::ops::Deref;
use std::path::{Path, PathBuf};

static CONTENT_DIR: &str = "content";

pub fn content_dir() -> PathBuf {
    let path = {
        #[cfg(not(feature = "dev"))]
        {
            dirs::data_dir()
                .map(|data| data.join(NAME).join(CONTENT_DIR))
                .unwrap_or(PathBuf::new().join(CONTENT_DIR))
        }
        #[cfg(feature = "dev")]
        {
            PathBuf::new().join(CONTENT_DIR)
        }
    };

    if !path.exists() {
        std::fs::create_dir_all(&path).expect("unable to create content directory");
    }

    path
}

#[derive(Debug, Deserialize)]
pub struct ContentPack {
    #[serde(skip)]
    pub path: PathBuf,

    pub id: String,
    pub name: String,
    pub authors: Vec<String>,
}

impl ContentPack {
    pub fn init(path: impl AsRef<Path>) -> Result<ContentPack, ResourceError> {
        let info_path = path.as_ref().join("info.ron");

        if !info_path.exists() {
            return Err(ResourceError::InvalidPath(info_path));
        }

        let info_str = std::fs::read_to_string(info_path)?;

        let mut result: ContentPack = ron::from_str(&info_str)?;
        result.path = path.as_ref().to_path_buf();

        Ok(result)
    }
}

pub fn content_packs() -> Vec<ContentPack> {
    let pack_dirs: Vec<PathBuf> = std::fs::read_dir(content_dir())
        .expect("unable to access content directory")
        .filter_map(|it| it.ok().map(|e| e.path()))
        .collect();

    let mut result: Vec<ContentPack> = Vec::new();
    for dir in pack_dirs {
        match ContentPack::init(dir) {
            Ok(it) => {
                tracing::info!("- Found: {}", it.name);
                result.push(it);
            }
            Err(err) => {
                tracing::error!("Can't load content pack: {:?}", err);
            }
        }
    }

    result
}

impl Deref for LoadedBlocks {
    type Target = BTreeMap<MaterialID, BlockProperties>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Component)]
pub enum BlockTextureHandles {
    Uniform(HandleUntyped),
    Sided([HandleUntyped; 6]),
}

impl BlockTextureHandles {
    pub fn get_side(&self, s: Side) -> &HandleUntyped {
        match self {
            BlockTextureHandles::Uniform(it) => it,
            BlockTextureHandles::Sided(values) => &values[s.index()],
        }
    }

    pub fn set_side(&mut self, s: Side, value: HandleUntyped) {
        match self {
            BlockTextureHandles::Uniform(it) => {
                *it = value;
            }
            BlockTextureHandles::Sided(values) => {
                values[s.index()] = value;
            }
        }
    }
}

#[derive(Resource)]
pub struct LoadedContentPacks(Vec<ContentPack>);

#[derive(Resource)]
pub struct LoadedBlocks(BTreeMap<MaterialID, BlockProperties>);

#[derive(Resource)]
pub struct LoadedTextures(BTreeMap<MaterialID, BlockTextureHandles>);

pub fn load_content(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let mut block_props = BTreeMap::new();

    let packs = content_packs();
    match packs.len() {
        0 => tracing::warn!("No content packs found."),
        it => tracing::info!("Loading {} content pack(s)...", it),
    }

    for pack in &packs {
        let materials_path = pack.path.as_path().join("materials");
        if let Ok(materials_dir) = std::fs::read_dir(materials_path) {
            let material_metas: Vec<std::fs::DirEntry> = materials_dir
                .filter_map(|e| e.ok())
                .filter(|d| d.metadata().unwrap().is_dir())
                .collect();

            tracing::info!(
                "Loading {} materials from '{}' ...",
                material_metas.len(),
                pack.name
            );

            for material_meta in material_metas {
                let material_path = material_meta.path();
                let prop_file = material_path.join("properties.ron");

                let name = material_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .expect("can't get material directory name");
                let id = pack.id.clone() + ":" + name;

                if let Ok(prop_str) = std::fs::read_to_string(prop_file) {
                    match ron::from_str::<BlockProperties>(&prop_str) {
                        Ok(props) => {
                            tracing::info!("- Material: '{}'", &id);
                            block_props.insert(MaterialID::new(id), props);
                        }
                        Err(err) => {
                            tracing::error!("Unable to read '{}' properties file: {:?}", &id, err)
                        }
                    }
                } else {
                    tracing::error!("Unable to read '{}' properties file.", &id)
                }
            }
        }
    }

    commands.insert_resource(LoadedContentPacks(packs));
    commands.insert_resource(LoadedBlocks(block_props));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum MatterState {
    Plasma,
    Gaseous,
    Liquid,
    Solid,
}
