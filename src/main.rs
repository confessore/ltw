use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{
        collide,
        Collision
    }
};

use ltwlib::{
    Collider,
    Unit
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_cameras.system())
        .run();
}

fn init_cameras(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>) {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        commands.spawn_bundle(UiCameraBundle::default());
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            sprite: Sprite::new(Vec2::new(64.0, 64.0)),
            ..Default::default()
        })
        .insert(Unit { speed: 500.0 })
        .insert(Collider::Unit);
}

fn walls(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>) {
        let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
        let wall_thickness = 10.0;
        let bounds = Vec2::new(900.0, 600.0);
        commands.spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Unit);
        commands.spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Unit);
        commands.spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Unit);
        commands.spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Unit);
}
