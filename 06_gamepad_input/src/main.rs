// Just window, load a sprite.

use bevy::{
    input::gamepad::{GamepadEvent, GamepadEventType},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(gamepad_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn gamepad_events(mut gamepad_event: EventReader<GamepadEvent>) {
    // go through all gamepad events.
    // GamepadEvent - https://docs.rs/bevy/latest/bevy/input/prelude/struct.GamepadEvent.html
    for event in gamepad_event.iter() {
        // GamepadEventType - https://docs.rs/bevy/latest/bevy/input/prelude/enum.GamepadEventType.html
        match &event.event_type {
            GamepadEventType::Connected(gamepad_info) => {
                info!("{} {} Connected", event.gamepad.id, gamepad_info.name);
            }
            GamepadEventType::Disconnected => {
                info!("{} Disconnected", event.gamepad.id);
            }
            // GamepadButtonType - https://docs.rs/bevy/latest/bevy/input/gamepad/enum.GamepadButtonType.html
            GamepadEventType::ButtonChanged(button_type, value) => {
                info!(
                    "{:?} of {} is changed to {}",
                    button_type, event.gamepad.id, value
                );
            }
            // GamepadAxisType - https://docs.rs/bevy/latest/bevy/input/gamepad/enum.GamepadAxisType.html
            GamepadEventType::AxisChanged(axis_type, value) => {
                info!(
                    "{:?} of id: {} is changed to {}",
                    axis_type, event.gamepad.id, value
                );
            }
        }
    }
}
