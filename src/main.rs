mod player;
mod raillines;

use bevy::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    post_process::bloom::Bloom,
    prelude::*,
    window::{CursorOptions, WindowResolution},
};

use crate::{player::PlayerPlugin, raillines::RailLinesPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RailLines".to_string(),
                resolution: WindowResolution::new(1080, 1080),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RailLinesPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut cursor: Single<&mut CursorOptions>) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::hsl(249., 0.22, 0.12)),
            ..Default::default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMax {
                max_width: 1080.,
                max_height: 1080.,
            },
            scale: 1.,
            ..OrthographicProjection::default_2d()
        }),
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

    cursor.visible = false;
}
