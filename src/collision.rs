#[derive(Clone, Copy)]
pub struct BoundingBox {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

use super::objects::Vec2d;

type Repulsion = Vec2d;

impl BoundingBox {
    pub fn is_intersecting(&self, other: &Self) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
    }
    pub fn invert(&self) -> BoundingBox {
        let mut bb = *self;
        (bb.min_x, bb.max_x) = (bb.max_x, bb.min_x);
        (bb.min_y, bb.max_y) = (bb.max_y, bb.min_y);
        bb
    }
    pub fn is_inside(&self, other: &BoundingBox) -> bool {
        self.min_x >= other.min_x
            && self.max_x <= other.max_x
            && self.min_y >= other.min_y
            && self.max_y <= other.max_y
    }
}

pub trait Collideable {
    fn get_bb(&self) -> BoundingBox;

    fn invert_bb(&self) -> BoundingBox {
        let mut bb = self.get_bb();
        (bb.min_x, bb.max_x) = (bb.max_x, bb.min_x);
        (bb.min_y, bb.max_y) = (bb.max_y, bb.min_y);
        bb
    }

    fn check_intersection<T: Collideable>(&self, other: &T) -> bool {
        self.get_bb().is_intersecting(&other.get_bb())
    }
}
