use rand::distributions::{Distribution, Uniform};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_pancam::{PanCam, PanCamPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(PanCamPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default())
    .insert(PanCam::default());

    let mut rng = rand::thread_rng();

    let mut points: Vec<(f32, f32)> = vec![(0.0,0.0)];
    let uniform: Uniform<f32> = Uniform::from(0.0..1.0);

    for _ in 0..30000 { 

        let p = uniform.sample(&mut rng);
        
        let point = points.last();
        let x = point.unwrap().0;
        let y = point.unwrap().1;

        let new_point: (f32, f32);

        if p < 0.01 {
            new_point = (0.0, 0.16 * y);
        } 
        else if p < 0.86 {
            new_point = (0.85 * x + 0.04 * y, -0.04 * x + 0.85 * y + 1.6);
        }
        else if p < 0.93 {
            new_point = (0.2 * x - 0.26 * y, 0.23 * x + 0.22 * y + 1.6);
        }
        else {
            new_point = (-0.15 * x + 0.28 * y - 0.028, 0.26 * x + 0.24 * y + 1.05);
        }

        points.push(new_point);

    }   

    for point in points {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2 { x: (1.), y: (1.) }).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.0, 0.5, 0.0))),
            transform: Transform::from_translation(Vec3::new(point.0 * 50.0, point.1 * 50.0, 0.)),
            ..default()
        });
    }

}
