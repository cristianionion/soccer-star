use bevy::{color::palettes::css::*, prelude::*, window::WindowResolution};
use std::path::Path;

///
/// https://bevyengine.org/examples/
///     lotta cool stuff
///
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(900.0, 900.0).with_scale_factor_override(1.0),
                ..default()
            }),
            close_when_requested: true,
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
        }))
        .insert_resource(XShot {
            x: 0.0,
            first_choice: false,
        })
        .insert_resource(YShot {
            y: 0.0,
            second_choice: false,
        })
        .insert_resource(Power {
            power: 0.0,
            active: false,
            shot_taken: false,
            shot_taken2: false,
            shot_over: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, shot_selection)
        .run();
}

// components to allow functionality for arrow, powerbar, and ball
#[derive(Component, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

#[derive(Component, Debug)]
enum BarDirection {
    Up,
    Down,
    Stop,
}
#[derive(Component, Debug)]
enum PowerBar {
    Off,
}

#[derive(Component, Debug)]
enum Ball {
    Move,
    Stop,
}

//https://bevyengine.org/examples/transforms/scale/

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // https://stackoverflow.com/questions/66199782/how-to-add-a-background-image-to-2d-game-in-bevy
    //      how to load background pic

    // https://taintedcoders.com/bevy/assets
    // note, image was converted from jpg to png to satisfy bevy
    let path = Path::new("green-grass-background (1).png");

    // https://docs.rs/bevy/latest/bevy/asset/struct.AssetServer.html
    commands.spawn((
        Sprite::from_image(asset_server.load(path)),
        Transform::from_xyz(0.0, 0.0, -100.0),
    ));

    // create goalposts and crossbar
    let post_width = 7.0;
    let post_length = 250.0;
    let crossbar_length = 550.0;

    let posts = [
        meshes.add(Capsule2d::new(post_width, post_length)),
        meshes.add(Capsule2d::new(post_width, crossbar_length)),
        meshes.add(Capsule2d::new(post_width, post_length)),
    ];

    let post_x_pos = 275.0;
    let post_y_pos = 175.0;
    let crossbar_x_pos = 0.0;
    let crossbar_y_pos = 300.0;

    commands.spawn((
        Mesh2d(posts[0].clone()),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(-post_x_pos, post_y_pos, 0.0),
    ));

    commands.spawn((
        Mesh2d(posts[2].clone()),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(post_x_pos, post_y_pos, 0.0),
    ));

    // https://bevy-cheatbook.github.io/fundamentals/transforms.html
    //     rotation on crossbar
    commands.spawn((
        Mesh2d(posts[1].clone()),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(crossbar_x_pos, crossbar_y_pos, 0.0)
            .with_rotation(Quat::from_rotation_z((90.0_f32).to_radians())),
    ));

    // add outlines to posts since background is too bright

    let post_outlines = [
        meshes.add(Capsule2d::new(post_width + 5.0, post_length + 2.0)),
        meshes.add(Capsule2d::new(post_width + 5.0, crossbar_length)),
        meshes.add(Capsule2d::new(post_width + 5.0, post_length + 2.0)),
    ];

    commands.spawn((
        Mesh2d(post_outlines[0].clone()),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(-post_x_pos, post_y_pos, -1.0),
    ));

    commands.spawn((
        Mesh2d(post_outlines[2].clone()),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(post_x_pos, post_y_pos, -1.0),
    ));

    // https://bevy-cheatbook.github.io/fundamentals/transforms.html
    //     rotation on crossbar outline
    commands.spawn((
        Mesh2d(post_outlines[1].clone()),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(crossbar_x_pos, crossbar_y_pos, -1.0)
            .with_rotation(Quat::from_rotation_z((90.0_f32).to_radians())),
    ));

    // add penalty spot

    let penalty_spot = meshes.add(Circle::new(50.0));

    commands.spawn((
        Mesh2d(penalty_spot),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -375.0, 0.0),
    ));

    let penalty_spot_outline = meshes.add(Circle::new(55.0));
    commands.spawn((
        Mesh2d(penalty_spot_outline),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(0.0, -375.0, -1.0),
    ));

    // add soccer ball
    // original from freepik https://www.freepik.com/free-psd/classic-black-white-soccer-ball-transparent-background_406618570.htm#fromView=search&page=1&position=4&uuid=3e4bafb2-eb92-4d79-bcf1-8d5f4ddeb279&query=soccer+ball
    // cropped using https://www.remove.bg/upload
    let ball_path = Path::new("cropped_ball.png");
    commands.spawn((
        Sprite::from_image(asset_server.load(ball_path)),
        Transform::from_xyz(0.0, -340.0, 10.0).with_scale(Vec3::splat(0.4)),
        Ball::Stop,
    ));

    // add bouncing triangle and rectangle to make an arrow in front of ball

    let rectangle_width = 20.0;
    let rectangle_size = 250.0;
    let triangle_width = 60.0;
    let rectangle_height = -100.0;
    let triangle_height = 50.0;

    let arrow = [
        meshes.add(Rectangle::new(rectangle_width, rectangle_size)),
        meshes.add(Triangle2d::new(
            Vec2::new(-triangle_width, -triangle_height / 2.0),
            Vec2::new(triangle_width, -triangle_height / 2.0),
            Vec2::new(0.0, triangle_height / 2.0),
        )),
    ];

    let parent = commands.spawn((Transform::default(), Direction::Left)).id();

    commands.entity(parent).with_children(|parent| {
        parent.spawn((
            Mesh2d(arrow[0].clone()),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_xyz(0.0, rectangle_height, 1.0),
        ));
        parent.spawn((
            Mesh2d(arrow[1].clone()),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_xyz(0.0, rectangle_height + (rectangle_size / 2.0), 1.0),
        ));
    });

    // add powerbar
    let bar = meshes.add(Rectangle::new(rectangle_width * 2.0, 850.0));
    commands.spawn((
        Mesh2d(bar),
        MeshMaterial2d(materials.add(Color::from(BLACK))),
        Transform::from_xyz(400.0, -400.0, 1.0),
    ));

    let power_bar = meshes.add(Rectangle::new(rectangle_width * 2.0, rectangle_width * 0.3));
    commands.spawn((
        Mesh2d(power_bar),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(400.0, -200.0, 1.5),
        PowerBar::Off,
        BarDirection::Up,
    ));
}

