use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent};

use crate::{player::Player, window};

fn init_keyboard(canvas: HtmlCanvasElement, mut p: &Player) -> Result<(), JsValue> {
    let document = window().document().unwrap();

    let p_copy = p.clone();
    /*let mousemove_closure = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
        p_copy.set_angle(event.movement_x() as f64);
    });

    document.add_event_listener_with_callback(
        "mousemove",
        mousemove_closure.as_ref().unchecked_ref(),
    )?;*/

    /*let click_closure = Closure::new(|| {

    })*/

    /*

    let mut kd_copy = p.clone();
    let keydown_closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        //let p = p.clone();
        //web_sys::console::log_1(&JsValue::from_str(e.key().as_str()));
        if e.key() == "w" {
            kd_copy.set_speed(2);
        }

        if e.key() == "s" {
            //web_sys::console::log_1(&JsValue::from_str("Arrow Down"));
            kd_copy.set_speed(-2);
        }
    });

    let keyup_closure = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        if e.key() == "w" || e.key() == "s" {
            p.set_speed(0);
        }
    });

    let mousemove_closure = Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
        p.set_angle(event.movement_x() as f64);
    });

    document
        .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback(
        "mousemove",
        mousemove_closure.as_ref().unchecked_ref(),
    )?;

    keydown_closure.forget();
    keyup_closure.forget();
    mousemove_closure.forget();*/
    Ok(())
}