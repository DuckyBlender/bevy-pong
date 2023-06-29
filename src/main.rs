// Classic pong game

use bevy::prelude::*;

const PADDLE_SIZE: Vec2 = Vec2::new(BALL_SIZE, 50.);
const PADDLE_SPEED: f32 = 3.;
const PADDLE_OFFSET: f32 = BALL_SIZE * 5.;
const BALL_SPEED: f32 = 3.;
const BALL_SIZE: f32 = 10.;

const ARENA_WIDTH: f32 = 800.;
const ARENA_HEIGHT: f32 = 400.;

#[derive(Component)]
struct Collider;

enum WhichPaddle {
    Left,
    Right,
}

#[derive(Component)]
struct Paddle {
    side: WhichPaddle,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
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
        .add_startup_system(setup)
        .add_systems((paddle_movement, ball_movement, print_ball_location))
        .run();
}

fn setup(mut commands: Commands) {
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

    // Left paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-ARENA_WIDTH / 2. + PADDLE_OFFSET, 0.0, 1.0),
                scale: PADDLE_SIZE.extend(1.),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
            ..default()
        },
        Paddle {
            side: WhichPaddle::Left,
        },
        Collider,
    ));

    // Right paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(ARENA_WIDTH / 2. - PADDLE_OFFSET, 0.0, 1.0),
                scale: PADDLE_SIZE.extend(1.),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.5, 0.5),
                ..default()
            },
            ..default()
        },
        Paddle {
            side: WhichPaddle::Right,
        },
        Collider,
    ));

    // Ball
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 3.0),
                scale: Vec3::new(BALL_SIZE, BALL_SIZE, 1.),

                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),

                ..default()
            },

            ..default()
        },
        Ball {
            velocity: Vec2::new(BALL_SPEED, BALL_SPEED),
        },
        Collider,
    ));
}

fn print_ball_location(mut ball_query: Query<(&Ball, &Transform)>) {
    let ball = ball_query.single_mut();
    info!(
        "Ball: ({}, {}),",
        ball.1.translation.x, ball.1.translation.y
    );
}

// paddle movement, WS for left paddle, up/down for right paddle
fn paddle_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        let mut offset = 0.0;
        match paddle.side {
            WhichPaddle::Left => {
                if keyboard_input.pressed(KeyCode::W)
                    && transform.translation.y + PADDLE_SIZE.y / 2. < ARENA_HEIGHT / 2.
                // check if paddle is going out of bounds
                {
                    offset += PADDLE_SPEED;
                }
                if keyboard_input.pressed(KeyCode::S)
                    && transform.translation.y - PADDLE_SIZE.y / 2. > -ARENA_HEIGHT / 2.
                {
                    offset -= PADDLE_SPEED;
                }
            }
            WhichPaddle::Right => {
                if keyboard_input.pressed(KeyCode::Up)
                    && transform.translation.y + PADDLE_SIZE.y / 2. < ARENA_HEIGHT / 2.
                {
                    offset += PADDLE_SPEED;
                }
                if keyboard_input.pressed(KeyCode::Down)
                    && transform.translation.y - PADDLE_SIZE.y / 2. > -ARENA_HEIGHT / 2.
                {
                    offset -= PADDLE_SPEED;
                }
            }
        }
        transform.translation.y += offset;
    }
}

fn ball_movement(
    mut ball_query: Query<(&mut Ball, &mut Transform), Without<Paddle>>,
    paddle_query: Query<(&Paddle, &Transform)>,
) {
    let (mut ball, mut ball_transform) = ball_query.single_mut();

    // Bounce the ball off walls
    if ball_transform.translation.y < -ARENA_HEIGHT / 2. + BALL_SIZE / 2.
        || ball_transform.translation.y > ARENA_HEIGHT / 2. - BALL_SIZE / 2.
    {
        ball.velocity.y = -ball.velocity.y;
    }

    if ball_transform.translation.x < -ARENA_WIDTH / 2. + BALL_SIZE / 2.
        || ball_transform.translation.x > ARENA_WIDTH / 2. - BALL_SIZE / 2.
    {
        ball.velocity.x = -ball.velocity.x;
    }

    // Bounce the ball off paddles. Change the angle based on where it hits the paddle.
    for (paddle, paddle_transform) in paddle_query.iter() {
        let paddle_y = paddle_transform.translation.y - PADDLE_SIZE.y / 2.;
        let paddle_top = paddle_transform.translation.y + PADDLE_SIZE.y / 2.;
        let paddle_left = paddle_transform.translation.x - PADDLE_SIZE.x / 2.;
        let paddle_right = paddle_transform.translation.x + PADDLE_SIZE.x / 2.;

        if ball_transform.translation.x < paddle_right
            && ball_transform.translation.x > paddle_left
            && ball_transform.translation.y < paddle_top
            && ball_transform.translation.y > paddle_y
        {
            match paddle.side {
                WhichPaddle::Left => {
                    ball.velocity.x = -ball.velocity.x;
                    ball.velocity.y = (ball_transform.translation.y
                        - paddle_transform.translation.y)
                        / PADDLE_SIZE.y
                        * 5.;
                }
                WhichPaddle::Right => {
                    ball.velocity.x = -ball.velocity.x;
                    ball.velocity.y = (ball_transform.translation.y
                        - paddle_transform.translation.y)
                        / PADDLE_SIZE.y
                        * 5.;
                }
            }
        }
    }

    // Move the ball according to its velocity
    ball_transform.translation += ball.velocity.extend(0.0);
}
