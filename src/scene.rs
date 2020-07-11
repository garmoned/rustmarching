use super::camera_state::Camera;
use super::Shape::Shape;
use futures_channel::oneshot;
use std::sync::Arc;
use web_sys::{CanvasRenderingContext2d,ImageData};
use super::raymarch;
//use rayon::prelude::*;
use super::init_context::init_context;
use super::pool::WorkerPool;
use rayon::prelude::*;
use wasm_bindgen::JsValue;


pub struct Scene {
    shapes: Vec<Shape>,
    ctx: CanvasRenderingContext2d,
}

impl Scene {
    pub fn new(shapes: Vec<Shape>) -> Self {
        let ctx = init_context().unwrap();

        Self {
            shapes: shapes,
            ctx: ctx,
        }
    }

    pub  async fn render(
        &self,
        camera: Arc<Camera>,
        pool: &WorkerPool,
        concurrency: usize,
    ){


        let mut screen:Vec<u8> = vec![0;4 * 400*300];

        let base = screen.as_ptr() as usize;
        let len = screen.len();
        
       let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency)
        .spawn_handler(|thread| Ok(pool.run(|| thread.run()).unwrap()))
        .build()
        .unwrap();
      

        let imgdata = create_img_data(&mut screen).unwrap();
        let (tx, rx) = oneshot::channel();
        pool.run(move || {
            thread_pool.install(|| {
                screen
                    .par_chunks_mut(4)
                    .enumerate()
                    .for_each(|(i, chunk)| {
                        chunk[0] = 255;
                    });
            });
            drop(tx.send(screen));
        }).unwrap();
        
     
        
        self.ctx.put_image_data(&imgdata,0.0,0.0).unwrap();
    }
}


fn create_img_data(screen: & mut Vec<u8>) -> Result<web_sys::ImageData,JsValue>{

    let slice:&mut [u8] = &mut screen[..];
    let clamped = wasm_bindgen::Clamped(slice);
    let img_data = web_sys::ImageData::new_with_u8_clamped_array(clamped,400);
    img_data
}

