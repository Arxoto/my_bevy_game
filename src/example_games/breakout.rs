//! A simplified implementation of the classic game "Breakout".
//!
//! Demonstrates Bevy's stepping capabilities if compiled with the `bevy_debug_stepping` feature.

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use super::stepping;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
// These values are exact
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 自定义的逐帧调试插件
        .add_plugins(
            stepping::SteppingPlugin::default()
                .add_schedule(Update)
                .add_schedule(FixedUpdate)
                .at(Val::Percent(35.0), Val::Percent(50.0)),
        )
        // 游戏得分
        .insert_resource(Score(0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // 自定义碰撞事件
        .add_event::<CollisionEvent>()
        // 初始化逻辑
        .add_systems(Startup, setup)
        // 把碰撞计算添加到固定步长的调度中去
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddle,
                check_for_collisions,
                play_collision_sound,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, update_scoreboard)
        .run();
}

/// 踏板
#[derive(Component)]
struct Paddle;

/// 球
#[derive(Component)]
struct Ball;

/// 速度向量
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

/// 碰撞体
#[derive(Component)]
struct Collider;

/// 碰撞事件
#[derive(Event, Default)]
struct CollisionEvent;

/// 砖块
#[derive(Component)]
struct Brick;

#[derive(Resource, Deref)]
struct CollisionSound(Handle<AudioSource>);

// This bundle is a collection of the components that define a "wall" in our game
/// 墙的捆绑包
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    /// Location of the *center* of the wall, used in `transform.translation()`
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    /// (x, y) dimensions of the wall, used in `transform.scale()`
    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

// This resource tracks the game's score
#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
struct ScoreboardUi;

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 视角相机
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // 固定的踏板的 y 坐标
    // Paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    // 添加踏板 具有碰撞体 和位置渲染组件
    commands.spawn((
        SpriteBundle {
            // 变换 定义了 位置 旋转 缩放
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE.extend(1.0),
                ..default()
            },
            // 精灵 游戏开发属于 常指代能感知时间流逝的物体 这里主要用作渲染
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));

    // 添加球 具有初速度  注意 整个游戏只有小球会与其他碰撞物体相撞 为简化计算 小球不设置碰撞
    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(BALL_COLOR),
            transform: Transform::from_translation(BALL_STARTING_POSITION)
                .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));

    // 得分板
    // Scoreboard
    commands.spawn((
        ScoreboardUi,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    ));

    // 四面墙
    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    // 加入砖块
    // Bricks
    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // Because we need to round the number of columns,
    // the space on the top and sides of the bricks only captures a lower bound, not an exact value
    let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_bricks = center_of_bricks
        // Space taken up by the bricks
        - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
        // Space taken up by the gaps
        - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    // In Bevy, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
    let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            // brick
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Brick,
                Collider,
            ));
        }
    }
}

/// 根据键盘输入控制踏板移动
fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    // 计算新坐标 速度乘以时间差
    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_seconds();

    // 在墙范围约束内刷新坐标
    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

/// 对所有具有速度组件的物体 根据时间差刷新下一时刻的位置
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.to_string();
}

/// 碰撞逻辑  
/// 球不具备碰撞属性 所以需要单独拿出来
fn check_for_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();

    for (collider_entity, collider_transform, maybe_brick) in &collider_query {
        let collision = ball_collision(
            // 小球 正圆
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.),
            // 碰撞体 均为矩形
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        // 小球与每个碰撞体之间每次至多产生一个碰撞
        if let Some(collision) = collision {
            // 发送一个碰撞事件
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // 如果是砖块碰撞了 就需要删除并且加分
            // Bricks should be despawned and increment the scoreboard on collision
            if maybe_brick.is_some() {
                commands.entity(collider_entity).despawn();
                **score += 1;
            }

            // 碰撞反射
            // Reflect the ball's velocity when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // 只有当速度与碰撞方向相反时才反射 这可以防止球卡在杆内
            // Reflect only if the velocity is in the opposite direction of the collision
            // This prevents the ball from getting stuck inside the bar
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            }

            // Reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // Reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

/// 播放碰撞音效 
fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        commands.spawn(AudioBundle {
            source: sound.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// Returns `Some` if `ball` collides with `bounding_box`.
// The returned `Collision` is the side of `bounding_box` that `ball` hit.
/// 返回碰撞方向 根据碰撞方向来扭转速度方向 完全弹性碰撞
fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    // 自带的相交检测
    if !ball.intersects(&bounding_box) {
        return None;
    }

    // 找到与圆心距离最近的点 最远不会超过矩形范围
    let closest = bounding_box.closest_point(ball.center());
    // 计算相对位置
    let offset = ball.center() - closest;
    // 计算碰撞方向 位移大先装上的优先
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}