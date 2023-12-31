use crate::level::{MAP_HEIGHT, MAP_WIDTH};
use crate::player::{CanLevel, Player};
use crate::sprite::{
    AnimationIndices, AnimationTimer, Direction, Health, Movable, SpriteSheetAnimatable,
};
use crate::state::{GameState, GameplayOnly, PIXEL_TO_WORLD};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::prelude::*;

const SPAWN_DISTANCE: f32 = 1000.;

pub struct EnemySpawnerPlugin;

// #[derive(Bundle)]
// struct EnemyBundle {
//     movable: Movable,
//     sprite_animate: SpriteSheetAnimatable,
//     health: Health,
//     ,
//
//     // We can nest/include another bundle.
//     // Add the components for a standard Bevy Sprite:
//     sprite: SpriteSheetBundle,
// }

#[derive(Debug, Component)]
pub struct Harmful {
    damage: f32,
}

#[derive(Debug, Clone, Component)]
struct Goblin;

#[derive(Debug, Clone, Component)]
struct Mushroom;

#[derive(Debug, Clone, Component)]
struct Slime;

pub enum EnemyType {
    Goblin,
    Mushroom,
    Slime,
}

pub trait SpawnedEnemy {
    fn new(
        self,
        atlases: &mut ResMut<Assets<TextureAtlas>>,
        image: Handle<Image>,
        commands: &mut Commands,
        current_stage: usize,
        player_pos: Vec2,
    ) -> ();
    fn get_sprite_location(&self) -> String;
}

#[derive(Resource)]
pub struct LevelSpawns {
    pub spawns: Vec<SpawnStage>,
    pub global_timer: Stopwatch,
    pub wave_timer: Timer,
    pub stage_timer: Timer,
    pub current_stage: usize,
}

#[derive()]
pub struct SpawnStage {
    pub mobs: Vec<Box<SpawnMob>>,
}

pub struct SpawnMob {
    pub enemy: EnemyType,
    pub count: u32,
}

pub struct SpawnWave;

impl SpawnedEnemy for Goblin {
    fn get_sprite_location(&self) -> String {
        "enemy/goblin/goblin_spritesheet.png".into()
    }

    fn new(
        self,
        mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        texture_handle: Handle<Image>,
        mut commands: &mut Commands,
        current_stage: usize,
        player_pos: Vec2,
    ) {
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 6, 1, None, None);

        // texture_atlas.
        // let texture_atlas_run =
        //     TextureAtlas::from_grid(texture_handle_run, Vec2::new(16.0, 16.0), 6, 1, None, None);

        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        // let texture_atlas_run_handle = texture_atlases.add(texture_atlas_run);

        // Use only the subset of sprites in the sheet that make up the run animation
        let idle_animation_indices = AnimationIndices { first: 0, last: 1 };
        let run_animation_indices = AnimationIndices { first: 0, last: 5 };

        let mut rng = rand::thread_rng();
        let rnd_x: f32 = rng.gen_range(0. ..SPAWN_DISTANCE);
        let rnd_y: f32 = SPAWN_DISTANCE - rnd_x;
        let switch: bool = rng.gen_bool(0.5);

        let x_pos = player_pos.x + if switch { rnd_x * -1. } else { rnd_x };
        let y_pos = player_pos.y + if switch { rnd_y * -1. } else { rnd_y };

        let final_x_pos = x_pos.clamp(-1. * MAP_WIDTH, MAP_WIDTH);
        let final_y_pos = y_pos.clamp(-1. * MAP_HEIGHT, MAP_HEIGHT);

        commands.spawn((
            self,
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(idle_animation_indices.first),
                transform: Transform::from_translation(Vec3::new(final_x_pos, final_y_pos, 1.)),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            GameplayOnly,
            SpriteSheetAnimatable {
                idle_anim_indices: idle_animation_indices,
                moving_anim_indices: run_animation_indices,
            },
            Movable {
                speed: 100.,
                direction: Direction::Right,
                is_moving: false,
            },
            Health(10.),
            Harmful { damage: 10. },
        ));
    }
}

