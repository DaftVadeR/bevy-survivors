use bevy::prelude::*;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Component, Debug)]
pub struct Movable {
    pub speed: f32,
    pub direction: Direction,
    pub is_moving: bool,
}

#[derive(Component, Debug)]
pub struct SpriteSheetAnimatable {
    pub idle_anim_indices: AnimationIndices,
    pub moving_anim_indices: AnimationIndices,
}

#[derive(Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Debug)]
pub struct Health(pub f32);
