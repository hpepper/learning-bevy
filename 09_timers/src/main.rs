// Just window, load a sprite.

use bevy::prelude::*;
use std::time::Duration;

const DISPLAY_TEXT_EVERY_GIVEN_SECONDS: u64 = 2;

#[derive(Resource)]
struct TextDisplayTimer {
    /// How often to show the text (repeating timer)
    show_text_timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(timer_operations)
        .run();
}

fn timer_operations(time: Res<Time>, mut text_timer_resource: ResMut<TextDisplayTimer>) {
    // tick the timer
    text_timer_resource.show_text_timer.tick(time.delta());
    // info!("DDD time: {:?}", time.startup()); // this show the instance time, when the game was started.
    // This will show all ticks within e.g. 10s: if time.elapsed_seconds() as i64 % 2 == 0 {
    if text_timer_resource.show_text_timer.finished() {
        info!("DDD time: {}", time.elapsed_seconds());
    }
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(TextDisplayTimer {
        // create the repeating timer
        show_text_timer: Timer::new(Duration::from_secs(DISPLAY_TEXT_EVERY_GIVEN_SECONDS), TimerMode::Repeating),
    })
}