mod player;
mod raillines;

use bevy::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    post_process::bloom::Bloom,
    prelude::*,
};

use crate::{player::PlayerPlugin, raillines::RailLinesPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RailLinesPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    //window: Single<&Window>,
) {
    //let window_size = window.resolution.physical_size().as_vec2();

    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::hsl(249., 0.22, 0.12)),
            ..Default::default()
        },
        Tonemapping::SomewhatBoringDisplayTransform,
        Bloom {
            intensity: 0.15,
            low_frequency_boost: 0.5,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            composite_mode: bevy::post_process::bloom::BloomCompositeMode::Additive,
            ..Default::default()
        },
        DebandDither::Enabled,
    ));
}
