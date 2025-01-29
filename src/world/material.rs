use std::fmt::Debug;
use std::string::ToString;

use bevy::prelude::*;

use crate::decl_id_type;

decl_id_type!(MaterialID);

const AIR_ID: MaterialID = MaterialID::Static("air");

impl MaterialID {
    #[must_use]
    pub const fn air() -> MaterialID {
        AIR_ID
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Block {
    /// Air block
    #[default]
    None,
    /// Blocks of registered materials
    MaterialID(MaterialID),
    /// Blocks that are combined out of multiple materials
    Multiblock(),
}

impl Block {
    pub fn material_id(&self) -> Option<MaterialID> {
        Some(match self {
            Block::None => AIR_ID,
            Block::MaterialID(material) => material.clone(),
            _ => return None,
        })
    }
}

/*
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum MatterState {
    Plasma,
    Gaseous,
    Liquid,
    Solid,
}
*/
