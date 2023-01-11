// Just window, load a sprite.

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_startup_system(setup).run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //let font = asset_server.load("fonts/Roboto-Regular.ttf");
    let font = asset_server.load("fonts/BevyTest.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Text2dBundle {
        text: Text::from_section("ABXY", text_style.clone()).with_alignment(text_alignment),
        ..default()
    });
}