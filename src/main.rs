use bevy::asset::io::processor_gated::TransactionLockedReader;
use bevy::color::palettes::css::BLUE;
use bevy::color::ColorRange;
use bevy::math::NormedVectorSpace;
use bevy::transform;
use bevy::{prelude::*, window::WindowResolution, color::palettes::css::*, input::InputPlugin,text::{FontSmoothing, LineBreak, TextBounds},};
//use bevy_gizmos::GizmoPlugin;
//use bevy::gizmos::arrows;
use std::path::Path;

///
/// https://bevyengine.org/examples/
///     lotta cool stuff
/// 
fn main() {
    println!("Hello, world!");

    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(900.0, 900.0).with_scale_factor_override(1.0),
            ..default()
        }),
        close_when_requested: true,
        exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
    }))
    .insert_resource(XShot{
        x:0.0,
        first_choice:false,
    })
    .insert_resource(YShot{
        y:0.0,
        second_choice:false,
    })
    .insert_resource(Power{
        power:0.0,
        active:false,
        shot_taken:false,
        shot_taken2:false,
        shot_over:false,
    })
    //.add_plugins(GizmoPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, shot_selection)
    .run();
}
#[derive(Component, Debug)]
enum Direction{
    Up,
    Down,
    Left,
    Right,
    Stop,
}


#[derive(Component, Debug)]
enum BarDirection{
    Up,
    Down,
    Stop,
}
#[derive(Component, Debug)]
enum PowerBar{
    Grow,
    Shrink,
    Off,
}

#[derive(Component,Debug)]
enum Ball{
    Move,
    Stop,
}
#[derive(Component,Debug)]
enum Rotate{
    On,
    Off,
}

//https://bevyengine.org/examples/transforms/scale/
#[derive(Component)]
struct Scaling {
    scale_direction: Vec3,
    scale_speed: f32,
    max_size: f32,
    min_size: f32,
}

impl Scaling {
    fn new() -> Self {
        Scaling {
            scale_direction: Vec3::new(1.0, 1.0, 1.0),
            scale_speed: 5.0,
            max_size: 20.0,
            min_size: 0.5,
        }
    }
}



