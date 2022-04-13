use std::panic::Location;

use bevy::prelude::*;
use bevy_pixelperfect::prelude::*;
use image::io::Reader;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelPerfectPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let img = Reader::open("examples/smile.png")
        .unwrap()
        .decode()
        .unwrap();

    commands.spawn_bundle((Camera, Position { x: 0, y: 0 }));

    commands.spawn_bundle((
        Position { x: 550, y: 250 },
        Scale(10),
        Image {
            data: img.to_rgba8(),
            width: img.width(),
            height: img.height(),
        },
    ));
    commands.spawn_bundle((
        Position { x: 0, y: 0 },
        Image {
            data: img.to_rgba8(),
            width: img.width(),
            height: img.height(),
        },
    ));
}
