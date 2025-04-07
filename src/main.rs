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
        //.add_systems(Startup, setup)
        .run();
}