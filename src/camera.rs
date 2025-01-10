use bevy::{prelude::*, window::PrimaryWindow};

pub const CAMERA_DOWN: KeyCode = KeyCode::KeyS;
pub const CAMERA_LEFT: KeyCode = KeyCode::KeyA;
pub const CAMERA_RIGHT: KeyCode = KeyCode::KeyD;
pub const CAMERA_UP: KeyCode = KeyCode::KeyW;
pub const CAMERA_ZOOM_IN: KeyCode = KeyCode::KeyE;
pub const CAMERA_ZOOM_OUT: KeyCode = KeyCode::KeyQ;
pub const CAMERA_SPEED_BOOST: KeyCode = KeyCode::ShiftLeft;

pub const CAMERA_PAN_SPEED: f32 = 500.0;
pub const CAMERA_ZOOM_SPEED: f32 = 0.01;
pub const CAMERA_ZOOM_MAX: f32 = 0.1;
pub const CAMERA_SPEED_BOOST_MULTIPLIER: f32 = 4.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (drag_to_move, keyboard_movement));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn drag_to_move(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&Camera, &mut Transform, &OrthographicProjection)>,
    mut last_pos: Local<Option<Vec2>>,
    time: Res<Time>,
) {
    // Access the primary window
    let window = if let Ok(window) = primary_window.get_single() {
        window
    } else {
        return;
    };

    // Check if the left mouse button is held
    if mouse_buttons.pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Some(last_cursor_pos) = *last_pos {
                let delta = cursor_pos - last_cursor_pos;
                for (_camera, mut transform, projection) in query.iter_mut() {
                    transform.translation.x -=
                        delta.x * time.delta_secs() * projection.scale * 100.0;
                    transform.translation.y +=
                        delta.y * time.delta_secs() * projection.scale * 100.0;
                }
            }
            *last_pos = Some(cursor_pos);
        }
    } else {
        *last_pos = None;
    }
}

fn keyboard_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        let mut speed_multiplier = 1.0;
        if keyboard_input.pressed(CAMERA_SPEED_BOOST) {
            speed_multiplier = CAMERA_SPEED_BOOST_MULTIPLIER;
        }

        if keyboard_input.pressed(CAMERA_LEFT) {
            direction -= Vec3::new(CAMERA_PAN_SPEED, 0.0, 0.0);
        }

        if keyboard_input.pressed(CAMERA_RIGHT) {
            direction += Vec3::new(CAMERA_PAN_SPEED, 0.0, 0.0);
        }

        if keyboard_input.pressed(CAMERA_UP) {
            direction += Vec3::new(0.0, CAMERA_PAN_SPEED, 0.0);
        }

        if keyboard_input.pressed(CAMERA_DOWN) {
            direction -= Vec3::new(0.0, CAMERA_PAN_SPEED, 0.0);
        }

        if keyboard_input.pressed(CAMERA_ZOOM_IN) {
            ortho.scale -= CAMERA_ZOOM_SPEED * speed_multiplier;
        }

        if keyboard_input.pressed(CAMERA_ZOOM_OUT) {
            ortho.scale += CAMERA_ZOOM_SPEED * speed_multiplier;
        }

        if ortho.scale < CAMERA_ZOOM_MAX {
            ortho.scale = CAMERA_ZOOM_MAX;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_secs() * direction * speed_multiplier;

        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
