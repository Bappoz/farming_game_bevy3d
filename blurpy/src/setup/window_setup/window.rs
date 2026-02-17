use bevy::prelude::*;
use bevy::window::{MonitorSelection, Window, WindowMode, WindowPlugin};

pub struct GameWindowPlugin;

impl Plugin for GameWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "FarmSimulation".into(),
                mode: if cfg!(debug_assertions) {
                    WindowMode::Windowed
                } else {
                    WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current)
                },
                resizable: false,
                ..default()
            }),
            ..default()
        }));
    }
}
