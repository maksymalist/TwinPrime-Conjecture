use std::collections::HashMap;
use bevy::prelude::*;
use std::ops::Range;

mod utils;
use utils::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

const SCROLL_SPEED: f32 = 10.0;

const ZOOM: f32 = 0.12;
const TILE_SIZE: f32 = 1.0;

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}

enum TwinType {
    None,
    FirstPrime,
    LastPrime
}

fn is_twin_prime(n: i32) -> TwinType {
    if is_prime(n) && is_prime(n + 2) {
        return TwinType::FirstPrime;
    } else if is_prime(n) && is_prime(n - 2) {
        return TwinType::LastPrime;
    }

    TwinType::None
}

fn main () {

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH,
                height: SCREEN_WIDTH,
                title: "Bevy 3d playground".to_string(),
                resizable: true,
                ..Default::default()
            },
            ..default()
        }))
        .add_startup_system(spawn_tiles)
        .add_startup_system(spawn_camera)
        .add_system(keyboard_movement)
        .run();
}


fn spawn_tiles(mut commands: Commands) {
    let n: i32 = 10000000;
    const START: i32 = 6;
    let mut last_prime: i32 = 1;
    let mut steps_between_twin_primes: Vec<(Range<i32>, i32)> = vec![(2..3, 1) , (3..5, 2)];
    let mut largest_gaps: Vec<(Range<i32>, i32)> = vec![(0..0, 0)];

    for i in START..n+1 {

        match is_twin_prime(i) {
            TwinType::FirstPrime => {
                if let Some((range, largest_gap)) = largest_gaps.last() {
                    if i - last_prime > *largest_gap {
                        largest_gaps.push((last_prime..i, i-last_prime));
                    }
                }
                steps_between_twin_primes.push((last_prime..i, i-last_prime));
            },
            TwinType::LastPrime => {
                last_prime = i
            },
            TwinType::None => {}
        }
    }

    // print the largest gap between twin primes
    println!("last prime: {:?} largest = ", steps_between_twin_primes.last().unwrap().1);
    // print all the largest gap i32
    let mut tmp = Vec::new();
    let mut steps: Vec<i32> = Vec::new();
    let mut last_step = 0;
    for (range, largest_gap) in largest_gaps {
        println!("range: {:?} largest = {}", range, largest_gap);
        tmp.push(largest_gap);
        steps.push(largest_gap - last_step);
        last_step = largest_gap;
    }

    println!("{:?}", tmp);

    println!("{:?}", steps);

    for (x, y) in steps_between_twin_primes.into_iter().enumerate() {
        let absice = (x as f32 * ZOOM) - SCREEN_WIDTH / 2.0;
        let ordinate = (y.1 as f32 * ZOOM) - SCREEN_HEIGHT / 2.0;
        commands.spawn(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(absice, ordinate, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }
}

fn keyboard_movement (
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>
) {
    
    for mut transform in camera.iter_mut() {
        
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= SCROLL_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += SCROLL_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += SCROLL_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= SCROLL_SPEED;
        }
    }

}
    

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }
    );
}