fn setup(mut commands: Commands, 
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>,
         asset_server: Res<AssetServer>,
         mut gizmos: Gizmos,
         time: Res<Time>,
){

    commands.spawn(Camera2d);

    // https://stackoverflow.com/questions/66199782/how-to-add-a-background-image-to-2d-game-in-bevy
    //      how to load background pic
    
    //let backgroud_pic: Handle<Image> = asset_server.load("green-grass-background.jpg");

    // https://taintedcoders.com/bevy/assets
    // note, image was converted from jpg to png to satisfy bevy
    let path = Path::new("green-grass-background (1).png");
    //let asset_path = AssetPath::from_path(path);

    /* 
    commands.spawn(Sprite {
        image: asset_server.load(asset_path),
        ..Default::default()
    });
    */
    // https://docs.rs/bevy/latest/bevy/asset/struct.AssetServer.html
    commands.spawn((
        Sprite::from_image(asset_server.load(path)),
        Transform::from_xyz(0.0, 0.0, -100.0),
    ));
    

    let post_width = 7.0;
    let post_length = 250.0;
    let crossbar_length = 550.0;

    let posts = [
        meshes.add(Capsule2d::new(post_width,post_length)),
        meshes.add(Capsule2d::new(post_width,crossbar_length)),
        meshes.add(Capsule2d::new(post_width,post_length)),
    ];

    let post_x_pos = 275.0;
    let post_y_pos=175.0;
    let crossbar_x_pos=0.0;
    let crossbar_y_pos=300.0;

    commands.spawn((Mesh2d(posts[0].clone()),
                            MeshMaterial2d(materials.add(Color::WHITE)),
                            Transform::from_xyz(-post_x_pos,post_y_pos,0.0)
    ));  

    commands.spawn((Mesh2d(posts[2].clone()),
                            MeshMaterial2d(materials.add(Color::WHITE)),
                            Transform::from_xyz(post_x_pos,post_y_pos,0.0)
    ));  

    // https://bevy-cheatbook.github.io/fundamentals/transforms.html
    //     rotation 
    commands.spawn((Mesh2d(posts[1].clone()),
                            MeshMaterial2d(materials.add(Color::WHITE)),
                            Transform::from_xyz(crossbar_x_pos,crossbar_y_pos,0.0)
                            .with_rotation(Quat::from_rotation_z((90.0_f32).to_radians()))
    ));  


    // add outlines to posts since background is too bright

    let post_outlines = [
        meshes.add(Capsule2d::new(post_width+5.0,post_length+2.0)),
        meshes.add(Capsule2d::new(post_width+5.0,crossbar_length)),
        meshes.add(Capsule2d::new(post_width+5.0,post_length+2.0)),
    ];
    
    commands.spawn((Mesh2d(post_outlines[0].clone()),
    MeshMaterial2d(materials.add(Color::BLACK)),
    Transform::from_xyz(-post_x_pos,post_y_pos,-1.0)
    ));  

    commands.spawn((Mesh2d(post_outlines[2].clone()),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(post_x_pos,post_y_pos,-1.0)
    ));  

    // https://bevy-cheatbook.github.io/fundamentals/transforms.html
    //     rotation 
    commands.spawn((Mesh2d(post_outlines[1].clone()),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(crossbar_x_pos,crossbar_y_pos,-1.0)
        .with_rotation(Quat::from_rotation_z((90.0_f32).to_radians()))
    ));  


    // add penalty spot

    let penalty_spot = meshes.add(Circle::new(50.0));

    commands.spawn((
        Mesh2d(penalty_spot),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -375.0, 0.0)
    ));

    let penalty_spot_outline= meshes.add(Circle::new(55.0));
    commands.spawn((
        Mesh2d(penalty_spot_outline),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(0.0, -375.0, -1.0)
    ));


    // add soccer ball

    /*
    // downloaded from sketch fab
    // https://www.fab.com/listings/d18f66e9-932e-44b8-8ba2-7f4c1fbcbac3
    let my_gltf = asset_server.load("football_anime.glb#Scene0");

    commands.spawn((
        SceneRoot(my_gltf),
        Transform::from_xyz(0.0, -370.0, 10.0),
    ));

     */

    // add soccer ball
    // original from freepik https://www.freepik.com/free-psd/classic-black-white-soccer-ball-transparent-background_406618570.htm#fromView=search&page=1&position=4&uuid=3e4bafb2-eb92-4d79-bcf1-8d5f4ddeb279&query=soccer+ball
    // cropped using https://www.remove.bg/upload
    let ball_path = Path::new("cropped_ball.png");
    commands.spawn((
        Sprite::from_image(asset_server.load(ball_path)),
        Transform::from_xyz(0.0, -340.0, 10.0).with_scale(Vec3::splat(0.4)),
        Ball::Stop,
    ));



    // add oscilating triangle and rectangle to make an arrow in front of ball

    /*
    bottom of rect is y-val - (rectHeight/2)
    top    of rect is y-val + (rectHeight/2)


    */ 

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

    let parent = commands.spawn((
        Transform::default(),
        Direction::Left,
    )).id();

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


    let bar = meshes.add(Rectangle::new(rectangle_width*2.0,850.0));
    commands.spawn((
        Mesh2d(bar),
        MeshMaterial2d(materials.add(Color::from(BLACK))),
        Transform::from_xyz(400.0, -400.0, 1.0),
        
    ));

    let power_bar = meshes.add(Rectangle::new(rectangle_width*2.0,rectangle_width*0.3));
    commands.spawn((
        Mesh2d(power_bar),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(400.0, -200.0, 1.5),
        PowerBar::Off,
        BarDirection::Up,
    ));


    // https://github.com/bevyengine/bevy/blob/v0.13.0/examples/2d/rotation.rs
    let keeper = meshes.add(Rectangle::new(16.0,500.0 ));
    commands.spawn((
        Mesh2d(keeper),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform{
            translation: Vec3::new(0.0,200.0,1.0),
            rotation: Quat::from_rotation_z(0.0),
            ..Default::default()
        },
        Rotate::On,
    ));



}


#[derive(Resource)]
struct XShot{
    x:f32,
    first_choice:bool,
}

#[derive(Resource)]
struct YShot{
    y:f32,
    second_choice:bool,
}
#[derive(Resource)]
struct Power{
    power:f32,
    active:bool,
    shot_taken:bool,
    shot_taken2:bool,
    shot_over:bool,

}