// resources to store user decisions and to allow order of game decisions
#[derive(Resource)]
struct XShot {
    x: f32,
    first_choice: bool,
}

#[derive(Resource)]
struct YShot {
    y: f32,
    second_choice: bool,
}
#[derive(Resource)]
struct Power {
    power: f32,
    active: bool,
    shot_taken: bool,
    shot_taken2: bool,
    shot_over: bool,
}

// https://bevyengine.org/examples/2d-rendering/move-sprite/

// cargo clippy error i wasnt able to fix was the ParamSet
fn shot_selection(
    time: Res<Time>,
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<(&mut Direction, &mut Transform)>,
        Query<(&mut BarDirection, &mut Transform), With<PowerBar>>,
        Query<(&mut Ball, &mut Transform), With<Ball>>,
    )>,
    mut x: ResMut<XShot>,
    mut y: ResMut<YShot>,
    mut power: ResMut<Power>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let x_range = 400.0;
    let y_range = 300.0;

    // handle the arrow's movements
    for (mut arrow, mut transform) in &mut queries.p0().iter_mut() {
        match *arrow {
            Direction::Left => transform.translation.x -= x_range * 3.5 * time.delta_secs(),
            Direction::Right => transform.translation.x += x_range * 3.5 * time.delta_secs(),
            Direction::Up => transform.translation.y -= x_range * 3.5 * time.delta_secs(),
            Direction::Down => transform.translation.y += x_range * 3.5 * time.delta_secs(),
            Direction::Stop => {}
        }

        if !x.first_choice && input.just_pressed(KeyCode::KeyX) {
            x.first_choice = true;
            x.x = transform.translation.x;
            *arrow = Direction::Stop;
            *arrow = Direction::Up;
        } else if x.first_choice && !y.second_choice && input.just_pressed(KeyCode::KeyY) {
            y.second_choice = true;
            y.y = transform.translation.y;
            *arrow = Direction::Stop;
        }

        if transform.translation.x > x_range && !x.first_choice {
            *arrow = Direction::Left;
        } else if transform.translation.x < -x_range && !x.first_choice {
            *arrow = Direction::Right;
        }

        if transform.translation.y > y_range && !y.second_choice {
            *arrow = Direction::Up;
        } else if transform.translation.y < -y_range && !y.second_choice {
            *arrow = Direction::Down;
        }

        if x.first_choice && y.second_choice {
            *arrow = Direction::Stop;
            power.active = true;
        }
    }
    // if shot direction is picked, we can now pick shot power

    if power.active {
        // handle powerball actions
        for (mut bar, mut transform) in &mut queries.p1().iter_mut() {
            match *bar {
                BarDirection::Up => transform.translation.y += x_range * 3.0 * time.delta_secs(),
                BarDirection::Down => transform.translation.y -= x_range * 3.0 * time.delta_secs(),
                BarDirection::Stop => {}
            }

            if transform.translation.y > 0.0 {
                *bar = BarDirection::Down;
            }

            if transform.translation.y < -450.0 {
                *bar = BarDirection::Up;
            }

            // Reset the power bar if the 'Z' key is pressed
            if input.just_pressed(KeyCode::KeyZ) {
                power.power = transform.translation.y;

                power.active = false;
                *bar = BarDirection::Stop;
                if !power.shot_taken {
                    power.shot_taken = true;
                }
            }
        }
    }

    // calculate shot to be taken, and take it
    if power.shot_taken {
        // increase values to be above the negative initial values
        let shot_y = y.y + 300.0;
        let shot_z = power.power + 450.0;

        // combine y and z to get distance / height of shot
        // 300.0 < y+z < 650.0
        // then scale so lower bound is 50 (bottom of goal) and upper is 300 (top of goal)

        let mut yz = shot_y + shot_z;
        let m = (300.0 - 50.0) / (650.0 - 300.0);
        let b = 400.0 - m * 650.0;

        yz = m * yz + b;

        let dest = Vec2::new(x.x, yz);

        // handle ball movements
        for (mut ball, mut transform) in queries.p2().iter_mut() {
            match *ball {
                Ball::Move => {
                    // https://docs.rs/bevy/latest/bevy/prelude/struct.Vec3.html
                    let direction = (dest - transform.translation.truncate()).normalize();
                    let speed = shot_z;
                    let distance = speed * time.delta_secs();
                    let new_position = transform.translation + direction.extend(1.0) * distance;
                    transform.translation = new_position;

                    if transform.scale.x > 0.1 {
                        transform.scale -= Vec3::splat(speed / 150000.0);
                    }
                }
                Ball::Stop => {}
            }

            if !power.shot_taken2 {
                power.shot_taken2 = true;
                *ball = Ball::Move;
            }

            if transform.translation.y > dest.y {
                *ball = Ball::Stop;
                power.shot_over = true;

                // handle outcomes
                if transform.translation.x < -277.5 || transform.translation.x > 272.5 {
                    commands.spawn((Sprite::from_color(Color::BLACK, Vec2::new(300.0, 50.0)),));
                    commands.spawn((
                        Text2d::new("Missed Wide!"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Transform {
                            translation: Vec3::new(0.0, 0.0, 200.0),
                            ..Default::default()
                        },
                    ));
                } else if transform.translation.y > 309.5 {
                    commands.spawn((Sprite::from_color(Color::BLACK, Vec2::new(300.0, 50.0)),));
                    commands.spawn((
                        Text2d::new("Missed High!"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Transform {
                            translation: Vec3::new(0.0, 0.0, 200.0),
                            ..Default::default()
                        },
                    ));
                } else if transform.translation.y < 287.5
                    && transform.translation.y > 50.0
                    && transform.translation.x > -251.0
                    && transform.translation.x < 252.0
                {
                    commands.spawn((Sprite::from_color(Color::BLACK, Vec2::new(300.0, 50.0)),));
                    commands.spawn((
                        Text2d::new("GOOOAAALLL!!!"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Transform {
                            translation: Vec3::new(0.0, 0.0, 200.0),
                            ..Default::default()
                        },
                    ));
                } else if transform.translation.y < 50.0 {
                    commands.spawn((Sprite::from_color(Color::BLACK, Vec2::new(300.0, 50.0)),));
                    commands.spawn((
                        Text2d::new("Missed Short!"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Transform {
                            translation: Vec3::new(0.0, 0.0, 200.0),
                            ..Default::default()
                        },
                    ));
                } else {
                    commands.spawn((Sprite::from_color(Color::BLACK, Vec2::new(300.0, 50.0)),));
                    commands.spawn((
                        Text2d::new("Hit Post!"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Transform {
                            translation: Vec3::new(0.0, 0.0, 200.0),
                            ..Default::default()
                        },
                    ));
                }
            }
        }
    }
}
