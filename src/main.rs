use bevy::{
    prelude::*,
    transform::components::Transform,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
};
use bevy::ecs::event::{Events, ManualEventReader};

use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputButton;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // This plugin maps inputs to an input-type agnostic action-state
        // We need to provide it with an enum which stores the possible actions a player could take
        .add_plugin(InputManagerPlugin::<Action>::default())
        .init_resource::<Direction>()
        .init_resource::<InputState>()
        // The InputMap and ActionState components will be added to any entity with the Player component
        .add_startup_system(spawn_player)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_light)
        .add_startup_system(cursor_grab_system)
        // .add_startup_system(initial_grab_cursor)
        // Read the ActionState in your systems using queries!
        // .add_system(player_input)
        // .add_system(apply_velocity)
        .add_system(update_directional_input)
        .add_system(move_player)
        .add_system(mouse_motion)
        .run();
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Thrust,
    Activate,
    Weapon1,
    Weapon2,
    Forward,
    Reverse,
    StrafeLeft,
    StrafeRight,
    // ReleaseMouse,
}

// Define a resource for the current movement direction;
#[derive(Default)]
struct Direction(Vec3);

// Define a marker for entities that should move.
#[derive(Component)]
struct Move;

#[derive(Component)]
struct Camera;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    velocity: Velocity,
    #[bundle]
    input_manager: InputManagerBundle<Action>,
    #[bundle]
    model: PbrBundle,
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<Action> {
        use Action::*;

        InputMap::new([
            (StrafeLeft, InputButton::Keyboard(KeyCode::Left)),
            (StrafeLeft, InputButton::Keyboard(KeyCode::A)),
            (StrafeRight, InputButton::Keyboard(KeyCode::Right)),
            (StrafeRight, InputButton::Keyboard(KeyCode::D)),
            (Thrust, InputButton::Keyboard(KeyCode::LShift)),
            (Weapon1, InputButton::Mouse(MouseButton::Left)),
            (Weapon2, InputButton::Mouse(MouseButton::Right)),
            (Activate, InputButton::Keyboard(KeyCode::Space)),
            (Forward, InputButton::Keyboard(KeyCode::W)),
            (Reverse, InputButton::Keyboard(KeyCode::S)),
            // (ReleaseMouse, InputButton::Keyboard(KeyCode::Escape)),
        ])
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(PlayerBundle {
        player: Player {},
        // player: Player { x: 0., y: 0., z: 0. },
        velocity: Velocity { x: 0.0, y: 0.0, z: 0.0 },
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            action_state: ActionState::default(),
        },
        model: PbrBundle {
            mesh: asset_server.load("models/ships/starter.glb#Mesh0/Primitive0"),
            // material: material_handle.clone(),
            transform: Transform {
                scale: Vec3::new(1.0, 1.0, 0.0),
                ..Default::default()
            },
            ..default()
        }
    });
    // .insert(Move);

}

// Query for the `ActionState` component in your game logic systems!
fn player_input(
    mut player_query: Query<(&ActionState<Action>, &mut Velocity), With<Player>>,
) {

    const VELOCITY_RATIO: f32 = 1000.0;
    let (action_state, mut velocity) = player_query.single_mut();

    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(Action::Weapon1) {
        println!("I'm shooting!");
    }
    if action_state.just_pressed(Action::Weapon2) {
        println!("Missiles!!!!");
    }
    if action_state.just_pressed(Action::Thrust) {
        println!("I'm thrusting!");
    }
    if action_state.just_pressed(Action::Activate) {
        println!("Activate");
    }
    if action_state.just_pressed(Action::Forward) {
        println!("Forward");
    }
    if action_state.just_pressed(Action::Reverse) {
        println!("Reverse");
    }
    if action_state.just_released(Action::StrafeLeft) {
        println!("Strafe Left");
    }
    if action_state.just_pressed(Action::StrafeRight) {
        println!("Strafe Right");
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
        transform.translation.z += velocity.z * time.delta_seconds();
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        .insert(Move)
        .insert(Camera);
}

fn setup_light(mut commands: Commands,
               mut meshes: ResMut<Assets<Mesh>>,
               mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a light source for better 3d visibility.
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::splat(3.0)),
        ..Default::default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..default()
    });
}

fn move_player(
    mut player: Query<&mut Transform, With<Move>>,
    direction: Res<Direction>,
    timer: Res<Time>,
) {
    for mut transform in player.iter_mut() {
        transform.translation += direction.0 * timer.delta_seconds();
    }
}

// This system updates a resource that defines in which direction the cubes should move.
// The direction is defined by the input of arrow keys and is only in left/right and up/down direction.
// fn update_directional_input(mut direction: ResMut<Direction>, mut query: Query<&ActionState<Action>, With<Player>>) {
fn update_directional_input(
    mut query: Query<(&ActionState<Action>, &mut Transform, With<Player>)>,
    // mut transform: Query<&mut Transform, With<Move>>,
    // mut query: Query<&ActionState<Action>, With<Player>>
) {

    // let (action_state, mut transform) = query.single_mut();

    for (action_state, mut transform, player) in query.iter_mut() {
        let forward_movement = Vec3::Z
            * (action_state.just_released(Action::Forward) as i32
            - action_state.just_released(Action::Reverse) as i32) as f32;
        let rotation = transform.rotation;
        let rotated_forward_movement = rotation.mul_vec3(forward_movement);
        transform.translation += rotated_forward_movement;

        // let horizontal_movement = Vec3::X
        //     * (action_state.just_released(Action::StrafeRight) as i32
        //     - action_state.just_released(Action::StrafeLeft) as i32) as f32;
        // let vertical_movement = Vec3::Y
        //     * (action_state.just_released(Action::Forward) as i32
        //     - action_state.just_released(Action::Reverse) as i32) as f32;
        // direction.0 = horizontal_movement + vertical_movement;
        // println!("{:?}", direction.0);

        // let forward_movement = Vec3::new(0.0, 0.0, -1.0);
    }
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}

fn mouse_motion(
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<(&mut Transform, With<Camera>)>,
) {
    let window = windows.get_primary().unwrap();
    let mut delta_state = state.as_mut();

    for (mut transform, camera) in query.iter_mut() {
        for ev in delta_state.reader_motion.iter(&motion) {
            println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);

            // let horizontal_movement = Vec3::X + ((ev.delta.x as i32) as f32 * 0.001);
            // let horizontal_rotation = transform.rotation;
            // let rotated_horizontal_movement = horizontal_rotation.mul_vec3(horizontal_movement);
            // transform.translation += horizontal_movement;
            //
            // let vertical_movement = Vec3::Y + ((ev.delta.y as i32) as f32 * 0.001);
            // let vertical_rotation = transform.rotation;
            // let rotated_vertical_movement = vertical_rotation.mul_vec3(vertical_movement);
            // transform.translation += vertical_movement;
            // let window_scale = window.height().min(window.width());
            // delta_state.pitch -=
            //     (0.01 * ev.delta.y * window_scale).to_radians();
            // delta_state.yaw -= (0.01 * ev.delta.x * window_scale).to_radians();
            //
            // delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);
            let window_scale = window.height().min(window.width());
            delta_state.pitch -=
                (0.01 * ev.delta.y * window_scale).to_radians();
            delta_state.yaw -= (0.01 * ev.delta.x * window_scale).to_radians();
            delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

            let delta_x = ev.delta.x as f32 * 0.001;
            let delta_y = ev.delta.y as f32 * 0.001;
            transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
        }
    }
}