// https://bevyengine.org/examples/2d-rendering/move-sprite/
fn shot_selection(time: Res<Time>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut queries: ParamSet<(
        Query::<(&mut Direction, &mut Transform)>,
        Query<(&mut BarDirection, &mut Transform), With<PowerBar>>,
        Query<(&mut Ball, &mut Transform), With<Ball>>,
        Query<(&mut Rotate, &mut Transform), With<Rotate>>,
    )>,
    mut x: ResMut<XShot>,
    mut y: ResMut<YShot>,
    mut power: ResMut<Power>,
    input: Res<ButtonInput<KeyCode>>,
) {

    let x_range = 300.0;
    let y_range = 300.0;
    let mut speed = 5.0;
    for (mut keeper, mut transform)in &mut queries.p3().iter_mut(){
        match *keeper{
            Rotate::On => {
                let speed = 2.0; 
                let rotation = Quat::from_rotation_z(speed * time.delta_secs());
                transform.rotation *= rotation; 
            },
            Rotate::Off => {}
                

        }

        if power.shot_over == true{
            *keeper = Rotate::Off;

        }
    }

    for (mut arrow, mut transform) in &mut queries.p0().iter_mut(){
        match *arrow{
            Direction::Left => transform.translation.x -=x_range *time.delta_secs(),
            Direction::Right => transform.translation.x +=x_range *time.delta_secs(),
            Direction::Up => transform.translation.y -=x_range *time.delta_secs(),
            Direction::Down => transform.translation.y +=x_range *time.delta_secs(),
            Direction::Stop =>{},
        }

        if x.first_choice != true && input.just_pressed(KeyCode::KeyX){
            x.first_choice = true;
            x.x = transform.translation.x;
            println!("X: {}", x.x);
            *arrow = Direction::Stop;
            *arrow = Direction::Up;
        }
        else if x.first_choice == true && y.second_choice!=true && input.just_pressed(KeyCode::KeyY){
            y.second_choice = true;
            y.y = transform.translation.y;
            println!("y: {}", y.y);
            *arrow = Direction::Stop;
        }



        if transform.translation.x>x_range && x.first_choice != true{
            *arrow = Direction::Left;
        } else if transform.translation.x< -x_range &&x.first_choice!= true{
            *arrow = Direction::Right;
        }


        if transform.translation.y>y_range && y.second_choice != true{
            *arrow = Direction::Up;
        } else if transform.translation.y< -y_range&&y.second_choice!=true{
            *arrow = Direction::Down;
        }
        
        if x.first_choice == true && y.second_choice == true{
            *arrow = Direction::Stop;
            power.active = true;
        }

    }

    // if shot direction is picked, we can now pick shot power
    // https://bevyengine.org/examples/transforms/scale/
    

    if power.active {
        for (mut bar, mut transform) in &mut queries.p1().iter_mut() {
            match *bar {
                BarDirection::Up => transform.translation.y +=x_range *3.0*time.delta_secs(),
                BarDirection::Down => transform.translation.y -=x_range *3.0*time.delta_secs(),
                BarDirection::Stop =>{},
            }

           
           if transform.translation.y >0.0{
            *bar = BarDirection::Down;
           }

           if transform.translation.y < -450.0{
            *bar = BarDirection::Up;
           }

            // Reset the power bar if the 'Z' key is pressed
            if input.just_pressed(KeyCode::KeyZ) {
                power.power = transform.translation.y;
                println!("Power Level: {}", power.power);

                power.active = false;
                *bar = BarDirection::Stop;
                if power.shot_taken != true{
                    power.shot_taken = true;
                    //take_shot(x.x, y.y, power.power, &mut queries.p2(), time,power);
                    //println!("we do thisright?");
                    return;
                }
            }

        }
        

    }


    if power.shot_taken == true{
        //let shot_x = x.x+300.0;
        let shot_x = x.x;
        let shot_y = y.y+300.0;
        let shot_z = power.power+450.0;
        let ball_y = -340.0;
        //let dest = Vec2::new(shot_x,shot_y-ball_y);

        // combine y and z to get distance / height of shot
        // 300.0 < y+z < 650.0
        // then scale so lower bound is 150 (bottom of goal) and upper is 400 (top of goal)

        let mut yz = shot_y+shot_z;
        let m = (300.0-150.0) / (650.0-400.0);
        let b = 400.0 - m*650.0;

        yz = m*yz + b;

        let dest = Vec2::new(x.x, yz + 50.0);
        let keeper = queries.p3().single().1.clone();

        for (mut ball, mut transform) in queries.p2().iter_mut(){
            match *ball{
                Ball::Move=>{
                // https://docs.rs/bevy/latest/bevy/prelude/struct.Vec3.html
                let direction = (dest-transform.translation.truncate()).normalize();
                let speed = shot_z;
                let distance = speed * time.delta_secs();
                let new_position = transform.translation + direction.extend(1.0) * distance;
                transform.translation = new_position;
                
                if transform.scale.x > 0.1{
                    transform.scale -= Vec3::splat(speed/150000.0);
                }


                },
                Ball::Stop => {},
            }

            

            if power.shot_taken2!= true{
                power.shot_taken2 = true;
                *ball = Ball::Move;
            }


            if transform.translation.y > dest.y{
                *ball = Ball::Stop;
                power.shot_over = true;

                // handle outcomes
                if transform.translation.x < -277.5 || transform.translation.x >272.5{
                    commands.spawn((
                        Sprite::from_color(Color::BLACK, Vec2::new(300.0,50.0)),
                        
                    ));
                    commands.spawn((
                        Text2d::new("Missed Wide!"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Transform{
                            translation:Vec3::new(0.0,0.0,100.0),
                            ..Default::default()
                        },
                    ));
                }
                else if transform.translation.y > 309.5{
                    commands.spawn((
                        Sprite::from_color(Color::BLACK, Vec2::new(300.0,50.0)),
                        
                    ));
                    commands.spawn((
                        Text2d::new("Missed High!"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Transform{
                            translation:Vec3::new(0.0,0.0,100.0),
                            ..Default::default()
                        },
                    ));
                }
                else if (transform.translation.y<309.5&&transform.translation.y>40.5&&
                        transform.translation.x>-277.5&&transform.translation.x<272.5){
                            // if it is touching keep, its saved, else its a goal
                            let distance = transform.translation.distance(keeper.translation);
                            return;
                        }

            }

                
            }
            
            
        

        
    }

    
    

}
