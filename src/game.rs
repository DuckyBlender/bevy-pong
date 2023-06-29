use bevy::prelude::*;

// import consts from main.rs
use crate::*;

pub fn start_game(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
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
        Game,
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
        Game,
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
        Game,
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
        Game,
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
        Game,
    ));
}

pub fn paddle_movement(
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

pub fn ball_movement(
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

pub fn score_logic(
    mut score_event: EventReader<ScoreEvent>,
    mut scoreboard: ResMut<Score>,
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    mut score_query: Query<(&mut Text, &ScoreText)>,
    mut paddle_query: Query<(&Paddle, &mut Transform), Without<Ball>>,
) {
    if score_event.is_empty() {
        return;
    }

    let side = score_event.iter().next().unwrap().side;
    score_event.clear();
    // Update the correct side
    match side {
        Sides::Left => {
            scoreboard.left += 1;
            for (mut text, score_text) in score_query.iter_mut() {
                if score_text.side == Sides::Left {
                    text.sections[0].value = scoreboard.left.to_string();
                }
            }
        }
        Sides::Right => {
            scoreboard.right += 1;
            for (mut text, score_text) in score_query.iter_mut() {
                if score_text.side == Sides::Right {
                    text.sections[0].value = scoreboard.right.to_string();
                }
            }
        }
    }

    for (mut ball, mut transform) in ball_query.iter_mut() {
        transform.translation = Vec3::new(0.0, 0.0, 3.0);
        ball.velocity = Vec2::new(STARTING_BALL_SPEED, STARTING_BALL_SPEED);
    }

    // Reset paddle positions
    for (paddle, mut transform) in paddle_query.iter_mut() {
        match paddle.side {
            Sides::Left => {
                transform.translation = Vec3::new(-ARENA_WIDTH / 2. + PADDLE_OFFSET, 0.0, 1.0);
            }
            Sides::Right => {
                transform.translation = Vec3::new(ARENA_WIDTH / 2. - PADDLE_OFFSET, 0.0, 1.0);
            }
        }
    }
}

pub fn esc_check(
    mut commands: Commands,
    query: Query<Entity, With<Game>>,
    keyboard: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        // If esc is pressed, despawn all entities with a collider

        state.set(GameState::Menu);
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
