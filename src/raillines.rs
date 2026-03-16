#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_prototype_lyon::{
    path::ShapePath,
    plugin::ShapePlugin,
    prelude::{ShapeBuilder, ShapeBuilderBase},
};
use itertools::Itertools;

pub struct RailLinesPlugin;

#[derive(Component, Default)]
pub struct ControlPoints {
    pub points: VecDeque<Vec2>,
}

#[derive(Component)]
pub struct ControlPointMarkers(Vec<Entity>);

#[derive(Component)]
pub struct ControlPointLines(Vec<Entity>);

fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    all_changed_control_points: Query<
        (
            Entity,
            &ControlPoints,
            Option<&ControlPointMarkers>,
            Option<&ControlPointLines>,
        ),
        Changed<ControlPoints>,
    >,
) {
    for (entity, control_points, control_point_markers, control_point_lines) in
        all_changed_control_points
    {
        if let Some(control_point_markers) = control_point_markers {
            for &e in control_point_markers.0.iter() {
                commands.entity(e).despawn();
            }
        }
        if let Some(control_point_lines) = control_point_lines {
            for &e in control_point_lines.0.iter() {
                commands.entity(e).despawn();
            }
        }

        let points = &control_points.points;

        // Control Point Markers
        let mut control_point_markers = ControlPointMarkers(vec![]);
        let material = MeshMaterial2d(materials.add(Color::hsl(267., 0.57, 0.78)));
        for &p in points.iter() {
            let mesh2d = Mesh2d(meshes.add(RegularPolygon::new(5.0, 6)));
            let transform = Transform::from_translation(p.extend(1.));
            let id = commands.spawn((mesh2d, material.clone(), transform)).id();
            control_point_markers.0.push(id);
        }
        commands.entity(entity).insert(control_point_markers);

        // Control Point Lines
        let mut control_point_lines = ControlPointLines(vec![]);
        if points.len() >= 2 {
            for (a, b) in points.iter().tuple_windows() {
                let line = ShapeBuilder::with(&ShapePath::new().move_to(*a).line_to(*b))
                    .stroke((Color::hsl(189., 0.43, 0.73), 2.))
                    .build();
                let transform = Transform::from_translation(Vec3::Z * 0.5);
                let id = commands.spawn((line, transform)).id();
                control_point_lines.0.push(id);
            }
        }
        commands.entity(entity).insert(control_point_lines);
    }
}

#[derive(Component)]
pub struct RaliLineTraveller {
    pub next_target: usize,
    pub speed: f32,
    pub direction: TravelDirection,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum TravelDirection {
    #[default]
    Forward,
    Backward,
}

fn travel(
    time: Res<Time>,
    mut travellers: Query<(&mut RaliLineTraveller, &mut Transform, &ControlPoints)>,
) {
    for (mut traveller, mut transform, control_points) in &mut travellers {
        if control_points.points.len() < 2 {
            continue;
        }

        let target_pos = match control_points.points.get(traveller.next_target) {
            Some(target_pos) => target_pos.extend(transform.translation.z),
            None => {
                continue;
            }
        };

        let delta = traveller.speed * time.delta_secs();
        let direction = (target_pos - transform.translation).normalize();
        transform.translation += direction * delta;

        if transform.translation.distance(target_pos) < delta * 2. {
            match traveller.direction {
                TravelDirection::Forward => {
                    if control_points.points.len() <= traveller.next_target + 1 {
                        traveller.direction = TravelDirection::Backward;
                    } else {
                        traveller.next_target += 1;
                    }
                }
                TravelDirection::Backward => {
                    if 0 == traveller.next_target {
                        traveller.direction = TravelDirection::Forward;
                    } else {
                        traveller.next_target -= 1;
                    }
                }
            }
        }
    }
}

impl Plugin for RailLinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
            .add_systems(Update, (draw, travel));
    }
}
