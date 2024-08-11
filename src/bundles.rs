use bevy::{
    pbr::{MaterialMeshBundle, StandardMaterial},
    prelude::Bundle,
};

#[derive(Bundle)]
pub struct BuildingBundle {
    pub(crate) mesh: MaterialMeshBundle<StandardMaterial>,
}
