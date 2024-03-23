use bevy::{app::AppExit, prelude::*};

#[derive(Component, Debug)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_quit);
    }
}

fn listen_for_quit(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: ResMut<Events<AppExit>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}