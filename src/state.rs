use bevy::{prelude::States, ecs::component::Component};

#[derive(States, Default, Debug, PartialEq, Clone, Hash, Eq )]
pub enum GameState {
    #[default]
    StartingLoop,
    Gameplay,
    LevelUp,
    GameOver,   
}

#[derive(Component)]
pub struct GameplayOnly;

pub const PIXEL_TO_WORLD: f32 = 50. / 100.;

