use js_sys::*;
use std::f64;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::rc::Rc;

#[macro_use]
extern crate lazy_static;

mod camera_state;
mod common_funcs;
mod init_context;
mod linal;
mod raymarch;
mod scene;
mod shape;

use linal::Vec3;
use shape::Shape;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_f32(a: f32);
}

#[wasm_bindgen(start)]

pub fn start() {
    let perform = web_sys::window()
        .expect("should have window here")
        .performance()
        .expect("should have performance");

    let mut shape_vec = Vec::new();

    shape_vec.push(Shape::Cube(Vec3::new(0.0, 0.0, 0.0), 0.543));
    shape_vec.push(Shape::Plane(Vec3::new(0.0,-1.0,0.0),Vec3::new(0.0, 1.0, 0.0)));
   // shape_vec.push(Shape::Sphere(Vec3::new(-1.0, 0.0, 1.5), 0.543));
    //shape_vec.push(Shape::Sphere(Vec3::new(-1.5, 0.0, 1.5), 0.3));

    let sc = scene::Scene::new(shape_vec);

    let mut screen: Vec<u8> = Vec::new();

    for _i in 0..4 * 400 * 300 {
        screen.push(0)
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let fps_throttle = (1000.0 / 15.0) as f64;

    let mut last_draw_time: f64 = -1.0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let camera = camera_state::get_curr_cam_state();

        let curr_time = perform.now();

        if curr_time >= last_draw_time + fps_throttle {
            last_draw_time = curr_time;
            let c = curr_time as f32 / 3000.0;

            sc.render(camera, &mut screen);
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    /* let camera = camera_state::get_curr_cam_state();
    sc.render(camera,&mut screen);*/
}
