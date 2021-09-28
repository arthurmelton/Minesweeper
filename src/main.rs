use bevy::prelude::*;
use rand::Rng;
use bevy::input::mouse::MouseButtonInput;
use bevy::app::Events;
use bevy::ui::Val::Px;
use std::borrow::{Borrow, BorrowMut};

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
        .add_system(text_system.system())
        .add_system(img_place.system())
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

struct Img {
    index:i32,
}

fn startup(mut commands: Commands, start: Res<Start>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let bounds = start.bounds;
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    let mut index = 0;
    let mut bombs = 0;
    for x in 0..start.tiles_x {
        for y in 0..start.tiles_y {
            let bomb = ((start.bombs-bombs) as f32 / ((start.tiles_y*start.tiles_x)-index) as f32) > rng.gen::<f32>();
            if bomb {
                bombs += 1;
            }
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(asset_server.load("images/facingDown.png").into()),
                    transform: Transform::from_xyz((bounds.x / 2.0) - (x as f32 * (bounds.x / start.tiles_x as f32)) - ((bounds.x/start.tiles_x as f32)/2.0), (bounds.y / 2.0) - (y as f32 * (bounds.y / start.tiles_y as f32)) - ((bounds.y/start.tiles_y as f32)/2.0), 0.0),
                    sprite: Sprite::new(Vec2::new(bounds.x/start.tiles_x as f32, bounds.y/start.tiles_y as f32)),
                    ..Default::default()
                })
                .insert(Tile { index: index, bomb: bomb, shown:false });
            index += 1;
        }
    }
}

fn handle_mouse_clicks(mouse_input: Res<Input<MouseButton>>, mut windows: ResMut<Windows>, start: Res<Start>, mut query: Query<(&mut Tile, Entity)>, mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>,) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) || mouse_input.just_pressed(MouseButton::Right) {
        let pressed_left =  mouse_input.just_pressed(MouseButton::Left);
        let pos = Vec2::new(((win.cursor_position().unwrap().x - win.width()/2.0 + (start.bounds.x / 2.0))/(start.bounds.x / start.tiles_x as f32)).floor(), ((win.cursor_position().unwrap().y - win.height()/2.0 + (start.bounds.y / 2.0))/(start.bounds.y / start.tiles_y as f32)).floor());
        let index = ((start.tiles_y) as f32 - pos.y -1.0) + (start.tiles_y as f32 * ((start.tiles_x) as f32 - pos.x - 1.0));
        let mut query_var = query.iter_mut();
        println!("{}", index);
        let query_var_var = query_var.borrow_mut().nth(index as usize).unwrap();
        let x = query_var_var.0;
        let y = query_var_var.1;
        if !x.shown && pressed_left && !x.bomb {
            pressed_left_fn(commands, windows, index as i32, y, start, materials, query, asset_server);
        }
        else if !x.shown && pressed_left && x.bomb {
            let window = windows.get_primary_mut().unwrap();
            commands.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "You Lost".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                            },
                        },
                    ],
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Px(0.0),
                        left: Px(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                node: Node {
                    ..Default::default()
                },
                ..Default::default()
            });
        }
        else if !pressed_left && !x.shown {
            let yy = index%(start.tiles_x as f32);
            let xx = (index as f32/start.tiles_x as f32).floor();
            let bounds = start.bounds;
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(asset_server.load("images/flag.png").into()),
                    transform: Transform::from_xyz((bounds.x / 2.0) - (xx as f32 * (bounds.x / start.tiles_x as f32)) - ((bounds.x / start.tiles_x as f32) / 2.0), (bounds.y / 2.0) - (yy as f32 * (bounds.y / start.tiles_y as f32)) - ((bounds.y / start.tiles_y as f32) / 2.0), 1.0),
                    sprite: Sprite::new(Vec2::new(bounds.x / start.tiles_x as f32, bounds.y / start.tiles_y as f32)),
                    ..Default::default()
                });
        }
    }
}

fn text_system(mut windows: ResMut<Windows>, mut query: Query<(&mut Style, &mut Node, &Text)>) {
    let mut query_var = query.iter_mut();
    for mut x in query_var.borrow_mut() {
        if x.borrow_mut().2.sections[0].value == format!("You Lost") {
            let window = windows.get_primary_mut().unwrap();
            x.borrow_mut().0.position.top = Px((window.height() / 2.0) - (x.borrow_mut().1.size.y / 2.0));
            x.borrow_mut().0.position.left = Px((window.width() / 2.0) - (x.borrow_mut().1.size.x / 2.0));
        }
    }
}

