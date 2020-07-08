use super::linal::Vec3;
use super::shape::Shape;
use super::camera_state::Camera;
use std::sync::Arc;

pub fn ray_march(screen: & mut Vec<u8>,height:usize,
     width:usize,shapes:&Vec<Shape>,camera:&Arc<Camera>) {
    
    for x in 0..width {
        for y in 0..height {

            let mut hit = false;

            let xpos = (x as f32 - ((width / 2) as f32)) / height as f32;
            let ypos = (y as f32 - ((height / 2) as f32)) / height as f32;

            let mut raypos = Vec3::new(xpos+camera.x_trans, ypos+camera.y_trans, 0.0 + camera.z_trans);

            let dir = Vec3::new(xpos, ypos, 1.0).unit();
            
            let mut step = 0;

            let i = (y * width + x) * 4;

            while step < 20 && !hit{

                let steplength = lowest_distance(shapes,&raypos);
                 
                if steplength < 0.1 {
                    hit = true;
                }
                raypos = raypos.add(&dir, steplength);
                
                step += 1;
            }

            if hit {

                set_pixel_color(screen, i, (255.0*(1.0-(step as f32 /50.0)))as u8, (192.0*(1.0-(step as f32 /100.0)))as u8,(203.0*(1.0-(step as f32 /100.0)))as u8);
            
            }else{
                set_pixel_color(screen, i,0, 0,0);
            }


        }
    }

}

fn lowest_distance(shapes:&Vec<Shape>,vec:&Vec3) -> f32{

    let mut lowest_distance = shapes[0].dist_from(vec);

    for shape in shapes{

        let new_dist = shape.dist_from(vec);

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
