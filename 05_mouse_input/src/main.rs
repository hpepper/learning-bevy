// Just window, load a sprite.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(react_to_mouse_click)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn react_to_mouse_click(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let position = get_mouse_cursor_position(&windows);
        println!("Left button was pressed, at position: {}", position);
    }
    if buttons.just_released(MouseButton::Left) {
        let position = get_mouse_cursor_position(&windows);
        println!("Left button was RELEASED, at position: {}", position);
    }
    if buttons.pressed(MouseButton::Right) {
        let position = get_mouse_cursor_position(&windows);
        println!("Right Button is being held down at {}", position);
    }
}

/**
 * Returns 0,0 if the window or position can't be found.
 */
fn get_mouse_cursor_position(
    windows: &Res<Windows>,
) -> Vec2 {
    match windows.get_primary() {
        Some(window) => 
          match window.cursor_position() {
            Some(position) => position,
            None => Vec2::new(0.0, 0.0),
          },
        None => Vec2::new(0.0, 0.0),
    }
}
