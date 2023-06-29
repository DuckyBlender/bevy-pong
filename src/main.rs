// Classic pong game

use bevy::prelude::*;

const PADDLE_SIZE: Vec2 = Vec2::new(BALL_SIZE, 50.);
const PADDLE_SPEED: f32 = 3.;
const PADDLE_OFFSET: f32 = BALL_SIZE * 5.;

const BALL_SPEED: f32 = 3.;
const STARTING_BALL_SPEED: f32 = BALL_SPEED / 3.;
const BALL_SIZE: f32 = 10.;
const BALL_MAX_ANGLE_MULTIPLIER: f32 = 6.;

const ARENA_WIDTH: f32 = 800.;
const ARENA_HEIGHT: f32 = 400.;

#[derive(Component)]
struct Collider;

#[derive(Clone, Copy, PartialEq)]
enum Sides {
    Left,
    Right,
}

#[derive(Component)]
struct Paddle {
    side: Sides,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

#[derive(Resource)]
struct Score {
    left: u32,
    right: u32,
}

#[derive(Component)]
struct ScoreText {
    side: Sides,
}

// scoreboard update event
#[derive(Component)]
struct ScoreEvent {
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
        .add_startup_system(setup)
        .add_systems((paddle_movement, ball_movement, update_scoreboard))
        .insert_resource(Score { left: 0, right: 0 })
        .add_event::<ScoreEvent>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
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
        Paddle { side: Sides::Left },
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
        Paddle { side: Sides::Right },
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
            velocity: Vec2::new(STARTING_BALL_SPEED, STARTING_BALL_SPEED),
        },
        Collider,
    ));

    let font = asset_server.load("fonts/blocky.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    // Left score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(score.left.to_string(), text_style.clone()),
            transform: Transform {
                translation: Vec3::new(-ARENA_WIDTH / 4., ARENA_HEIGHT / 3., 2.),
                ..default()
            },
            ..default()
        },
        ScoreText { side: Sides::Left },
    ));

    // Right score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(score.right.to_string(), text_style),
            transform: Transform {
                translation: Vec3::new(ARENA_WIDTH / 4., ARENA_HEIGHT / 3., 2.),
                ..default()
            },
            ..default()
        },
        ScoreText { side: Sides::Right },
    ));
}

fn paddle_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        let mut offset = 0.0;
        match paddle.side {
            Sides::Left => {
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
            Sides::Right => {
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
    mut score_event: EventWriter<ScoreEvent>,
) {
    let (mut ball, mut ball_transform) = ball_query.single_mut();

    // Bounce the ball off top and bottom walls
    if ball_transform.translation.y < -ARENA_HEIGHT / 2. + BALL_SIZE / 2.
        || ball_transform.translation.y > ARENA_HEIGHT / 2. - BALL_SIZE / 2.
    {
        ball.velocity.y = -ball.velocity.y;
    }

    // If the ball goes off the left or right edges, reset it
    if ball_transform.translation.x < -ARENA_WIDTH / 2. {
        info!("Right player scores!");
        // Update scoreboard
        score_event.send(ScoreEvent { side: Sides::Right });

        ball_transform.translation = Vec3::new(0.0, 0.0, 3.0);
        ball.velocity = Vec2::new(STARTING_BALL_SPEED, STARTING_BALL_SPEED);
    } else if ball_transform.translation.x > ARENA_WIDTH / 2. {
        info!("Left player scores!");
        // Update scoreboard
        score_event.send(ScoreEvent { side: Sides::Left });

        ball_transform.translation = Vec3::new(0.0, 0.0, 3.0);
        ball.velocity = Vec2::new(-STARTING_BALL_SPEED, -STARTING_BALL_SPEED);
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
                Sides::Left => {
                    // If this is the first time, accelerate the ball
                    if ball.velocity.x == STARTING_BALL_SPEED {
                        ball.velocity.x = BALL_SPEED
                    }
                    ball.velocity.x = -ball.velocity.x;
                    ball.velocity.y = (ball_transform.translation.y
                        - paddle_transform.translation.y)
                        / PADDLE_SIZE.y
                        * BALL_MAX_ANGLE_MULTIPLIER;
                }
                Sides::Right => {
                    if ball.velocity.x == STARTING_BALL_SPEED {
                        ball.velocity.x = BALL_SPEED
                    }
                    ball.velocity.x = -ball.velocity.x;
                    ball.velocity.y = (ball_transform.translation.y
                        - paddle_transform.translation.y)
                        / PADDLE_SIZE.y
                        * BALL_MAX_ANGLE_MULTIPLIER;
                }
            }
        }
    }

    // Move the ball according to its velocity
    ball_transform.translation += ball.velocity.extend(0.0);
}

fn update_scoreboard(
    mut score_event: EventReader<ScoreEvent>,
    mut scoreboard: ResMut<Score>,
    mut query: Query<(&mut Text, &ScoreText)>,
) {
    if !score_event.is_empty() {
        let side = score_event.iter().next().unwrap().side;
        score_event.clear();
        for (mut text, score_text) in query.iter_mut() {
            match score_text.side {
                Sides::Left => {
                    if side == Sides::Left {
                        scoreboard.left += 1;
                        text.sections[0].value = scoreboard.left.to_string();
                    }
                }
                Sides::Right => {
                    if side == Sides::Right {
                        scoreboard.right += 1;
                        text.sections[0].value = scoreboard.right.to_string();
                    }
                }
            }
        }
    }
}
