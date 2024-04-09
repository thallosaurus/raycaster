use std::{
    cell::RefCell,
    rc::Rc,
};

use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{console, HtmlCanvasElement, KeyboardEvent, MouseEvent};

use crate::{player::Player, window};

pub fn init_keyboard(canvas: Rc<HtmlCanvasElement>, p: Rc<RefCell<Player>>) -> Result<(), JsValue> {
    let document = window().document().unwrap();

    //capture mouse
    {
        let c = canvas.clone();
        let canvas_click = Closure::<dyn FnMut()>::new(move || {
            c.request_pointer_lock();
        });

        canvas.add_event_listener_with_callback("click", canvas_click.as_ref().unchecked_ref())?;
        canvas_click.forget();
    }

    //set angle
    {
        let p = p.clone();
        let func = move |event: MouseEvent| {
            event.prevent_default();
            p.borrow_mut().set_angle(event.movement_x() as f64);

            let debug = format!("{:?}", p);

            console::log_1(&JsValue::from_str(debug.as_str()));
        };

        let mousemove_closure = Closure::<dyn FnMut(MouseEvent)>::new(func);

        document.add_event_listener_with_callback(
            "mousemove",
            mousemove_closure.as_ref().unchecked_ref(),
        )?;

        mousemove_closure.forget();
    }

    //Keyboard Input
    {
        let p = p.clone();
        let keydown_closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            e.prevent_default();
            let mut p = p.borrow_mut();

            if e.key() == "w" {
                p.set_speed(2);
            }
            if e.key() == "s" {
                p.set_speed(-2);
            }
            let debug = format!("{:?}", p);
            console::log_1(&JsValue::from_str(debug.as_str()));
        });

        document.add_event_listener_with_callback(
            "keydown",
            keydown_closure.as_ref().unchecked_ref(),
        )?;
        keydown_closure.forget();
    }

    //keyboard release event
    {
        let p = p.clone();
        let keyup_closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            e.prevent_default();
            let mut p = p.borrow_mut();
            if e.key() == "w" || e.key() == "s" {
                p.set_speed(0);
            }
        });

        document.add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())?;
        keyup_closure.forget();
    }

    Ok(())
}
