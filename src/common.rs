use bevy::prelude::*;

pub fn mouse_coords(window: &Window, position: Vec2) -> Vec2 {
    let window_size = Vec2::new(window.width(), window.height());
    position - window_size / 2.
}

pub fn mouse_coords_to_cell(world_pos: Vec2, cell_size: i32) -> IVec2 {
    let mouse_position = world_pos / cell_size as f32;
    IVec2::new(
        mouse_position.x.round() as i32,
        mouse_position.y.round() as i32,
    )
}
