use std::collections::VecDeque;

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

pub struct SplinePlugin;

#[derive(Resource, Default)]
struct ControlPoints {
    points: VecDeque<Vec2>,
}

#[derive(Component)]
struct ControlPointMarker;
#[derive(Component)]
struct ConnectorLine;
#[derive(Component)]
struct CurvedConnectorLine;

impl Plugin for SplinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (redraw_spline, handle_click));
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(ControlPoints {
        points: VecDeque::from([
            vec2(-500., -200.),
            vec2(-250., 250.),
            vec2(250., 250.),
            vec2(500., -200.),
        ]),
    });
}

fn handle_click(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut control_points: ResMut<ControlPoints>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(pos) = window
        .cursor_position()
        .and_then(|cp| camera.0.viewport_to_world_2d(camera.1, cp).ok())
    {
        let points = &mut control_points.points;
        points.push_back(pos);
        if points.len() > 4 {
            points.pop_front();
        }
    }
}

fn redraw_spline(
    mut commands: Commands,
    control_points: Res<ControlPoints>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    old_dots: Query<Entity, With<ControlPointMarker>>,
    old_lines: Query<Entity, With<ConnectorLine>>,
    old_curve_lines: Query<Entity, With<CurvedConnectorLine>>,
) {
    if !control_points.is_changed() {
        return;
    }

    for e in old_dots
        .iter()
        .chain(old_lines.iter())
        .chain(old_curve_lines.iter())
    {
        commands.entity(e).despawn();
    }

    let points = &control_points.points;

    // Points
    let material = MeshMaterial2d(materials.add(Color::hsl(267., 0.57, 0.78)));
    for &p in points.iter() {
        let mesh2d = Mesh2d(meshes.add(RegularPolygon::new(10.0, 6)));
        let transform = Transform::from_translation(p.extend(1.));
        commands.spawn((ControlPointMarker, mesh2d, material.clone(), transform));
    }
    drop(material);

    // // Straigh Connecting lines
    // if points.len() >= 2 {
    //     let mesh2d = Mesh2d(meshes.add(build_line_strip(points, 1.)));
    //     let material = MeshMaterial2d(materials.add(Color::hsl(189., 0.43, 0.73)));
    //     let transform = Transform::from_translation(Vec3::Z * 0.5);
    //     commands.spawn((ConnectorLine, mesh2d, material, transform));
    // }

    // Catmull-Rom Curve
    if points.len() >= 2 {
        let mut points = points.clone();
        points.push_front(2. * points[0] - points[1]);
        points.push_back(2. * points[points.len() - 1] - points[points.len() - 2]);
        let curve = sample_catmull_rom(&points, 32);

        let mesh2d = Mesh2d(meshes.add(build_line_strip(&curve, 1.)));
        let material = MeshMaterial2d(materials.add(Color::hsl(197., 0.49, 0.38)));
        let transform = Transform::from_translation(Vec3::Z * 0.75);
        commands.spawn((CurvedConnectorLine, mesh2d, material, transform));
    }
}

fn build_line_strip(points: &VecDeque<Vec2>, width: f32) -> Mesh {
    let hw = width * 0.5;
    let n = points.len();
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n * 2);
    let mut indices: Vec<u32> = Vec::with_capacity((n - 1) * 6);

    for (i, &p) in points.iter().enumerate() {
        let dir = if i == 0 {
            (points[1] - points[0]).normalize_or_zero()
        } else if i == n - 1 {
            (points[n - 1] - points[n - 2]).normalize_or_zero()
        } else {
            ((points[i + 1] - points[i]).normalize_or_zero()
                + (points[i] - points[i - 1]).normalize_or_zero())
            .normalize_or_zero()
        };
        let perp = Vec2::new(-dir.y, dir.x) * hw;
        positions.push((p + perp).extend(0.0).to_array());
        positions.push((p - perp).extend(0.0).to_array());
    }

    for i in 0..(n as u32 - 1) {
        let b = i * 2;
        indices.extend_from_slice(&[b, b + 1, b + 2, b + 1, b + 3, b + 2]);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

pub fn catmull_rom(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32, alpha: f32) -> Vec2 {
    let t2 = t * t;
    let t3 = t2 * t;
    let q1 = -t3 + 2.0 * t2 - t;
    let q2 = 3.0 * t3 - 5.0 * t2 + 2.0;
    let q3 = -3.0 * t3 + 4.0 * t2 + t;
    let q4 = t3 - t2;
    (p0 * q1 + p1 * q2 + p2 * q3 + p3 * q4) * (alpha * 0.5)
}

pub fn sample_catmull_rom(points: &VecDeque<Vec2>, samples: usize) -> VecDeque<Vec2> {
    if points.len() < 4 {
        return points.to_owned();
    }
    let mut out = VecDeque::new();
    for i in 0..points.len().saturating_sub(3) {
        let (p0, p1, p2, p3) = (points[i], points[i + 1], points[i + 2], points[i + 3]);
        for s in 0..=samples {
            let t = s as f32 / samples as f32;
            out.push_back(catmull_rom(p0, p1, p2, p3, t, 1.0));
        }
    }
    out
}
