use super::camera_state::Camera;
use super::linal::{Matrix, Vec3};
use super::shape;
use super::shape::Shape;
use std::cmp::min;
use std::sync::Arc;

pub fn ray_march(
    screen: &mut Vec<u8>,
    height: usize,
    width: usize,
    shapes: &Vec<Shape>,
    camera: &Arc<Camera>,
) {
    let rot_mat = Matrix::new_rot(camera.y_rotation);
    let cam_trans = Vec3::new(camera.x_trans, camera.y_trans, camera.z_trans);

    for x in 0..width {
        for y in 0..height {
            let mut hit = false;

            let xpos = (x as f32 - ((width / 2) as f32)) / height as f32;
            let ypos = (y as f32 - ((height / 2) as f32)) / height as f32;

            let raypos = Vec3::new(xpos, ypos, 1.0);

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

    let eps = 0.001;

    let mut lights = vec![];

    lights.push(Vec3::new(-5.0, -2.0, 1.0));
    lights.push(Vec3::new(5.0, -2.0, -1.0));

    let amb = 100;

    let dif = 100.0/lights.len() as f32;
    let spc = 115.0/lights.len() as f32;

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

            let spec = reflection.dot(&pos.unit()).powf(2.0) * dif;
            let diffusion = (norm_vec.dot(&lvec) * spc).max(0.0);
            let shad = shadow(&pos, &lvec, shapes);
            let diffusion = diffusion;
            let spec = spec * shad;

            r += spec as u8;
            g += spec as u8;
            b += diffusion as u8;
            b += spec as u8;
        }

        //ambient lighting
        r+= 50;
    }

    screen[i + 0] = min(r, 255);
    screen[i + 1] = min(g, 255);
    screen[i + 2] = min(b, 255);
    screen[i + 3] = 255;
}

fn shadow(pos: &Vec3, lvec: &Vec3, shapes: &Vec<Shape>) -> f32 {
    let mut shad_pos = Vec3::new(pos.x, pos.y, pos.z).add(lvec, 0.05);

    let mut dist = lowest_distance(shapes, &shad_pos);
    let mut travel = 0.0;

    while travel < 5.0 {
        shad_pos = shad_pos.add(lvec, dist);
        dist = lowest_distance(shapes, &shad_pos);

        if dist < 0.01 {
            return 0.0;
        }

        travel += dist;
    }

    return 1.0;
}
