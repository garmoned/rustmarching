use super::linal::{Vec3,Matrix};
use std::sync::Arc;
use std::sync::Mutex;


lazy_static! {
    static ref CAM_STATE: Mutex<Arc<Camera>> = Mutex::new(Arc::new(Camera::new()));
}

pub fn get_curr_cam_state() -> Arc<Camera>{
    CAM_STATE.lock().unwrap().clone()
}


pub struct Camera {
    pub dist_from_cam:f32,
    pub z_trans:f32,
    pub x_trans:f32,
    pub y_trans:f32,
    pub y_rotation:f32,
}



impl Camera {
    pub fn new() -> Self {
        Self{
            dist_from_cam:0.0,
            z_trans:-1.0,
            x_trans:0.0,
            y_trans:0.0,
            y_rotation:0.0,
        }
    }


    pub fn _test(){
        super::log("pressed button");
    }

}

pub fn update_rotation(rot_amt:f32){
    let mut data = CAM_STATE.lock().unwrap();

    let new_rot = data.y_rotation + rot_amt;

    *data = Arc::new(Camera {
        y_rotation:new_rot,
        ..*data.clone()
    });
    
}

pub fn update_translation(tvec:&Vec3){
    let mut data = CAM_STATE.lock().unwrap();

    let mat = Matrix::new_rot(data.y_rotation);
    let vec = mat.mult(tvec);

    let new_x = vec.x + data.x_trans;
    let new_y = vec.y + data.y_trans;
    let new_z = vec.z + data.z_trans;


    *data = Arc::new(Camera {
        z_trans:new_z,
        x_trans:new_x,
        y_trans:new_y,
        ..*data.clone()
    });
    
}