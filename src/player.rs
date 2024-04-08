use wasm_bindgen::prelude::*;
use web_sys::js_sys::{wasm_bindgen, Math};

use crate::CELL_SIZE;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub speed: i32
}

#[wasm_bindgen]
impl Player {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let cell_size = CELL_SIZE as f64;

        Self {
            x: 2.0,
            y: 2.0,
            angle: 0.0_f64.to_radians(),
            speed: 2
        }
    }
    pub fn move_player(mut self) {
        self.x += Math::cos(self.angle) * self.speed as f64;
        self.y += Math::sin(self.angle) * self.speed as f64;
    }

    pub fn set_speed(mut self, speed: i32) {
        self.speed = speed;
    }

    pub fn set_angle(mut self, angle: f64) {
        self.angle += angle.to_radians()
    }
}