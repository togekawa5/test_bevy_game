use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin{
                    primary_window : Some(Window{
                        title: "Bevy Rapier 2D test title".to_string(),
                        name: Some("Bevy Rapier 2D test name".to_string()),
                        resolution: (800., 600.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            )
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2d::default());

    let shape = meshes.add(Circle::new(50.0));
    let material = materials.add(Color::srgb(0.8, 0.5, 0.3));
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Dynamic,
        Collider::ball(50.0),
    ));

    let bar_shape = meshes.add(Rectangle::new(100.0, 20.0));
    let bar_material = materials.add(Color::srgb(0.9, 0.7, 0.8));
    commands.spawn((
        Mesh2d(bar_shape),
        MeshMaterial2d(bar_material),
        Transform::from_xyz(0.0, -80.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(100.0, 20.0),
    ));
}