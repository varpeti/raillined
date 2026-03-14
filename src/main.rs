mod spline_plugin;

use bevy::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    post_process::bloom::Bloom,
    prelude::*,
};
//use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::spline_plugin::SplinePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SplinePlugin)
        // .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

    let mesh2d = Mesh2d(meshes.add(RegularPolygon::new(10.0, 6)));
    let material = MeshMaterial2d(materials.add(Color::hsl(2., 0.55, 0.83)));
    let transform = Transform::from_xyz(0., 0., 0.);
    commands.spawn((mesh2d, material, transform));
}
