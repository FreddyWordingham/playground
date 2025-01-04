use bevy::{prelude::*, window::PrimaryWindow};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, drag_to_move);
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
