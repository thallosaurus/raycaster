mod map;
pub mod player;
mod ray;
mod render;
mod utils;
mod input;

use std::{cell::RefCell, ops::Deref, rc::Rc};

use input::init_keyboard;
use map::GameMap;
use player::Player;
use ray::get_rays;
use wasm_bindgen::prelude::*;

use web_sys::CanvasRenderingContext2d;

pub static CELL_SIZE: u32 = 32;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    //create canvas
    let window = web_sys::window().expect("no global window found; are we running in a browser?");
    let document = window
        .document()
        .expect("no document found in the current window");
    let body = document.body().expect("document doesn't have a body");

    let canvas = document.create_element("canvas")?;
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_id("main_canvas");
    body.append_child(&canvas)?;

    let mut player = Rc::new(RefCell::new(Player::new()));

    init_keyboard(&canvas, player)?;

    // Gather 2d Context
    let context = canvas
        .get_context("2d")
        .expect("couldn't get context from canvas");
    let context = context
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    //Initial Resize
    let inner_width = get_screen_width() as u32;
    let inner_height = get_screen_height() as u32;
    canvas.set_width(inner_width);
    canvas.set_height(inner_height);

    let canvas_copy = canvas.clone();

    let resize_callback = Closure::<dyn Fn()>::new(move || {
        //let window = web_sys::window().expect("no global window found in this context");
        let inner_width = get_screen_width() as u32;
        let inner_height = get_screen_height() as u32;
        canvas_copy.set_width(inner_width);
        canvas_copy.set_height(inner_height);
    });
    window.add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())?;
    resize_callback.forget();

    let performance = window
        .performance()
        .expect("performance should be available");
    let runtime_now = performance.now();

    // map config
    let map = GameMap::default();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let player = Rc::clone(&player);
    *g.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
        let window = web_sys::window().expect("no global window found in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let now = performance.now();

        game_loop(&context, &player.borrow(), &map, now - runtime_now);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn game_loop(ctx: &CanvasRenderingContext2d, player: &Player, map: &GameMap, _ts: f64) {
    render::clear(ctx);
    //player.get_mut().move_player();

    let window = window();
    let canvas_dimensions = (
        window.inner_width().unwrap().as_f64().unwrap() as u32,
        window.inner_height().unwrap().as_f64().unwrap() as u32,
    );
    let rays = get_rays(&player, canvas_dimensions, map);
    render::render_scene(ctx, &player, &rays);
    render::draw_minimap(ctx, player, map, &rays, 1.0)
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn get_fov() -> f64 {
    60.0_f64.to_radians()
}

fn get_screen_height() -> f64 {
    window().inner_height().unwrap().as_f64().unwrap()
}

fn get_screen_width() -> f64 {
    window().inner_width().unwrap().as_f64().unwrap()
}
