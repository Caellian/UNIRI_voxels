use crate::error::ResourceError;
use crate::world::material::Side;
use crate::MaterialID;
use ahash::HashMap;
use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use serde::Deserialize;
use std::collections::btree_map::BTreeMap;
use std::hash::Hash;
use std::mem::size_of;
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

#[derive(Resource)]
pub struct LoadedContentPacks(pub Vec<ContentPack>);

#[derive(Debug, Clone, PartialEq, ShaderType, Deserialize)]
#[serde(default)]
#[repr(C)]
pub struct FaceProperties {
    #[serde(deserialize_with = "crate::color::deserialize_hex_color")]
    pub base_color: Vec4,

    #[serde(skip)]
    pub uv: Vec2,

    #[serde(deserialize_with = "crate::color::deserialize_hex_color")]
    pub emissive_color: Vec4,

    pub roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
}

impl Hash for FaceProperties {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let byte_slice = unsafe {
            // FIXME: Reinterpreting FaceProperties as bytes to hash floats
            let source: *const FaceProperties = self;
            std::slice::from_raw_parts::<u8>(source as *const u8, size_of::<FaceProperties>())
        };

        byte_slice.hash(state)
    }
}
impl Eq for FaceProperties {}

impl Default for FaceProperties {
    fn default() -> Self {
        FaceProperties {
            base_color: Vec4::new(1., 1., 1., 1.),
            emissive_color: Vec4::new(0., 0., 0., 1.),

            roughness: 0.5,
            metallic: 0.5,
            reflectance: 0.5,

            uv: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BlockFaces {
    Uniform {
        face: FaceProperties,
    },
    Sided {
        face: FaceProperties,
        face_override: HashMap<Side, FaceProperties>,
    },
}

impl Default for BlockFaces {
    fn default() -> Self {
        BlockFaces::Uniform {
            face: FaceProperties::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct MaterialProperties {
    #[serde(deserialize_with = "crate::color::deserialize_hex_color")]
    pub color: Vec4,

    #[serde(flatten)]
    pub faces: Option<BlockFaces>,
}

impl Default for MaterialProperties {
    fn default() -> Self {
        MaterialProperties {
            color: Vec4::new(1., 1., 1., 1.),
            faces: None,
        }
    }
}

#[derive(Resource)]
pub struct LoadedMaterials {
    pub properties: BTreeMap<MaterialID, MaterialProperties>,
    /// MaterialID -> (texture_location, [UVs; 6]);
    pub texture_location: BTreeMap<MaterialID, (u16, [Rect; 6])>,
    //pub face_positions: BTreeMap<MaterialID, [u32; 6]>,
}

impl LoadedMaterials {
    pub fn insert_material(&mut self, id: MaterialID, mut props: MaterialProperties) {
        if props.faces.is_none() {
            props.faces = Some(BlockFaces::Uniform {
                face: FaceProperties {
                    base_color: props.color,
                    ..default()
                },
            });
        }

        self.properties.insert(id, props);
        //self.face_positions.insert(id, face_positions);
    }
}

pub fn load_content(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let mut loaded = LoadedMaterials {
        properties: BTreeMap::new(),
        texture_location: BTreeMap::new(),
        //buffer: StorageBuffer::default(),
        //face_positions: BTreeMap::new(),
    };

    let packs = content_packs();
    match packs.len() {
        0 => tracing::warn!("No content packs found."),
        it => tracing::info!("Loading {} content pack(s)...", it),
    }

    fn process_material(loaded: &mut LoadedMaterials, pack: &ContentPack, material_path: &Path) {
        let prop_file = material_path.join("properties.ron");

        let name = material_path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("can't get material directory name");
        let id = pack.id.clone() + ":" + name;

        // TODO: Texture packing

        if let Ok(prop_str) = std::fs::read_to_string(prop_file) {
            match ron::from_str::<MaterialProperties>(&prop_str) {
                Ok(props) => {
                    tracing::info!("- Material: '{}'", &id);
                    loaded.insert_material(MaterialID::new(id), props);
                }
                Err(err) => {
                    tracing::error!("Unable to read '{}' properties file: {:#?}", &id, err);
                }
            }
        } else {
            tracing::error!("Unable to read '{}' properties file.", &id);
        }
    }

    fn process_pack(loaded: &mut LoadedMaterials, pack: &ContentPack) {
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
                process_material(loaded, pack, &material_path);
            }
        }
    }

    for pack in &packs {
        process_pack(&mut loaded, pack);
    }

    commands.insert_resource(LoadedContentPacks(packs));
    commands.insert_resource(loaded);
}
