use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::transform::components::Transform;
use bevy_oxr::graphics::extensions::XrExtensions;
use bevy_oxr::graphics::XrAppInfo;
use bevy_oxr::graphics::XrPreferdBlendMode::AlphaBlend;
use bevy_oxr::passthrough::{passthrough_layer_pause, passthrough_layer_resume};
use bevy_oxr::xr_init::XrRenderData;
use bevy_oxr::xr_input::debug_gizmos::OpenXrDebugRenderer;
use bevy_oxr::xr_input::prototype_locomotion::{proto_locomotion, PrototypeLocomotionConfig};
use bevy_oxr::xr_input::trackers::{
    OpenXRController, OpenXRLeftController, OpenXRRightController, OpenXRTracker,
};
use bevy_oxr::DefaultXrPlugins;

#[bevy_main]
fn main() {
    let mut xr_extensions = XrExtensions::default();
    xr_extensions.enable_fb_passthrough();
    App::new()
        .add_plugins(DefaultXrPlugins {
            reqeusted_extensions: xr_extensions,
            app_info: XrAppInfo {
                name: "Bevy OXR Android Example".into(),
            },
            prefered_blend_mode: bevy_oxr::graphics::XrPreferdBlendMode::Opaque,
        })
        .add_plugins(OpenXrDebugRenderer)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (proto_locomotion, toggle_passthrough))
        .add_systems(Startup, spawn_controllers_example)
        .insert_resource(PrototypeLocomotionConfig::default())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.8, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 1.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn spawn_controllers_example(mut commands: Commands) {
    //left hand
    commands.spawn((
        OpenXRLeftController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
    ));
    //right hand
    commands.spawn((
        OpenXRRightController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
    ));
}

// Does this work? Not getting logs
fn toggle_passthrough(keys: Res<Input<KeyCode>>, mut xr_data: ResMut<XrRenderData>) {
    if keys.just_pressed(KeyCode::Space) {
        if xr_data.xr_passthrough_active {
            passthrough_layer_pause(xr_data);
            bevy::log::info!("Passthrough paused");
        } else {
            passthrough_layer_resume(xr_data);
            bevy::log::info!("Passthrough resumed");
        }
    }
}
