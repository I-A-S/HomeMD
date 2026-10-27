use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (ui_system, update_camera_controls))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = meshes.add(Sphere::new(0.3));
    let cylinder_mesh = meshes.add(Cylinder::new(0.15, 1.0));

    let carbon_mat = materials.add(Color::srgb(0.2, 0.2, 0.2));
    let oxygen_mat = materials.add(Color::srgb(0.8, 0.1, 0.1));
    let bond_mat = materials.add(Color::srgb(0.75, 0.75, 0.75));

    commands.spawn(PbrBundle {
        mesh: sphere_mesh.clone(), 
        material: carbon_mat.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: sphere_mesh.clone(), 
        material: oxygen_mat.clone(),
        transform: Transform::from_xyz(-1.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: sphere_mesh.clone(), 
        material: oxygen_mat.clone(),
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: cylinder_mesh.clone(), 
        material: bond_mat.clone(),
        transform: Transform::from_xyz(-0.5, 0.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::PI/2.0)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: cylinder_mesh.clone(), 
        material: bond_mat.clone(),
        transform: Transform::from_xyz(0.5, 0.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::PI/2.0)),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight { intensity: 2_000_000.0, ..default() },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

fn ui_system(mut contexts: EguiContexts) {
    egui::Window::new("HomeMD").show(contexts.ctx_mut(), |ui| {
        if ui.button("Start").clicked() {
        }
    });
}

fn update_camera_controls(
    mut contexts: EguiContexts,
    mut camera_query: Query<&mut PanOrbitCamera>,
) {
    let ctx = contexts.ctx_mut();
    let ui_wants_pointer = ctx.wants_pointer_input() || ctx.is_pointer_over_area();

    for mut camera in camera_query.iter_mut() {
        camera.enabled = !ui_wants_pointer;
    }
}
