mod enemies;
mod level;
mod player;
mod sprite;
mod state;
mod ui;

use enemies::EnemySpawnerPlugin;
// pub use crate::player::CharacterLife;
use level::LevelPlugin;
use player::PlayerPlugin;
use ui::GameUiPlugin;

use bevy::prelude::*;
use state::{GameState, GameplayOnly};

const WIDTH: f32 = 1600.0;
const HEIGHT: f32 = 900.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Survivors".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: true,

                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(GameUiPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemySpawnerPlugin)
        .add_systems(OnEnter(GameState::GameOver), despawn_game_play)
        .run();
}

fn despawn_game_play(mut commands: Commands, entities: Query<Entity, With<GameplayOnly>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 0.5,
            ..default()
        },
        ..default()
    });
    // commands.spawn()gcc=
}
