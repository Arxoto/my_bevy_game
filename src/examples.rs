#![allow(dead_code)]

mod a_2d_shapes;
mod bloom_2d;
mod bounding_2d;
mod mesh2d_arcs;
mod move_sprite;
mod rotation;
mod sprite;
mod sprite_animation;
mod sprite_flipping;
mod sprite_sheet;
mod sprite_slice;
mod sprite_tile;

pub fn run_example() {
    // 绘制各种形状
    // a_2d_shapes::main();

    // 旋转动画
    // rotation::main();

    // sprite 精灵是一个抽象类 表示能感知时间流逝和有坐标位置的游戏元素
    // sprite::main();

    // sprite 根据时间位移
    // move_sprite::main();

    // sprite 翻转
    // sprite_flipping::main();

    // sprite 形变
    // sprite_slice::main();

    // sprite 平铺填充
    // sprite_tile::main();

    // sprite 逐帧播放动画
    // sprite_sheet::main();

    // sprite 交互式动画 底层实现拆分为动画的触发和执行
    // sprite_animation::main();

    // =========
    // 高级
    // =========

    // 体积碰撞和相交
    // bounding_2d::main();

    // 泛光效果
    // bloom_2d::main();

    // 演示了 弧形和扇形的 UV 映射
    // mesh2d_arcs::main();
}
