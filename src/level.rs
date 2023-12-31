use bevy::prelude::*;

use crate::{sprite::AnimationIndices, state::GameplayOnly};

pub struct LevelPlugin;
use rand::Rng;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub const MAP_WIDTH: f32 = 2560.;
pub const MAP_HEIGHT: f32 = 2560.;

pub struct Level {
    pub tilemap: String,
    pub height: f32,
    pub width: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let block_width = 32.;
    let block_height = 32.;
    let block_cols = 8;
    let block_rows = 8;

    let total_atlas_blocks = block_cols * block_rows;
    let total_tiles_wide = (MAP_WIDTH / block_width) as i32;
    let total_tiles_high = (MAP_HEIGHT / block_height) as i32;
    let total_tiles = total_tiles_high * total_tiles_wide;

    let mut rng = rand::thread_rng();

    // Start at -1280, -1280 for 2560 size map.
    let starting_point_x = -1. * (MAP_WIDTH / 2.);
    let starting_point_y = -1. * (MAP_HEIGHT / 2.);
    let mut rolling_x = starting_point_x;
    let mut rolling_y = starting_point_y;

    //commands.spawn(Camera2dBundle::default());
    for index in 0..total_tiles {
        let texture_handle = asset_server.load("level/tx_tileset_grass.png");

        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(block_width, block_height),
            block_cols,
            block_rows,
            None,
            None,
        );

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let random_index: usize = rng.gen_range(0..total_atlas_blocks);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(random_index),
                transform: Transform {
                    scale: Vec3::new(1., 1., 1.),
                    rotation: Quat::IDENTITY,
                    translation: Vec3::new(rolling_x, rolling_y, -0.1),
                },
                // global_transform: Transform::
                ..default()
            },
            GameplayOnly,
        ));

        rolling_x += block_width;

        if rolling_x >= MAP_WIDTH / 2. {
            rolling_x = starting_point_x;
            rolling_y += block_height;
        }

        if rolling_y >= MAP_HEIGHT / 2. {
            break;
            //panic!("Should not get here - loop should finish before this.");
        }
    }
}
