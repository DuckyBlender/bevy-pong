// Classic pong game

use bevy::prelude::*;

pub const PADDLE_SIZE: Vec2 = Vec2::new(BALL_SIZE, 50.);
pub const PADDLE_SPEED: f32 = 3.;
pub const PADDLE_OFFSET: f32 = BALL_SIZE * 5.;

pub const BALL_SPEED: f32 = 3.;
pub const STARTING_BALL_SPEED: f32 = BALL_SPEED / 3.;
pub const BALL_SIZE: f32 = 10.;
pub const BALL_MAX_ANGLE_MULTIPLIER: f32 = 6.;

pub const ARENA_WIDTH: f32 = 800.;
pub const ARENA_HEIGHT: f32 = 400.;

mod game;
use game::*;

mod menu;
use menu::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Menu,
    #[default]
    Playing,
}

#[derive(Component)]
pub struct Collider;

#[derive(Clone, Copy, PartialEq)]
pub enum Sides {
    Left,
    Right,
}

#[derive(Component)]
pub struct Paddle {
    side: Sides,
}

#[derive(Component)]
pub struct Ball {
    velocity: Vec2,
}

#[derive(Resource)]
pub struct Score {
    left: u32,
    right: u32,
}

#[derive(Component)]
pub struct ScoreText {
    side: Sides,
}

// scoreboard update event
#[derive(Component)]
pub struct ScoreEvent {
    side: Sides,
}

fn main() {
    let window = WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Pong".into(),
            resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
            resizable: false,
            mode: bevy::window::WindowMode::Windowed,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window))
        .add_startup_system(setup_common)
        .insert_resource(Score { left: 0, right: 0 })
        .add_event::<ScoreEvent>()
        .add_state::<GameState>()
        // MENU
        .add_system(startup_menu.in_schedule(OnEnter(GameState::Menu)))
        // .add_systems((listen_menu))
        // PLAYING
        .add_system(start_game.in_schedule(OnEnter(GameState::Playing)))
        .add_systems((paddle_movement, ball_movement, score_logic))
        .run();
}

fn setup_common(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Black Background
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(ARENA_WIDTH, ARENA_HEIGHT, 1.),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    });
}
