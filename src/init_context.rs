use web_sys::{HtmlCanvasElement};
use wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use super::linal::Vec3;
use super::camera_state::update_translation;

const MOVE_SPEED:f32 = 0.3;

pub fn init_context() -> Result<web_sys::CanvasRenderingContext2d,JsValue>{

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let ctx:web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")?.unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    


    attack_mouse_down_handler(&canvas)?;

    Ok(ctx)
}


fn attack_mouse_down_handler(canvas:&HtmlCanvasElement) -> Result<(),JsValue> {

    let handler = move |event:web_sys::KeyboardEvent| {
        handle_key(&event.key())
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())?;
    handler.forget();
    Ok(())
}


fn handle_key(key:&str){
    match key {
        "w" => update_translation(&Vec3::new(0.0,0.0,MOVE_SPEED)),
        "a" => update_translation(&Vec3::new(-MOVE_SPEED,0.0,0.0)),
        "s" => update_translation(&Vec3::new(0.0,0.0,-MOVE_SPEED)),
        "d" => update_translation(&Vec3::new(MOVE_SPEED,0.0,0.0)),
         _ => super::log(key)
    }

}



