use bevy::prelude::*;
use bevy_aseprite::{AsepriteBundle, AsepritePlugin};

mod camera;
mod fella;
mod picking;
mod time;
mod ui;
mod world;

use camera::MainCamera;
use fella::FellaPlugin;
use picking::MyPickingPlugin;
use time::{advance_time, update_simulation_time, SimulationDeltaTime, SimulationTime, TimeScale};
use ui::GameUiPlugin;
use world::WorldPosition;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AsepritePlugin)
        //.add_plugins(DefaultPickingPlugins)
        .add_plugins((GameUiPlugin, FellaPlugin, MyPickingPlugin))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(SimulationTime::default())
        .insert_resource(TimeScale::Normal)
        .insert_resource(SimulationDeltaTime(None))
        .add_systems(Update, update_simulation_time)
        .add_systems(
            Update,
            (advance_time, world_position_to_transform).after(update_simulation_time),
        )
        .run();
}

mod sprites {
    use bevy_aseprite::aseprite;

    aseprite!(pub Fella01, "gfx/fella01.aseprite");
    aseprite!(pub Fella02, "gfx/fella02.aseprite");

    aseprite!(pub Bed, "gfx/bed.aseprite");
    aseprite!(pub Coffee, "gfx/coffee.aseprite");
    aseprite!(pub Hamburger, "gfx/hamburger.aseprite");
    aseprite!(pub Toilet, "gfx/toilet.aseprite");
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load(sprites::Hamburger::PATH),
        transform: Transform {
            scale: Vec3::splat(2.0),
            translation: Vec3::new(0.0, 128.0, 0.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load(sprites::Coffee::PATH),
        transform: Transform {
            scale: Vec3::splat(2.0),
            translation: Vec3::new(-128.0, 128.0, 0.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load(sprites::Bed::PATH),
        transform: Transform {
            scale: Vec3::splat(2.0),
            translation: Vec3::new(128.0, 128.0, 0.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load(sprites::Toilet::PATH),
        transform: Transform {
            scale: Vec3::splat(2.0),
            translation: Vec3::new(256.0, 128.0, 0.0),
            ..default()
        },
        ..default()
    });

    fella::create_fella(
        &mut commands,
        "Felix Fella",
        sprites::Fella01::PATH,
        Vec2::new(0.0, 0.0),
        asset_server.as_ref(),
    );
    fella::create_fella(
        &mut commands,
        "Fiona Fella",
        sprites::Fella02::PATH,
        Vec2::new(1.0, 0.0),
        asset_server.as_ref(),
    );
}

fn world_position_to_transform(mut query: Query<(&mut Transform, &WorldPosition)>) {
    for (mut transform, world_position) in query.iter_mut() {
        transform.translation =
            Vec3::new(world_position.0.x * 64.0, world_position.0.y * 64.0, 0.0);
    }
}
