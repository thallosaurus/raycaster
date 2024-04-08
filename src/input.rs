use std::{
    borrow::BorrowMut,
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{console, js_sys::Math, HtmlCanvasElement, KeyboardEvent, MouseEvent};

use crate::{player::Player, window};

pub fn init_keyboard(canvas: &HtmlCanvasElement, p: Rc<RefCell<Player>>) -> Result<(), JsValue> {
    let document = window().document().unwrap();
    //let p = *p.bo;
    //let rc = p.clone();
    //let mut p = Box::new(p);

    //let pl = Rc::clone(&rc);
    {
        let func = move |_event: MouseEvent| {
            console::log_1(&JsValue::from_str("Mousemove"));

            //let pl = p.to_owned();
            //rc.borrow_mut().set_angle(Math::random() * 10.0);
            //p.set_angle(Math::random() * 10.0);
            p.borrow_mut().set_angle(10.0);
            
            //pl.borrow()
            
            //let debug = format!("{:?}", pl);
            
            //console::log_1(&JsValue::from_str(debug.as_str()));
            
            /*let rc = Rc::clone(p);
            let mut rc = rc.borrow_mut();
            let rc = rc.deref_mut();
            rc.set_angle(event.movement_x() as f64);*/
        };

        let mousemove_closure = Closure::<dyn FnMut(MouseEvent)>::new(func);
        
        document.add_event_listener_with_callback(
            "mousemove",
            mousemove_closure.as_ref().unchecked_ref(),
        )?;

        mousemove_closure.forget();
    }

    /*{
        let pl = rc.clone();
        let keydown_closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            //let p = p.clone();
            //web_sys::console::log_1(&JsValue::from_str(e.key().as_str()));
            //p.set_angle(event.movement_x() as f64);

            let mut pp = pl.borrow();

            if e.key() == "w" {
                pp.borrow_mut().set_speed(2);
            }
            if e.key() == "s" {
                //web_sys::console::log_1(&JsValue::from_str("Arrow Down"));
                pp.borrow_mut().set_speed(-2);
            }
            let debug = format!("{:?}", pl);
            console::log_1(&JsValue::from_str(debug.as_str()));
        });

        document.add_event_listener_with_callback(
            "keydown",
            keydown_closure.as_ref().unchecked_ref(),
        )?;
        keydown_closure.forget();
    }*/

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

    keyup_closure.forget();
    */

    Ok(())
}
