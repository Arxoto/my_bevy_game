//! Shows how to render simple primitive shapes with a single color.

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin},
};

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_wireframe)
        .run();
}

const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // 将原始图形加入到 Mesh Resource 网格资源中防止被销毁 并返回其句柄
    let shapes = [
        // 圆
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        // 圆扇形 angle 角度即弧度与半径的倍数 如 Pi 等同于整圆
        Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
        // 圆片段 同上
        Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
        // 椭圆 即圆的拉伸
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        // 圆环
        Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
        // 胶囊 与线段在一定距离内的点的集合
        // 范围半径即两端半圆的半径 线段长度即中间矩形的长度
        // 实测渲染时中间长度会略长于预期 目前原因尚不清楚
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        // 菱形 水平对角线 竖直对角线 即宽高
        Mesh2dHandle(meshes.add(Rhombus::new(75.0, 100.0))),
        // 矩形 宽高
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        // 正多边形 circumradius 表示外接圆半径
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        // 三角形2d
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        // 将材质网格的捆绑包加入到世界中
        // 位置信息是以窗口正中心为原点的坐标
        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }

    // 添加左上角按键交互的提示信息
    commands.spawn(
        TextBundle::from_section("Press space to toggle wireframes", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}