use super::linal::Vec3;
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
    pub x_rotation:f32,
    pub y_rotation:f32,
    pub new_changes:bool
}



impl Camera {
    pub fn new() -> Self {
        Self{
            dist_from_cam:0.0,
            z_trans:0.0,
            x_trans:0.0,
            y_trans:0.0,
            x_rotation:0.0,
            y_rotation:0.0,
            new_changes:true
        }
    }

}


pub fn update_translation(tvec:&Vec3){
    let mut data = CAM_STATE.lock().unwrap();

    let new_x = tvec.x + data.x_trans;
    let new_y = tvec.y + data.y_trans;
    let new_z = tvec.z + data.z_trans;


    *data = Arc::new(Camera {
        z_trans:new_z,
        x_trans:new_x,
        y_trans:new_y,
        new_changes:true,
        ..*data.clone()
    });
    
}


pub fn upated_changes(){
    let mut data = CAM_STATE.lock().unwrap();
    *data = Arc::new(Camera {
        new_changes:true,
        ..*data.clone()
    });
}