/*impl Enemy for Mushroom {
    fn new() -> Self {
        Goblin {
            damage: 20,
            spritesheet: "assets/goblin/goblin_spritesheet.png",
        }
    }
}

impl Enemy for Slime {
    fn new() -> Self {
        Goblin {
            damage: 15,
            spritesheet: "assets/goblin/goblin_spritesheet.png",
        }
    }
}*/

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.insert_resource(get_first_level_spawns());
        app.add_systems(Update, check_for_spawns);

        // app.add_systems(Update, animate_sprite);
        // app.add_systems(Update, player_movement);
        // app.add_systems(Update, update_camera_from_player_position);

        // app.add_systems(Startup, /*OnEnter(GameState::StartingLoop),*/ spawn_player);
        /*.add_systems(
            (
                player_movement,
                player_exp_start_pickup,
                player_gain_exp,
                player_level_up,
                player_game_over,
            )
            .in_set(OnUpdate(GameState::Gameplay)),
        );*/
        // // simple "facilitator" schedules benefit from simpler single threaded scheduling
        // let mut main_schedule = Schedule::new(Main);
        // main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        // let mut fixed_update_loop_schedule = Schedule::new(RunFixedUpdateLoop);
        // fixed_update_loop_schedule.set_executor_kind(ExecutorKind::SingleThreaded);

        // app.add_schedule(main_schedule)
        //     .add_schedule(fixed_update_loop_schedule)
        //     .init_resource::<MainScheduleOrder>()
        //     .add_systems(Main, Main::run_main);
    }
}

fn get_first_level_spawns() -> LevelSpawns {
    LevelSpawns {
        global_timer: Stopwatch::new(),
        stage_timer: Timer::from_seconds(60., TimerMode::Repeating),
        wave_timer: Timer::from_seconds(5., TimerMode::Repeating),
        current_stage: 0,
        spawns: vec![
            SpawnStage {
                mobs: vec![Box::new(SpawnMob {
                    enemy: EnemyType::Goblin,
                    count: 5,
                })],
            },
            SpawnStage {
                mobs: vec![
                    Box::new(SpawnMob {
                        enemy: EnemyType::Goblin,
                        count: 7,
                    }),
                    // SpawnMob {
                    //     enemy: EnemyType::Mushroom,
                    //     count: 2,
                    // },
                ],
            },
        ],
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn((Enemy {}));

    // commands.spawn()gcc=
}

fn check_for_spawns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut level_spawns: ResMut<LevelSpawns>,
    player_query: Query<(&Transform), (With<Player>, Without<Harmful>)>,
    time: Res<Time>,
) {
    let transform = player_query.single();

    level_spawns.global_timer.tick(time.delta());
    level_spawns.wave_timer.tick(time.delta());
    level_spawns.stage_timer.tick(time.delta());

    if level_spawns.wave_timer.just_finished() {
        if level_spawns.spawns.len() > level_spawns.current_stage {
            let waves = &level_spawns.spawns[level_spawns.current_stage];
            let player_position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y);
            println!("Spawn wave {}", level_spawns.current_stage);
            spawn_enemy_wave(
                level_spawns.current_stage,
                waves,
                &mut commands,
                &asset_server,
                &mut texture_atlases,
                player_position,
            );
        }
    }

    // Update stage to next stage if another stage exists in array.
    if level_spawns.stage_timer.just_finished() {
        println!("Stage finished");
        let new_stage = level_spawns.current_stage + 1;
        if level_spawns.spawns.len() > new_stage {
            level_spawns.current_stage = new_stage;
        } else {
            println!("No stage to progress to");
        }
    }
}

fn get_enemy_for_type(enemy: &EnemyType) -> impl SpawnedEnemy {
    match enemy {
        EnemyType::Goblin => Goblin {},
        // EnemyType::Mushroom => {}
        // EnemyType::Slime => {}
        _ => Goblin {},
    }
}

fn spawn_enemy_wave(
    current_stage_index: usize,
    stage: &SpawnStage,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    player_pos: Vec2,
) {
    for mob in &stage.mobs {
        for index in 0..mob.count {
            let to_spawn = get_enemy_for_type(&mob.enemy);
            let texture_handle = asset_server.load(to_spawn.get_sprite_location());
            println!("Spawn enemy no {} for stage {}", index, current_stage_index);
            // Spawns enemy
            to_spawn.new(
                texture_atlases,
                texture_handle,
                commands,
                current_stage_index,
                player_pos,
            );
        }
    }
}
