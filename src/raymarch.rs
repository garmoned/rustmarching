use super::linal::Vec3;
use super::shape::Shape;
use super::shape;
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

            while step < 100 && !hit{
                let steplength = lowest_distance(shapes,&raypos);
                if steplength < 0.001 {
                    hit = true;
                }
                raypos = raypos.add(&dir, steplength);
                step += 1;
            }
            set_pixel_color(screen, hit, raypos, i,shapes,step);
        }
    }

}

fn lowest_distance(shapes:&Vec<Shape>,vec:&Vec3) -> f32{

    let mut lowest_distance = shape::dist_from(&shapes[0], &vec);

    for shape in shapes{

        let new_dist = shape::dist_from(shape,&vec);

        if new_dist < lowest_distance {
            lowest_distance = new_dist;
        }
    }

    return lowest_distance;
}

fn set_pixel_color(screen: &mut Vec<u8>, hit:bool, pos:Vec3, i:usize, shapes: &Vec<Shape>,steps:u8) {

    let mut r:u8 = 0;
    let mut g:u8 = 0;
    let mut b:u8 = 0;

    let eps = 0.01;

    let l_pos = Vec3::new(0.0,-1.0,1.0);
    let amb = 75;

    if hit {

        //phong lighting
        //create norm vect from sampling around hit point
        //to create gradient
        let norm_vec = Vec3::new(
            lowest_distance(shapes, &Vec3::new(pos.x+eps, pos.y, pos.z).
            add(&Vec3::new(pos.x-eps, pos.y, pos.z), -1.0)),
            lowest_distance(shapes, &Vec3::new(pos.x, pos.y+eps, pos.z).
            add(&Vec3::new(pos.x, pos.y-eps, pos.z), -1.0)),
            lowest_distance(shapes, &Vec3::new(pos.x, pos.y, pos.z+eps).
            add(&Vec3::new(pos.x, pos.y, pos.z-eps), -1.0)),
        ).unit();

        let lvec = l_pos.add(&pos, -1.0).unit().mult(-1.0); 


        let reflection = lvec.add(
            &norm_vec.mult((lvec.dot(&norm_vec) * 2.0) /(norm_vec.dot(&norm_vec))), 
            -1.0).unit();

        let mut spec = reflection.dot(&pos.mult(-1.0).unit()).powf(10.0) * 100.0;
        let mut diffusion = norm_vec.dot(&lvec) * 155.0;

        if diffusion < 0.0{
            diffusion = 0.0;
        }

        if spec < 0.0 {
            spec = 0.0;
        }

        r += spec as u8;
        g += spec as u8;
        b += diffusion as u8;
        b += spec as u8;

        r += amb + (steps/100) * 30;
        

    }

    screen[i + 0] = r;
    screen[i + 1] = g;
    screen[i + 2] = b;
    screen[i + 3] = 255;
}
