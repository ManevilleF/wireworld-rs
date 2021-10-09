use bevy::prelude::*;

#[derive(Debug)]
pub struct BoardMaterials {
    pub conductor_material: Handle<ColorMaterial>,
    pub electron_head_material: Handle<ColorMaterial>,
    pub electron_tail_material: Handle<ColorMaterial>,
    pub selector_material: Handle<ColorMaterial>,
    pub generator_material: Handle<ColorMaterial>,
}

impl BoardMaterials {
    pub fn new(assets: &mut Assets<ColorMaterial>) -> Self {
        Self {
            conductor_material: assets.add(Color::GOLD.into()),
            electron_head_material: assets.add(Color::CYAN.into()),
            electron_tail_material: assets.add(Color::WHITE.into()),
            selector_material: assets.add(Color::rgba(1., 1., 1., 0.2).into()),
            generator_material: assets.add(Color::ORANGE_RED.into()),
        }
    }
}
