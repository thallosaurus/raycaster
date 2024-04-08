use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{js_sys::Math, CanvasRenderingContext2d};

use crate::{get_screen_height, map::GameMap, player::Player, ray::Collision, CELL_SIZE};

fn fix_fish_eye(distance: f64, angle: f64, player_angle: f64) -> f64 {
    let diff = angle - player_angle;
    distance * Math::cos(diff)
}

pub fn render_scene(ctx: &CanvasRenderingContext2d, player: &Player, rays: &Vec<Collision>) {
    let cell_size = CELL_SIZE as f64;
    for (i, ray) in rays.iter().enumerate() {
        let distance = fix_fish_eye(ray.distance, ray.angle, player.angle);
        let wall_height = ((cell_size * 5.0) / distance) * 277.0;

        let wall_color = if ray.vertical { "lightgreen" } else { "green" };

        ctx.set_fill_style(&JsValue::from_str(wall_color));
        ctx.fill_rect(
            i as f64,
            get_screen_height() / 2.0 - wall_height / 2.0,
            1.0,
            wall_height,
        );

        ctx.set_fill_style(&JsValue::from_str("red"));
        ctx.fill_rect(
            i as f64,
            get_screen_height() / 2.0 + wall_height / 2.0,
            1.0,
            get_screen_height() / 2.0 - wall_height / 2.0,
        );

        ctx.set_fill_style(&JsValue::from_str("blue"));
        ctx.fill_rect(
            i as f64,
            0.0,
            1.0,
            get_screen_height() / 2.0 - wall_height / 2.0,
        )
    }
}

pub fn clear(ctx: &CanvasRenderingContext2d) {
    let canvas = ctx.canvas().unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    ctx.set_fill_style(&JsValue::from_str("black"));

    let w = canvas.width();
    let h = canvas.height();
    ctx.fill_rect(0.0, 0.0, w as f64, h as f64);
}

pub fn draw_minimap(
    ctx: &CanvasRenderingContext2d,
    player: &Player,
    map: &GameMap,
    rays: &Vec<Collision>,
    scale: f64,
) {
    let cell_size = scale * (CELL_SIZE as f64);
    let (pos_x, pos_y) = (0.0, 0.0);

    {
        for y in 0..map.height {
            for x in 0..map.width {

                if let Some(_) = map.get_xy(x, y) {
                    ctx.set_fill_style(&JsValue::from_str("grey"));
                    ctx.fill_rect(
                        pos_x + (x as f64) * cell_size,
                        pos_y + (y as f64) * cell_size,
                        cell_size,
                        cell_size,
                    );
                }
            }
        }
    }

    {
        ctx.set_fill_style(&JsValue::from_str("blue"));
        ctx.fill_rect(
            pos_x + player.x * scale * 10.0 / 2.0,
            pos_y + player.y * scale * 10.0 / 2.0,
            10.0,
            10.0,
        );
    }

    {
        ctx.set_stroke_style(&JsValue::from_str("blue"));
        ctx.begin_path();
        ctx.move_to(player.x * scale, player.y * scale);
        ctx.line_to(
            (player.x + Math::cos(player.angle) * 20.0) * scale,
            (player.y + Math::sin(player.angle) * 20.0) * scale,
        );
        ctx.close_path();
        ctx.stroke();
    }

    {
        ctx.set_stroke_style(&JsValue::from_str("yellow"));
        rays.iter().for_each(|ray| {
            ctx.begin_path();
            ctx.move_to(player.x * scale, player.y * scale);
            ctx.line_to(
                (player.x + Math::cos(ray.angle) * ray.distance) * scale,
                (player.y + Math::sin(ray.angle) * ray.distance) * scale,
            );
            ctx.close_path();
            ctx.stroke();
        });
    }
}
