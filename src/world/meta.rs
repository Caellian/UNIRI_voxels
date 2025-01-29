//! World metadata

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Structure {
    pub bounds: Aabb3d,
}
