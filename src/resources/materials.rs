use bevy::{
    color::palettes::css::{AQUA, GOLD, ORANGE_RED, WHITE},
    prelude::*,
};

#[derive(Debug, Resource)]
pub struct BoardMaterials {
    pub conductor_material: Color,
    pub electron_head_material: Color,
    pub electron_tail_material: Color,
    pub selector_material: Color,
    pub generator_material: Color,
}

impl BoardMaterials {
    #[inline]
    pub const fn new() -> Self {
        Self {
            conductor_material: Color::Srgba(GOLD),
            electron_head_material: Color::Srgba(AQUA),
            electron_tail_material: Color::Srgba(WHITE),
            selector_material: Color::srgba(1., 1., 1., 0.2),
            generator_material: Color::Srgba(ORANGE_RED),
        }
    }
}
