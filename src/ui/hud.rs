use bevy::prelude::*;

use crate::decl_id_type;

decl_id_type!(ScreenID);

#[derive(Debug, Clone, Component)]
pub struct HUDScreen {
    id: ScreenID
}

impl PartialEq for HUDScreen {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
