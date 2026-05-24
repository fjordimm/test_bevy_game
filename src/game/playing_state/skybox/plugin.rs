use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::{prelude::*, render::render_resource::*, shader::ShaderRef};
use bevy_mesh::Indices;
use std::f32::consts::PI;

use crate::game::{
    core::states::OverallState,
    playing_state::{
        SunPosition, player::tags::CameraForPlayer, sets::DuringPlayingUnpaused,
        tags::PlayingStateEntity,
    },
    util::{alrms, alrro},
};

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<SkyboxMaterial>::default())
            .add_systems(OnEnter(OverallState::Playing),
                spawn_skybox
            )
            .add_systems(Update,
                update_skybox
                    .in_set(DuringPlayingUnpaused::General)
            );
    }
}

#[derive(Component)]
pub struct SkyboxTag;

fn spawn_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SkyboxMaterial>>,
    sun_position: Res<SunPosition>,
) {
    commands.spawn((
        PlayingStateEntity,
        SkyboxTag,
        Mesh3d(meshes.add(alrro!(
            // Mesh::from(Sphere::new(10_000.0)).with_inverted_winding()
            // Mesh::from(Cuboid::new(10_000.0, 10_000.0, 10_000.0)).with_inverted_winding()
            Mesh::from(Icosahedron::new(500.0)).with_inverted_winding()
        ))),
        MeshMaterial3d(materials.add(SkyboxMaterial {
            sun_position: sun_position.0,
        })),
        Transform::default(),
    ));
}

fn update_skybox(
    camera_transf_q: Option<Single<&Transform, With<CameraForPlayer>>>,
    skybox_transf_q: Option<Single<&mut Transform, (With<SkyboxTag>, Without<CameraForPlayer>)>>,
    mut materials: ResMut<Assets<SkyboxMaterial>>,
    sun_position: Res<SunPosition>,
) {
    // Move it to be the same position as the camera.

    if let (Some(camera_transf), Some(mut skybox_transf)) =
        (alrms!(camera_transf_q), alrms!(skybox_transf_q))
    {
        skybox_transf.translation = camera_transf.translation;
    }

    // Update the shader uniforms.

    materials.iter_mut().for_each(|(_, mat)| {
        mat.sun_position = sun_position.0;
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SkyboxMaterial {
    #[uniform(0)]
    pub sun_position: Vec3,
}

impl Material for SkyboxMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sky.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

pub struct Icosahedron {
    pub radius: f32,
}

impl Icosahedron {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl From<Icosahedron> for Mesh {
    fn from(icosahedron: Icosahedron) -> Self {
        let radius = icosahedron.radius;
        let t = (1.0 + 5.0_f32.sqrt()) / 2.0;

        let vertices = [
            [-1.0, t, 0.0],
            [1.0, t, 0.0],
            [-1.0, -t, 0.0],
            [1.0, -t, 0.0],
            [0.0, -1.0, t],
            [0.0, 1.0, t],
            [0.0, -1.0, -t],
            [0.0, 1.0, -t],
            [t, 0.0, -1.0],
            [t, 0.0, 1.0],
            [-t, 0.0, -1.0],
            [-t, 0.0, 1.0],
        ];

        let faces = [
            [0_u32, 11, 5],
            [0, 5, 1],
            [0, 1, 7],
            [0, 7, 10],
            [0, 10, 11],
            [1, 5, 9],
            [5, 11, 4],
            [11, 10, 2],
            [10, 7, 6],
            [7, 1, 8],
            [3, 9, 4],
            [3, 4, 2],
            [3, 2, 6],
            [3, 6, 8],
            [3, 8, 9],
            [4, 9, 5],
            [2, 4, 11],
            [6, 2, 10],
            [8, 6, 7],
            [9, 8, 1],
        ];

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();

        for vertex in vertices {
            positions.push([vertex[0] * radius, vertex[1] * radius, vertex[2] * radius]);

            let len =
                (vertex[0] * vertex[0] + vertex[1] * vertex[1] + vertex[2] * vertex[2]).sqrt();
            let normal = [vertex[0] / len, vertex[1] / len, vertex[2] / len];
            normals.push(normal);

            // let u = 1.0 - (((normal[2].atan2(normal[0]) + PI) * 2.0 / PI) % 2.0 - 1.0).abs();
            // let u = 1.0 - ((normal[2].atan2(normal[0]) + PI) / PI - 1.0).abs();
            let u = 0.5 + normal[2].atan2(normal[0]) / (1.0 * PI);
            let v = 0.5 + normal[1].asin() / PI;
            uvs.push([u, v]);
        }

        // // Fix uv seams
        // for 
        // let min_u = uvs
        //     .iter()
        //     .copied()
        //     .fold(f32::INFINITY, |a, b| f32::min(a, b[0]));
        // let max_u = uvs
        //     .iter()
        //     .copied()
        //     .fold(f32::NEG_INFINITY, |a, b| f32::max(a, b[0]));
        // if max_u - min_u > 0.5 {
        //     // triangle crosses seam
        // }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(faces.as_flattened().to_vec()));

        mesh
    }
}
