use core::f32;
use std::collections::VecDeque;

use ::bevy::prelude::*;
use bevy_prototype_lyon::{
    entity::Shape,
    path::ShapePath,
    prelude::{ShapeBuilder, ShapeBuilderBase},
};

use crate::raillines::{ControlPoints, RaliLineTraveller, TravelDirection};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct CursorLine;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let control_points = VecDeque::from([
        vec2(-500., -200.),
        vec2(-250., 250.),
        vec2(250., 250.),
        vec2(500., -200.),
    ]);

    let mesh2d = Mesh2d(meshes.add(RegularPolygon::new(10.0, 6)));
    let material = MeshMaterial2d(materials.add(Color::hsl(189., 0.43, 0.73)));
    let transform = Transform::from_translation(Vec3::Z * 1.0);
    commands.spawn((
        Player,
        mesh2d,
        material.clone(),
        transform,
        ControlPoints {
            points: control_points,
        },
        RaliLineTraveller {
            next_target: 0,
            speed: 500.,
            direction: TravelDirection::Forward,
        },
    ));

    let mesh2d = Mesh2d(meshes.add(RegularPolygon::new(5.0, 6)));
    let transform = Transform::from_translation(Vec3::Z * 1.);
    commands.spawn((Cursor, mesh2d, material, transform));

    let line = ShapeBuilder::with(
        &ShapePath::new()
            .move_to(Vec2::default())
            .line_to(Vec2::default()),
    )
    .stroke((Color::hsl(189., 0.43, 0.73), 2.))
    .build();
    let transform = Transform::from_translation(Vec3::Z * 0.5);
    commands.spawn((CursorLine, line, transform));
}

fn handle_click_and_show_mouse(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    player: Single<(&mut ControlPoints, &mut RaliLineTraveller), With<Player>>,
    mut cursor: Single<&mut Transform, With<Cursor>>,
    cursor_line: Single<Entity, With<CursorLine>>,
) {
    if let Some(pos) = window
        .cursor_position()
        .and_then(|cp| camera.0.viewport_to_world_2d(camera.1, cp).ok())
    {
        let (control_points, traveller) = &mut player.into_inner();

        // Modify Cursor translation
        cursor.translation = pos.extend(2.);

        // Redraw Cursor line
        commands.entity(cursor_line.into_inner()).despawn();
        let line = ShapeBuilder::with(&ShapePath::new().move_to(cursor.translation.xy()).line_to(
            match traveller.direction {
                TravelDirection::Forward => control_points.points[control_points.points.len() - 1],
                TravelDirection::Backward => control_points.points[0],
            },
        ))
        .stroke((Color::hsl(189., 0.43, 0.73), 2.))
        .build();
        let transform = Transform::from_translation(Vec3::Z * 0.5);
        commands.spawn((CursorLine, line, transform));

        // If Clicked add a new point to the control_points
        if !mouse_buttons.just_pressed(MouseButton::Left) {
            return;
        }
        match traveller.direction {
            TravelDirection::Forward => control_points.points.push_back(pos),
            TravelDirection::Backward => {
                control_points.points.push_front(pos);
                traveller.next_target += 1;
            }
        }
    }
}

fn trim_railroad(
    player: Single<(&mut ControlPoints, &mut RaliLineTraveller), With<Player>>,
    mut camera: Single<&mut Transform, With<Camera>>,
) {
    let (control_points, traveller) = &mut player.into_inner();
    if control_points.points.len() <= 4 {
        return;
    }

    let mut trimmed = false;

    match traveller.direction {
        TravelDirection::Forward => {
            if traveller.next_target > 1 {
                control_points.points.pop_front();
                traveller.next_target -= 1;
                trimmed = true;
            }
        }
        TravelDirection::Backward => {
            if traveller.next_target < control_points.points.len() - 2 {
                control_points.points.pop_back();
                trimmed = true;
            }
        }
    }

    if trimmed {
        // TODO: lerp
        camera.rotate_z(f32::consts::FRAC_PI_6 / 2.);
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (handle_click_and_show_mouse, trim_railroad));
    }
}
