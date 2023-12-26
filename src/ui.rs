// use crate::player::CharacterLife;
use bevy::prelude::*;

use crate::player::{Player};
use crate::sprite::Health;
use crate::state::{GameState, GameplayOnly};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_setup)
        .add_systems(PostUpdate, ui_update);

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

#[derive(Component)]
struct HealthUiValue;

#[derive(Component)]
struct HealthUiBar;

fn ui_setup(mut commands: Commands) {
    // Health bar

    let parent_node = (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                width: Val::Percent(5.0),
                height: Val::Percent(2.0),
                 
                // size: Size::new(Val::Percent(5.0), Val::Percent(2.0)),
                
                left: Val::Percent(47.5),
                right: Val::Auto,
                top: Val::Percent(60.0),
                bottom: Val::Auto,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        GameplayOnly,
        HealthUiBar,
        Name::new("Health Bar UI"),
    );

    let health_node = (
        NodeBundle {
            style: Style {
                width: Val::Percent(0.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::RED),
            ..default()
        },
        HealthUiValue,
        Name::new("Health Bar Filled UI"),
    );

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(health_node);
    });

    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             width: Val::Percent(100.0),
    //             height: Val::Percent(100.0),
    //             align_items: AlignItems::Center,
    //             justify_content: JustifyContent::Center,
    //             flex_direction: FlexDirection::Column,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn(
    //             (
    //                 TextBundle::from_section(
    //                     (100.0).to_string(),
    //                     TextStyle {
    //                         font_size: 40.0,
    //                         color: Color::ORANGE,
    //                         ..default()
    //                     },
    //                 )
    //                 .with_style(Style {
    //                     position_type: PositionType::Absolute,
    //                     bottom: Val::Px(5.0),
    //                     left: Val::Px(5.0),
    //                     ..default()
    //                 }),
    //                 HealthUiValue
    //             )
    //         );
            // parent.spawn((
            //     TextBundle::from_section(
            //         "Orange: nothing counted yet".to_string(),
            //         TextStyle {
            //             font_size: 80.0,
            //             color: Color::ORANGE,
            //             ..default()
            //         },
            //     ),
            //     OrangeCount,
            // ));
        // });
}

fn ui_update(
    // mut commands: Commands,
    // mut game_state: ResMut<NextState<GameState>>,
    player_query: Query<&Health, With<Player>>,
    mut ui_health_query: Query<&mut Style, With<HealthUiValue>>,
    // assets: Res<AssetServer>
){
    // let health = player_query.single();

    // let mut health_block = ui_health_query.single_mut(); 

    // health_block.width = Val::Percent(health.0);

    // health_lbl.
 
    // for (mut health_lbl) in ui_health_query.iter_mut() {

    // }

    // health_lbl.sections[0].value = format!("{}", health.0);    // health_lbl;
}
