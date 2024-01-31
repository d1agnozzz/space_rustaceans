use macroquad::color::Color;
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
    pub move_speed: f32,
    pub boosted_move_speed: f32,
    pub is_boosted: bool,
    pub vx: f32,
    pub vy: f32,
}

pub struct ScreenBorders {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

use super::collision::{BoundingBox, Collideable};
use macroquad::shapes::draw_rectangle;

impl Collideable for ScreenBorders {
    fn get_bb(&self) -> BoundingBox {
        BoundingBox {
            min_x: self.min_x,
            min_y: self.min_y,
            max_x: self.max_x,
            max_y: self.max_y,
        }
    }
    fn check_intersection<T: Collideable>(&self, other: &T) -> bool {
        self.get_bb().is_inside(&other.get_bb())
    }
}

impl Rectangle {
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }

    pub fn move_relative(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
    pub fn move_by_velocity(&mut self, borders: &ScreenBorders) {
        self.x += self.vx;

        if self.x < borders.min_x {
            self.x += borders.min_x - self.x;
        }
        if self.x + self.w > borders.max_x {
            self.x += borders.max_x - self.x - self.w;
        }
    }

    pub fn clear_velocity(&mut self) {
        self.vx = 0.0;
        self.vy = 0.0;
    }

    pub fn speed(&self) -> f32 {
        if self.is_boosted {
            self.boosted_move_speed
        } else {
            self.move_speed
        }
    }
}

impl Collideable for Rectangle {
    fn get_bb(&self) -> BoundingBox {
        BoundingBox {
            min_x: self.x,
            min_y: self.y,
            max_x: self.x + self.w,
            max_y: self.y + self.h,
        }
    }
}
