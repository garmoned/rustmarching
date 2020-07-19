use super::camera_state::Camera;
use super::linal::{Vec3,Matrix};
use super::shape;
use super::shape::Shape;
use std::sync::Arc;
use std::cmp::{max};

pub fn ray_march(
    screen: &mut Vec<u8>,
    height: usize,
    width: usize,
    shapes: &Vec<Shape>,
    camera: &Arc<Camera>,
) {
    let rot_mat = Matrix::new_rot(camera.y_rotation);
    let cam_trans = Vec3::new(camera.x_trans,camera.y_trans,camera.z_trans);

    for x in 0..width {
        for y in 0..height {
            let mut hit = false;

            let xpos = (x as f32 - ((width / 2) as f32)) / height as f32;
            let ypos = (y as f32 - ((height / 2) as f32)) / height as f32;

            let raypos = Vec3::new(
                xpos,
                ypos,
                1.0
            );

            let raypos = rot_mat.mult(&raypos);
            
            let dir = raypos.unit();

            let mut raypos = raypos.add(&cam_trans, 1.0);

            let mut step = 0;
            let i = (y * width + x) * 4;

            while step < 100 && !hit {
                let steplength = lowest_distance(shapes, &raypos);
                if steplength < 0.01 {
                    hit = true;
                }
                raypos = raypos.add(&dir, steplength);
                step += 1;
            }
            set_pixel_color(screen, hit, raypos, i, shapes, step);
        }
    }
}

fn lowest_distance(shapes: &Vec<Shape>, vec: &Vec3) -> f32 {
    let mut lowest_distance = shape::dist_from(&shapes[0], &vec);

    for shape in shapes {
        let new_dist = shape::dist_from(shape, &vec);

        if new_dist < lowest_distance {
            lowest_distance = new_dist;
        }
    }

    return lowest_distance;
}

fn set_pixel_color(
    screen: &mut Vec<u8>,
    hit: bool,
    pos: Vec3,
    i: usize,
    shapes: &Vec<Shape>,
    steps: u8,
) {
    let mut r: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;

    let eps = 0.01;

    let mut lights = vec![];

    lights.push(Vec3::new(0.0, -2.0, 1.0));
 
 

    let amb = 20;

    if hit {
     
        let norm_vec = Vec3::new(
            lowest_distance(shapes, &Vec3::new(pos.x + eps, pos.y, pos.z))
                - lowest_distance(shapes, &Vec3::new(pos.x - eps, pos.y, pos.z)),
            lowest_distance(shapes, &Vec3::new(pos.x, pos.y + eps, pos.z))
                - lowest_distance(shapes, &Vec3::new(pos.x, pos.y - eps, pos.z)),
            lowest_distance(shapes, &Vec3::new(pos.x, pos.y, pos.z + eps))
                - lowest_distance(shapes, &Vec3::new(pos.x, pos.y, pos.z - eps)),
        )
        .unit();

        for l_pos in lights {
            let lvec = l_pos.add(&pos, -1.0).unit();

            let reflection = lvec
                .add(
                    &norm_vec.mult((lvec.dot(&norm_vec) * 2.0) / (norm_vec.dot(&norm_vec))),
                    -1.0,
                )
                .unit();

            let spec = reflection.dot(&pos.unit()).powf(2.0) * 30.0;
            let diffusion = max((norm_vec.dot(&lvec) * 90.0) as u8,0);

            r += spec as u8;
            g += spec as u8;
            b += amb;
            b += diffusion as u8;
            b += spec as u8;
        }
        r += amb;
    }

    screen[i + 0] = r;
    screen[i + 1] = g;
    screen[i + 2] = b;
    screen[i + 3] = 255;
}
