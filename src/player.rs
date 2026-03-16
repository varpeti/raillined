use std::collections::VecDeque;

use ::bevy::prelude::*;

use crate::raillines::{ControlPoints, RaliLineTraveller, TravelDirection};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh2d = Mesh2d(meshes.add(RegularPolygon::new(10.0, 6)));
    let material = MeshMaterial2d(materials.add(Color::hsl(2., 0.55, 0.83)));
    let transform = Transform::from_translation(Vec3::Z * 1.);
    let id = commands
        .spawn((
            Player,
            mesh2d,
            material,
            transform,
            ControlPoints {
                points: VecDeque::from([
                    vec2(-500., -200.),
                    vec2(-250., 250.),
                    vec2(250., 250.),
                    vec2(500., -200.),
                ]),
            },
            RaliLineTraveller {
                next_target: 2,
                speed: 500.,
                direction: TravelDirection::Backward,
            },
        ))
        .id();

    println!("Player Setup: {}", id);
}

fn handle_click(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    player: Single<(&mut ControlPoints, &mut RaliLineTraveller), With<Player>>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(pos) = window
        .cursor_position()
        .and_then(|cp| camera.0.viewport_to_world_2d(camera.1, cp).ok())
    {
        let (control_points, traveller) = &mut player.into_inner();

        match traveller.direction {
            TravelDirection::Forward => control_points.points.push_back(pos),
            TravelDirection::Backward => {
                control_points.points.push_front(pos);
                traveller.next_target += 1;
            }
        }
    }
}

fn trim_railroad(player: Single<(&mut ControlPoints, &mut RaliLineTraveller), With<Player>>) {
    let (control_points, traveller) = &mut player.into_inner();
    if control_points.points.len() <= 4 {
        return;
    }

    match traveller.direction {
        TravelDirection::Forward => {
            if traveller.next_target > 1 {
                control_points.points.pop_front();
                traveller.next_target -= 1;
            }
        }
        TravelDirection::Backward => {
            if traveller.next_target < control_points.points.len() - 2 {
                control_points.points.pop_back();
            }
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (handle_click, trim_railroad));
    }
}
