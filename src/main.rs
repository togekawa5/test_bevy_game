use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin{
                    primary_window : Some(Window{
                        title: "Bevy Rapier 2D test title".to_string(),
                        name: Some("Bevy Rapier 2D test name".to_string()),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            )
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_spawn_ball_timer)
        .add_systems(Update, spawn_ball)
        .add_systems(Startup, setup_rotating_bar)
        .add_systems(Update, rotate_bars)
        .run();
}

fn setup
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut asset_server: ResMut<AssetServer>,
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

fn setup_spawn_ball_timer
(
    mut commands: Commands,
) {
    commands.spawn((
        SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
    ));
}

#[derive(Component)]
struct SpawnTimer(Timer);

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: Query<&mut SpawnTimer>,
    
) {
    for mut spawn_timer in timer.iter_mut() {
        //spawn_timer.0.tick(time.delta());
        if spawn_timer.0.tick(time.delta()).just_finished() {
            let shape = meshes.add(Circle::new(50.0));
            let material = materials.add(Color::srgb(0.8, 0.5, 0.3));

            let random_position = Vec3::new(
                random_01() * WINDOW_WIDTH - WINDOW_WIDTH / 2.0,
                random_01() * WINDOW_HEIGHT - WINDOW_HEIGHT / 2.0,
                0.0,
            );
            commands.spawn((
                Mesh2d(shape),
                MeshMaterial2d(material),
                Transform::from_xyz(random_position.x, random_position.y, 0.0),
                RigidBody::Dynamic,
                //GravityScale(5.0),
                Collider::ball(50.0),
            ));
        }
    }
    
}

fn random_01() -> f32 {
    let mut rng = rand::rng();
    let rand_val : i32 = rng.random::<i32>();
    return rand_val as f32 / i32::MAX as f32;
}

#[derive(Component)]
struct Rotating{
    angular_speed: f32,
}

fn setup_rotating_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bar_shape = meshes.add(Rectangle::new(100.0, 20.0));
    let bar_material = materials.add(Color::srgb(0.4, 0.7, 0.8));
    commands.spawn((
        Mesh2d(bar_shape),
        MeshMaterial2d(bar_material),
        Transform::from_xyz(0.0, 90.0, 0.0),
        RigidBody::Dynamic,
        //ColliderMassProperties::Mass(100),
        Collider::cuboid(100.0, 20.0),
        Rotating{angular_speed: 30000.0},
        GravityScale(0.0),
        LockedAxes::TRANSLATION_LOCKED,
        ExternalImpulse{
            torque_impulse: 14.0,
            impulse: Vec2::new(50.0, 0.0),
        },
        ExternalForce {
            force: Vec2::ZERO,
            torque: 0.,
        },
        
    ));
}

fn rotate_bars(
    //time: Res<Time>,
    mut query: Query<(&Rotating, &mut ExternalImpulse)>,
) {
    for (rotating, mut torque) in query.iter_mut() {
        //println!("{}", torque.torque_impulse.to_string());
        torque.torque_impulse = rotating.angular_speed;
        torque.impulse = Vec2::new(0.0, 0.0);
        
        let torque_2 = ExternalImpulse::at_point(Vec2::new(rotating.angular_speed, 0.0), Vec2::new(100.0, 0.0), Vec2::new(0.0, 0.0));
        println!("inpulse: {}, torque: {}", torque_2.impulse.to_string(), torque_2.torque_impulse.to_string());
    }
}