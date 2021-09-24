use bevy::prelude::*;
use rand::Rng;
use bevy::input::mouse::MouseButtonInput;
use bevy::app::Events;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Minesweeper".to_string(),
            ..Default::default()
        })
        .insert_resource(Start { bombs: 10, tiles_x: 5, tiles_y: 5, bounds: Vec2::new(500.0, 500.0) })
        .add_startup_system(startup.system())
        .add_system(handle_mouse_clicks.system())
        .run();
}

struct Tile {
    bomb:bool,
    index:i32,
    shown:bool,
}

struct Start {
    bombs:i32,
    tiles_x:i32,
    tiles_y:i32,
    bounds:Vec2,
}

fn startup(mut commands: Commands, start: Res<Start>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut rng = rand::thread_rng();
    let bounds = start.bounds;
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    let mut index = 0;
    let mut bombs = 0;
    for x in 0..start.tiles_x {
        for y in 0..start.tiles_y {
            let mut bomb = ((start.bombs-bombs) as f32 / ((start.tiles_y*start.tiles_x)-index) as f32) > rng.gen::<f32>();
            if bomb {
                bombs += 1;
            }
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(0.0 + (((x+y)%2) as f32 * 255.0),0.0 + (((x+y)%2) as f32 * 255.0),0.0 + (((x+y)%2) as f32 * 255.0)).into()),
                    transform: Transform::from_xyz((bounds.x / 2.0) - (x as f32 * (bounds.x / start.tiles_x as f32)) - ((bounds.x/start.tiles_x as f32)/2.0), (bounds.y / 2.0) - (y as f32 * (bounds.y / start.tiles_y as f32)) - ((bounds.y/start.tiles_y as f32)/2.0), 0.0),
                    sprite: Sprite::new(Vec2::new(bounds.x/start.tiles_x as f32, bounds.y/start.tiles_y as f32)),
                    ..Default::default()
                })
                .insert(Tile { index: index, bomb: bomb, shown:false });
            index += 1;
        }
    }
}

fn handle_mouse_clicks(mouse_input: Res<Input<MouseButton>>, windows: Res<Windows>, start: Res<Start>, mut query: Query<(&mut Tile)>) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) || mouse_input.just_pressed(MouseButton::Right) {
        let pressed_left =  mouse_input.just_pressed(MouseButton::Left);
        let pos = Vec2::new(((win.cursor_position().unwrap().x - win.width()/2.0 + (start.bounds.x / 2.0))/(start.bounds.x / start.tiles_x as f32)).floor(), ((win.cursor_position().unwrap().y - win.height()/2.0 + (start.bounds.y / 2.0))/(start.bounds.y / start.tiles_y as f32)).floor());
        let index = ((start.tiles_y) as f32 - pos.y -1.0) + (start.tiles_y as f32 * ((start.tiles_x) as f32 - pos.x - 1.0));
        for mut x in query.iter_mut() {
            if x.index as f32 == index && !x.shown {
                if pressed_left {
                    let shown =  &mut x.shown;
                    if *shown {
                        *shown = false;
                    }
                    else {
                        *shown = true;
                    }
                    println!("{}", index);
                }
            }
        }
    }
}


