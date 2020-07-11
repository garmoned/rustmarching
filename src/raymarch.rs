use super::linal::Vec3;
use super::Shape::Shape;
use super::Shape::dist_from;
use super::camera_state::Camera;
use std::sync::Arc;


pub fn cast_ray(
    
    height:usize,
    width:usize,shapes:&Vec<Shape>,
    camera:&Arc<Camera>,
    x:usize,y:usize) -> [u8;3]{

    let mut hit = false;
    let xpos = (x as f32 - ((width / 2) as f32)) / height as f32;
    let ypos = (y as f32 - ((height / 2) as f32)) / height as f32;
    let mut raypos = Vec3::new(xpos+camera.x_trans, ypos+camera.y_trans, 0.0 + camera.z_trans);
    let dir = Vec3::new(xpos, ypos, 1.0).unit();
    let mut step = 0;
    let i = (y * width + x) * 4;
    while step < 50 && !hit{

        let steplength = lowest_distance(shapes,&raypos);
         
        if steplength < 0.01 {
            hit = true;
        }
        raypos = raypos.add(&dir, steplength);
        
        step += 1;
    }

    if hit {

        let r = 255.0*(1.0-(step as f32 /50.0));
        let g = 192.0*(1.0-(step as f32 /100.0));
        let b = 203.0*(1.0-(step as f32 /100.0));

        return[r as u8 ,g as u8,b as u8]
    
    }else{
        return[0, 0,0];
    }
}

fn lowest_distance(shapes:&Vec<Shape>,vec:&Vec3) -> f32{

    let mut lowest_distance = dist_from(&shapes[0],vec); 

    for shape in shapes {

        let new_dist = dist_from(shape,vec);

        if new_dist < lowest_distance {
            lowest_distance = new_dist;
        }
    }

    return lowest_distance;
}

fn set_pixel_color(screen: &mut Vec<u8>, i: usize, r: u8, g: u8, b: u8) {
    screen[i + 0] = r;
    screen[i + 1] = g;
    screen[i + 2] = b;
    screen[i + 3] = 255;
}
