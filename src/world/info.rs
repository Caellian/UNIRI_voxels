use crate::data::MatterState;
use crate::MaterialID;
use bevy::prelude::*;
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LayerComposition(Vec<(MaterialID, MatterState, f32)>);

impl<S: AsRef<[(MaterialID, MatterState, f32)]>> From<S> for LayerComposition {
    fn from(composition: S) -> Self {
        LayerComposition(composition.as_ref().to_vec())
    }
}

#[derive(Debug, Component)]
pub struct PlanetInfo {
    pub seed: u32,

    pub radius: u32,
    pub layers: Vec<(LayerComposition, f32)>,
}

impl Default for PlanetInfo {
    fn default() -> Self {
        let mut r = rand::thread_rng();

        let radius = 32000;

        fn layer_count(count: usize) -> f32 {
            count as f32 / 32000.0
        }

        PlanetInfo {
            seed: r.next_u32(),
            radius,
            layers: vec![
                (
                    LayerComposition::from(&[(
                        MaterialID::new("common:lava"),
                        MatterState::Liquid,
                        1.0,
                    )]),
                    0.1,
                ),
                (
                    LayerComposition::from(&[
                        (MaterialID::new("common:gold"), MatterState::Solid, 0.001),
                        (MaterialID::new("common:lava"), MatterState::Liquid, 0.1),
                        (MaterialID::new("common:diamond"), MatterState::Solid, 0.05),
                        (MaterialID::new("common:iron"), MatterState::Solid, 0.15),
                        (MaterialID::new("common:coal"), MatterState::Solid, 0.3),
                        (MaterialID::new("common:stone"), MatterState::Solid, 0.399),
                    ]),
                    0.3,
                ),
                (
                    LayerComposition::from(&[
                        (MaterialID::air(), MatterState::Gaseous, 0.2),
                        (MaterialID::new("common:iron"), MatterState::Solid, 0.2),
                        (MaterialID::new("common:coal"), MatterState::Solid, 0.3),
                        (MaterialID::new("common:stone"), MatterState::Solid, 0.3),
                    ]),
                    0.4,
                ), // 2 rem
                (
                    LayerComposition::from(&[
                        (MaterialID::air(), MatterState::Gaseous, 0.4),
                        (MaterialID::new("common:iron"), MatterState::Solid, 0.1),
                        (MaterialID::new("common:coal"), MatterState::Solid, 0.2),
                        (MaterialID::new("common:stone"), MatterState::Solid, 0.3),
                    ]),
                    0.5,
                ),
                (
                    LayerComposition::from(&[
                        (MaterialID::new("common:stone"), MatterState::Solid, 0.3),
                        (MaterialID::new("common:dirt"), MatterState::Solid, 0.7),
                    ]),
                    0.1,
                ),
                (
                    LayerComposition::from(&[
                        (MaterialID::new("common:stone"), MatterState::Solid, 0.3),
                        (MaterialID::new("common:dirt"), MatterState::Solid, 0.7),
                    ]),
                    layer_count(2),
                ),
            ],
        }
    }
}
