use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use web_sys::js_sys::Math;
use crate::{get_fov, map::GameMap, player::Player, CELL_SIZE};

static PI: f64 = 3.14159265359_f64;

pub fn get_rays(player: &RefCell<Player>, canvas_dimensions: (u32, u32), map: &GameMap) -> Vec<Collision> {
    let p = player.borrow();

    let initial_angle = (p.angle - get_fov()) / 2.0_f64;
    let number_of_rays = canvas_dimensions.0;
    let angle_step = get_fov() / number_of_rays as f64;
    (0..number_of_rays)
        .into_iter()
        .map(|v| {
            let angle = initial_angle + (v as f64) * angle_step;
            let ray = cast_ray(angle, &p, map);
            ray
        })
        .collect()
}

fn cast_ray(angle: f64, player: &Player, map: &GameMap) -> Collision {
    
    let v_collision = get_v_collision(angle, player, map);
    let h_collision = get_h_collision(angle, player, map);

    if h_collision.distance >= v_collision.distance {
        v_collision
    } else {
        h_collision
    }
}

fn get_v_collision(angle: f64, player: &Player, map: &GameMap) -> Collision {
    let right = Math::abs(Math::floor(angle - PI / 2.0));
    let cell_size = CELL_SIZE as f64;
    //let player = player.as_ref().borrow();

    let first_x = if right > 0.0 {
         Math::floor(player.x / cell_size) * cell_size + cell_size
    } else {
        Math::floor(player.x / cell_size) * cell_size
    };

    let first_y = player.y + (first_x - player.x) * Math::tan(angle);

    let x_a = if right > 0.0 {
        cell_size
    } else {
        cell_size * -1.0
    };

    let y_a = x_a * Math::tan(angle);

    let mut wall: u8 = 0;
    let mut next_x = first_x;
    let mut next_y = first_y;
    
    while wall == 1 {
        let cell_x = if right > 0.0 {
            Math::floor(next_x / cell_size) as u8
        } else {
            (Math::floor(next_x / cell_size) - 1.0) as u8
        };

        let cell_y = Math::floor(next_y / cell_size) as u8;

        if map.out_of_bounds(cell_x, cell_y) {
            break;
        }

        if let Some(_) = map.get_xy(cell_x, cell_y) {
            next_x += x_a;
            next_y += y_a;
        }
    }

    Collision {
        angle,
        distance: distance(player.x, player.y, next_x, next_y),
        vertical: true
    }
}

fn get_h_collision(angle: f64, player: &Player, map: &GameMap) -> Collision {
    let up = Math::abs(Math::floor(angle / PI) % 2.0);
    let cell_size = CELL_SIZE as f64;
    //let player = player.as_ref().borrow();

    let first_y = if up > 0.0 {
        Math::floor(player.y / cell_size) * cell_size
    } else {
        Math::floor(player.y / cell_size) * cell_size + cell_size
    };

    let first_x = player.x + (first_y - player.y) / Math::tan(angle);

    let y_a = if up > 0.0 {
        -cell_size
    } else {
        cell_size
    };

    let x_a = y_a / Math::tan(angle);

    let mut wall = 0;
    let mut next_x = first_x;
    let mut next_y = first_y;

    while wall == 1 {
        let cell_x = (Math::floor(next_x / cell_size)) as u8;
        let cell_y = if up > 0.0 {
            (Math::floor(next_y / cell_size) - 1.0) as u8
        } else {
            (Math::floor(next_y / cell_size)) as u8
        };

        if map.out_of_bounds(cell_x, cell_y) {
            break;
        }

        if let Some(_) = map.get_xy(cell_x, cell_y) {
            next_x += x_a;
            next_y += y_a;
        }

        /*wall = map.get_xy(cell_x, cell_y);

        if wall > 0 {
            next_x += x_a;
            next_y += y_a;
        }*/
    }

    Collision {
        angle,
        distance: distance(player.x, player.y, next_x, next_y),
        vertical: false
    }
}

pub struct Collision {
    pub angle: f64,
    pub distance: f64,
    pub vertical: bool,
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    /*
    function distance(x1, y1, x2, y2) {
        return Math.sqrt(Math.pow(x2 - x1, 2) + Math.pow(y2 - y1, 2));
    }
    */
    Math::sqrt(Math::pow(x2 - x1, 2.0) + Math::pow(y2 - y1, 2.0))
}