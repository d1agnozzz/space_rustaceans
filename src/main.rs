use collision::Collideable;
use macroquad::{prelude::*, window::Conf};

mod collision;
mod objects;

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Rustaceans".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

use objects::{Rectangle, ScreenBorders};

#[macroquad::main(window_conf)]
async fn main() {
    let mut square = Rectangle {
        // x: screen_width()/2.0,
        // y: screen_height() - 30.0,
        x: screen_width() / 2.0,
        y: screen_height() - 30.0,
        w: 15.0,
        h: 15.0,
        move_speed: 3.0,
        boosted_move_speed: 5.0,
        is_boosted: false,
        color: WHITE,
        vx: 0.0,
        vy: 0.0,
    };

    let screen_borders = ScreenBorders {
        min_x: 0.0,
        min_y: 0.0,
        max_x: screen_width(),
        max_y: screen_height(),
    };

    loop {
        clear_background(BLACK);

        square.is_boosted = is_key_down(KeyCode::LeftShift);

        if is_key_down(KeyCode::A) {
            square.vx = -square.speed();
        }
        if is_key_down(KeyCode::D) {
            square.vx = square.speed();
        }
        if is_key_down(KeyCode::W) {
            square.vy = -square.speed();
        }
        if is_key_down(KeyCode::S) {
            square.vy = square.speed();
        }
        square.move_by_velocity(&screen_borders);
        square.clear_velocity();

        if square.get_bb().is_inside(&screen_borders.get_bb()) {
            square.color = WHITE;
        } else {
            square.color = RED;
        }

        square.draw();

        next_frame().await;
    }
}
