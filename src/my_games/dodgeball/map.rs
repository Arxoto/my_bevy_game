use bevy::{
    color::Color,
    prelude::{Circle, Rectangle},
    transform::components::Transform,
};

use super::{
    bi_vector::{BiVector, Unit},
    matter::{Entity, Area},
};

pub static SHOWN_LOGICAL_RATIO: Unit = 1.;

pub struct WorldMap {
    logical_size: BiVector,
}

pub static WORLD_MAP: WorldMap = WorldMap {
    logical_size: BiVector(800.0, 600.0),
};

trait MapSized {
    type Output;
    fn to_map_size(&self) -> Self::Output;
}

impl MapSized for Unit {
    type Output = f32;

    fn to_map_size(&self) -> Self::Output {
        (self * SHOWN_LOGICAL_RATIO) as f32
    }
}

impl WorldMap {
    pub fn gen_mesh(&self) -> Rectangle {
        Rectangle::new(
            self.logical_size.0.to_map_size(),
            self.logical_size.1.to_map_size(),
        )
    }

    pub fn gen_transform(&self) -> Transform {
        Transform::from_xyz(0.0, 0.0, 0.0)
    }

    pub fn gen_color(&self) -> Color {
        Color::WHITE
    }
}

pub trait MapShowable {
    fn map_2_map(&self) -> (Circle, Transform, Color);
}

impl MapShowable for Area {
    fn map_2_map(&self) -> (Circle, Transform, Color) {
        (
            Circle::new(self.space.radius.to_map_size()),
            Transform::from_xyz(
                self.space.location.0.to_map_size(),
                self.space.location.1.to_map_size(),
                1.,
            ),
            match self.place_type {
                super::matter::PlaceType::Producter => Color::hsl(350.0, 0.40, 0.80),
                super::matter::PlaceType::Consumer => Color::hsl(173.0, 0.50, 0.75),
            },
        )
    }
}

impl MapShowable for Entity {
    fn map_2_map(&self) -> (Circle, Transform, Color) {
        (
            Circle::new(self.space.radius.to_map_size()),
            Transform::from_xyz(
                self.space.location.0.to_map_size(),
                self.space.location.1.to_map_size(),
                100.,
            ),
            Color::BLACK,
        )
    }
}
