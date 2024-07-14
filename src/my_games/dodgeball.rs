mod bi_vector;
mod matter;
mod map;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use map::{MapShowable, WORLD_MAP};
use matter::Area;

/// 位置和形状 与物理运动和显示关联  
/// 颜色 与功能关联  
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "dodgeball".into(),
                resolution: (820., 640.).into(),
                resizable: true,
                decorations: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 摄像机
    commands.spawn(Camera2dBundle::default());

    // map
    commands.spawn(MaterialMesh2dBundle {
        transform: WORLD_MAP.gen_transform(),
        mesh: Mesh2dHandle(meshes.add(WORLD_MAP.gen_mesh())),
        material: materials.add(WORLD_MAP.gen_color()),
        ..default()
    });

    // place
    let place_list = [
        Area::new_producter(300.0, 200.0, 40.0),
        Area::new_consumer(-300.0, -200.0, 50.0),
    ];
    for place in place_list.into_iter() {
        let (the_circle, the_transform, the_color) = place.map_2_map();
        commands.spawn(MaterialMesh2dBundle {
            transform: the_transform,
            mesh: Mesh2dHandle(meshes.add(the_circle)),
            material: materials.add(the_color),
            ..Default::default()
        });
    }
}
