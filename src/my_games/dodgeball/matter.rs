//! matter 物质 本模块定义了物理运动相关

use super::bi_vector::{BiVector, Unit};

/// 简化为圆 具有位置和体积等空间属性
pub struct SpaceObject {
    pub(crate) location: BiVector,
    pub(crate) radius: Unit,
}

/// 实体 具有质量和速度等物理属性
pub struct Entity {
    pub(crate) space: SpaceObject,
    pub(crate) velocity: BiVector,
    pub(crate) mass: Unit,
}

impl Entity {
    /// 移动  
    /// s = v * t
    fn move_4_a_while(&mut self, delta_time: Unit) {
        self.space.location += self.velocity * delta_time;
    }

    /// 受到牵引力的作用  
    /// a = F / m  
    /// delta v = a * t  
    fn receive_traction(&mut self, force: BiVector, delta_time: Unit) {
        self.velocity += force / self.mass * delta_time;
    }
}

/// 区域 空间位置的特殊标记 不参与物理运动
pub struct Area {
    pub space: SpaceObject,
    pub place_type: PlaceType,
}

pub enum PlaceType {
    Producter,
    Consumer,
}

impl Area {
    pub fn new_producter(x: Unit, y: Unit, r: Unit) -> Area {
        Area {
            space: SpaceObject {
                location: BiVector(x, y),
                radius: r,
            },
            place_type: PlaceType::Producter,
        }
    }

    pub fn new_consumer(x: Unit, y: Unit, r: Unit) -> Area {
        Area {
            space: SpaceObject {
                location: BiVector(x, y),
                radius: r,
            },
            place_type: PlaceType::Consumer,
        }
    }
}
