use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::app::Events;
use bevy::reflect::List;
use bevy::ui::Val::Px;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Sand".to_string(),
            ..Default::default()
        })
        .insert_resource(Poses { pos: Vec::new() })
        .add_startup_system(startup.system())
        .add_system(handle_mouse_clicks.system())
        .add_system(update.system())
        .run();
}

struct Sand {
    velocity: f32,
}

struct Poses {
    pos: Vec<(f32, f32)>,
}

fn startup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn handle_mouse_clicks(mouse_input: Res<Input<MouseButton>>, windows: ResMut<Windows>, mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut poses: ResMut<Poses>,) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.pressed(MouseButton::Left) {
        let pos = ((win.cursor_position().unwrap().x - win.width()/2.0).round(), (win.cursor_position().unwrap().y - win.height()/2.0).round());
        let sand = Sand { velocity: 0.0};
        commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(pos.0, pos.1, 0.0),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
            .insert(sand);
        poses.pos.push(pos);
    }
}

fn update(mut query: Query<(&mut Sand, &mut Transform)>, windows: ResMut<Windows>, time: Res<Time>, mut poses: ResMut<Poses>) {
    let win = windows.get_primary().expect("no primary window");
    let mut poseses = Vec::new();
    for mut i in query.iter_mut() {
        let mut reached = false;
        let mut new_position = 0.0;
        let old = (i.1.translation.x, i.1.translation.y);
        i.0.velocity = i.0.velocity + (9.8 * time.delta_seconds());
        for x in &poses.pos {
            if reached == false && old != *x && old.0 == x.0 && old.1-1.0 >= x.1 && old.1-1.0 - i.0.velocity < x.1 {
                reached = true;
                new_position = x.1+1.0;
                i.0.velocity = 0.0;
            }
        }
        if reached == false {
            if i.1.translation.y - i.0.velocity > -(win.height()/2.0)+1.0 {
                i.1.translation.y -= i.0.velocity;
            }
            else {
                i.1.translation.y = -(win.height()/2.0)+1.0;
                i.0.velocity = 0.0;
            }
        }
        else {
            i.1.translation.y = new_position;
        }
        let new_pos = (i.1.translation.x, i.1.translation.y);
        poseses.push(new_pos);
    }
    poses.pos = poseses;
}