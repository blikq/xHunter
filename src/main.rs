use bevy::transform::commands;
use bevy::{window::*, time, prelude::*};
use bevy::{sprite::MaterialMesh2dBundle};
use database::database::{receive_log, get_idf};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{self, Rng};
use redis::{Client, JsonAsyncCommands, ErrorKind, RedisError, RedisResult};

// // const player_id: i32 = rand::thread_rng().gen_range(1000..9999);

// //TODO: implement delete obsolete bullets
// //TODO: Find way to efficiently store data and keep state

// #[derive(Component)]
// struct Idf;

// #[derive(Component)]
// struct Number(i32);

// #[derive(Component)]
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// #[derive(Component)]
// struct Character;

// #[derive(Component)]
// struct Location(f32, f32);

// #[derive(Component)]
// struct Projectile;

// #[derive(Component)]
// struct Bullet {
//     location: Vec3,
//     time: f64,
//     direction: Direction,
// }

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) -> redis::RedisResult<()>{
//     commands.spawn(Camera2dBundle::default());
//     // setup_t();
//     let idf = get_idf()?;
//     commands.spawn((Number(idf), Idf));
//     let location = Location(0.0, 0.0);
//     add_my_player(commands, meshes, materials, location);
//     println!("{}", get_unix_time());
//     Ok(())
// }

fn get_unix_time() -> f64 {
    // Get the current time
    let current_time = SystemTime::now();

    // Calculate the Unix time
    let unix_time = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Extract the number of seconds since the Unix epoch
    unix_time.as_secs_f64()
}

// fn add_my_player(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     location: Location,
// ) {
//     commands.spawn((
//         MaterialMesh2dBundle {
//             mesh: meshes.add(shape::RegularPolygon::new(20., 3).into()).into(),
//             material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
//             transform: Transform::from_translation(Vec3::new(location.0, location.1, 0.)),

//             ..default()
//         },
//         Character,
//     ));
// }

// fn register_and_send_move(
//     time: Res<Time>,
//     mut player_position: Query<(&mut Character, &mut Transform)>,
//     keyboard_input: Res<Input<KeyCode>>,
// ) {
//     for (_, mut transform) in &mut player_position {
//         if keyboard_input.pressed(KeyCode::W) {
//             if !(transform.translation.y >= 280.0) {
//                 transform.translation.y += 150.0 * time.delta_seconds();
//             }
//             transform.rotation = Quat::default()
//         } else if keyboard_input.pressed(KeyCode::A) {
//             if !(transform.translation.x <= -380.0) {
//                 transform.translation.x -= 150.0 * time.delta_seconds();
//             }
//             transform.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)
//         } else if keyboard_input.pressed(KeyCode::S) {
//             if !(transform.translation.y <= -280.0) {
//                 transform.translation.y -= 150.0 * time.delta_seconds()
//             }
//             transform.rotation = Quat::from_rotation_z(std::f32::consts::PI)
//         } else if keyboard_input.pressed(KeyCode::D) {
//             if !(transform.translation.x >= 380.0) {
//                 transform.translation.x += 150.0 * time.delta_seconds()
//             }
//             transform.rotation = Quat::from_rotation_z(3.0 * std::f32::consts::FRAC_PI_2)
//         };
//     }
// }

// fn init_bullets(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut player_position: Query<(&mut Character, &mut Transform)>,
// ) {
//     if keyboard_input.pressed(KeyCode::Space) {
//         for (_, mut transform) in &mut player_position {
//             let mut dir = Direction::Up;
//             let mut x: f32 = transform.translation.x;
//             let mut y:f32 = transform.translation.y;
//             if transform.rotation == Quat::default(){
//                 dir = Direction::Up;
//                 y = transform.translation.y + 10.0;
//             }else if transform.rotation == Quat::from_rotation_z(std::f32::consts::FRAC_PI_2){
//                 dir = Direction::Left;
//                 x = transform.translation.x - 10.0;
//             }else if transform.rotation == Quat::from_rotation_z(std::f32::consts::PI){
//                 dir = Direction::Down;
//                 y = transform.translation.y - 10.0;
//             }else if transform.rotation == Quat::from_rotation_z(3.0 * std::f32::consts::FRAC_PI_2){
//                 dir = Direction::Right;
//                 x = transform.translation.x + 10.0;
//             }

//             commands.spawn((MaterialMesh2dBundle {
//                 mesh: meshes
//                     .add(shape::Quad::new(Vec2::new(3., 15.)).into())
//                     .into(),
//                 material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
//                 transform: Transform {
//                     translation: Vec3::new(x, y, -2.),
//                     rotation: transform.rotation,
//                     scale: Vec3::splat(1.0),
//                 },
//                 ..default()
//             }, Bullet{
//                 location: transform.translation,
//                 time: get_unix_time(),
//                 direction: dir,
//             }));
//         }
//     }
// }

// fn up_del_bullets(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut bullets: Query<(&mut Bullet, &mut Transform, Entity), With<Bullet>>,
// ) {
//     for (bullet, mut transform, entity) in &mut bullets {
//         match bullet.direction {
//             Direction::Up => transform.translation.y += ((get_unix_time() - bullet.time)+5.0) as f32,
//             Direction::Left => transform.translation.x -= ((get_unix_time() - bullet.time)+5.0) as f32,
//             Direction::Down => transform.translation.y -= ((get_unix_time() - bullet.time)+5.0) as f32,
//             Direction::Right => transform.translation.x += ((get_unix_time() - bullet.time)+5.0) as f32,
//         }
//         if (get_unix_time() - bullet.time) >= 5.0{
//             commands.entity(entity).despawn();
//         }
//     }


// }

// fn main() {
//     let window_plugin = WindowPlugin {
//         primary_window: Some(Window {
//             title: "X-Hunter".into(),
//             resolution: WindowResolution::new(800.0, 600.0),
//             resizable: false,
//             ..default()
//         }),
//         ..default()
//     };
//     App::new()
//         .add_plugins(DefaultPlugins.set(window_plugin))
//         .add_systems(Startup, setup)
//         .add_systems(Update, (register_and_send_move, init_bullets, up_del_bullets))
//         .run();
// }


use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct PrintNamesEvent {}

fn hello_world() {
    println!("hello world!");
}


fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
    commands.spawn((Person, Name("a".to_string())));

}

// fn greet_people(query: Query<&Name, With<Person>>) {
//     for name in &query {
//         println!("hello {}!", name.0);
//     }
// }

fn delete_people(
    query: Query<(&Name, Entity), With<Person>>,
    mut commands: Commands,
) {
    for (name, entity) in &query {

        if name.0 == "a".to_string() {
            commands.entity(entity).despawn();
        }
    }    
}



fn print_names(query: Query<(&Name, Entity), With<Person>>, mut commands: Commands,
){
    //Printing all the names again
    for (name, _) in &query {
        println!("hello {}!", name.0);
    }

}

fn main() {
    let window_plugin = WindowPlugin {
    primary_window: Some(Window {
        title: "X-Hunter".into(),
        resolution: WindowResolution::new(800.0, 600.0),
        resizable: false,
        ..default()
    }),
    ..default()
    };
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world,  print_names, delete_people, ))
        .run();
}