fn img_place(mut windows: ResMut<Windows>, mut query: Query<(&mut Style, &mut Node, &Img)>, start: Res<Start>) {
    for mut x in query.iter_mut() {
        let bounds = start.bounds;
        let yy = (start.tiles_y - 1) - (x.2.index%start.tiles_x);
        let xx = (x.2.index as f32/start.tiles_x as f32).floor();
        let window = windows.get_primary_mut().unwrap();
        x.0.position.left = Px(((bounds.x / 2.0) - (xx as f32 * (bounds.x / start.tiles_x as f32)) - ((bounds.x / start.tiles_x as f32) / 2.0) + window.width() / 2.0) - x.1.size.x / 2.0);
        x.0.position.top = Px(((bounds.y / 2.0) - (yy as f32 * (bounds.y / start.tiles_y as f32)) - ((bounds.y / start.tiles_y as f32) / 2.0) + window.height() / 2.0) - x.1.size.y / 2.0);
    }
}

fn pressed_left_fn(mut commands:Commands, mut windows:ResMut<Windows>, index: i32, y: bevy::prelude::Entity, start:Res<Start>, mut materials: ResMut<Assets<ColorMaterial>>, mut query: Query<(&mut Tile, Entity)>, asset_server: Res<AssetServer>) {
    let mut query_var = query.iter_mut();
    let yy = index%start.tiles_x;
    let xx = (index as f32/start.tiles_x as f32).floor();
    let bounds = start.bounds;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load("images/empty.png").into()),
            transform: Transform::from_xyz((bounds.x / 2.0) - (xx as f32 * (bounds.x / start.tiles_x as f32)) - ((bounds.x / start.tiles_x as f32) / 2.0), (bounds.y / 2.0) - (yy as f32 * (bounds.y / start.tiles_y as f32)) - ((bounds.y / start.tiles_y as f32) / 2.0), 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x / start.tiles_x as f32, bounds.y / start.tiles_y as f32)),
            ..Default::default()
        })
        .insert(Tile { index: index, bomb: false, shown: true });
    let window = windows.get_primary_mut().unwrap();
    let mut number_of_bombs_near = 0;
    for xxx in query_var.borrow_mut() {
        if xxx.0.index == index - start.tiles_y && index - start.tiles_y >= 0 && index - start.tiles_y < start.tiles_x * start.tiles_y && xxx.0.bomb {
            number_of_bombs_near += 1;
        }
        if xxx.0.index == index + start.tiles_y && index + start.tiles_y >= 0 && index + start.tiles_y < start.tiles_x * start.tiles_y && xxx.0.bomb {
            number_of_bombs_near += 1;
        }
        if xxx.0.index == index + 1 && (((index + 1)/start.tiles_y) as f32).floor() == ((index/start.tiles_y) as f32).floor() && xxx.0.bomb {
            number_of_bombs_near += 1;
        }
        if xxx.0.index == index - 1 && (((index - 1)/start.tiles_y) as f32).floor() == ((index/start.tiles_y) as f32).floor() && xxx.0.bomb {
            number_of_bombs_near += 1;
        }
        if xxx.0.index == index + start.tiles_y + 1 &&  (((index + start.tiles_y + 1)/start.tiles_y - 1) as f32).floor() == ((index/start.tiles_y) as f32).floor() && xxx.0.bomb {
            number_of_bombs_near += 1;
        }
        if xxx.0.index == index + start.tiles_y - 1&& (((index + start.tiles_y - 1)/start.tiles_y - 1) as f32).floor() == ((index/start.tiles_y) as f32).floor() && xxx.0.bomb {
            number_of_bombs_near += 1;
        }
        if xxx.0.index == index - start.tiles_y + 1 && (((index - start.tiles_y + 1)/start.tiles_y + 1) as f32).floor() == ((index/start.tiles_y) as f32).floor() && xxx.0.bomb {
            number_of_bombs_near += 1;
        }
        if xxx.0.index == index - start.tiles_y - 1 && (((index - start.tiles_y - 1)/start.tiles_y + 1) as f32).floor() == ((index/start.tiles_y) as f32).floor() && xxx.0.bomb {
            number_of_bombs_near += 1;
        }
    }
    if number_of_bombs_near != 0 {
        commands.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: number_of_bombs_near.to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1.0, 0.0, 0.0),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Px(0.0),
                    left: Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            node: Node {
                ..Default::default()
            },
            ..Default::default()
        }).insert(Img { index: index });
    }
    commands.entity(y).despawn();
}