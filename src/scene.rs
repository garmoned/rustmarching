use super::shape::Shape;
use super::camera_state::Camera;

use std::sync::Arc;
use web_sys::CanvasRenderingContext2d;

use super::raymarch::ray_march;
use wasm_bindgen::JsValue;

use super::init_context::init_context;

pub struct Scene {
    shapes:Vec<Shape>,
    ctx:CanvasRenderingContext2d
}

impl Scene{

    pub fn new(shapes:Vec<Shape>) -> Self{

        let ctx = init_context().unwrap();
        
        Self{
            shapes:shapes,
            ctx:ctx
        }
    }


    pub fn render(&self,camera:Arc<Camera>,screen:&mut Vec<u8>){
        ray_march(screen,300,400,&self.shapes,&camera);
        let img_data = create_img_data(screen).unwrap();
        self.ctx.put_image_data(&img_data,0.0,0.0).unwrap();

    }

}

fn create_img_data(screen: & mut Vec<u8>) -> Result<web_sys::ImageData,JsValue>{

    let slice:&mut [u8] = &mut screen[..];
    let clamped = wasm_bindgen::Clamped(slice);
    let img_data = web_sys::ImageData::new_with_u8_clamped_array(clamped,400);
    img_data
}
