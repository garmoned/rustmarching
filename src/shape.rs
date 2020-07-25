use super::linal::{Matrix, Vec3};
use std::cmp::{max, min};

pub enum Shape {
    Sphere(Vec3, f32),
    Spheres(Vec3, f32),
    Cube(Vec3, f32),
    Plane(Vec3,Vec3)
}

pub fn dist_from(shape: &Shape, location: &Vec3) -> f32 {
    match shape {
        Shape::Sphere(loc, rad) => sphere_dist(loc, location, rad),
        Shape::Spheres(loc, rad) => spheres_dist(loc, location, rad),
        Shape::Cube(loc, len) => cube_dist(loc, location, len),
        Shape::Plane(norm,loc) => plane_dist(norm,loc,location),
    }
}

fn plane_dist(norm:&Vec3,p_loc:&Vec3,loc:&Vec3) -> f32 {
    let dist = p_loc.add(loc, -1.0);
    let cast = dist.dot(&norm.unit()).max(dist.dot(&norm.unit().mult(-1.0)));
    return cast;
}

fn sphere_dist(sphere_loc: &Vec3, location: &Vec3, radius: &f32) -> f32 {

    return sphere_loc.dist_between(&location) - radius;
}

fn spheres_dist(sphere_loc: &Vec3, location: &Vec3, radius: &f32) -> f32 {
    return sphere_loc.dist_between(&location.modu(2.0)) - radius;
}


fn cube_dist(cube_loc: &Vec3, loc: &Vec3, len: &f32) -> f32 {

    let nloc = Vec3::new(loc.x, loc.y, loc.z); 

    let nloc = Matrix::new_rot(loc.y*2.0).mult(&nloc);
    //let nloc = Matrix::new_rot_z(loc.y/5.0).mult(&nloc);

    let dist = Vec3::new(
        (nloc.x - cube_loc.x).abs() - len,
        (nloc.y - cube_loc.y).abs() - len*3.0,
        (nloc.z - cube_loc.z).abs() - len,
    );

    return dist.x.max(dist.z.max(0.0))
    .max(-sphere_dist(cube_loc,&Vec3::new(nloc.x, nloc.y%0.60, nloc.z),&0.75));
}

fn pinski(pos:&Vec3) -> f32{
    let iters = 3;
    let mut n = 0;
    let scale = 0.5;

    let mut npos = Vec3::new(pos.x, pos.y, pos.z);

    while n < iters {
        npos = fold(&Vec3::new(1.0,1.0,0.0),&npos);
        npos = fold(&Vec3::new(1.0,0.0,1.0),&npos);
        npos = fold(&Vec3::new(0.0,1.0,1.0),&npos);
        npos = npos.mult(scale).add(&Vec3::new(1.0,1.0,1.0),scale - 1.0);
        n+=1;
    }

    return npos.mag()*scale.powf(-n as f32);
}

fn fold(norm:&Vec3,vec:&Vec3) -> Vec3{
    let norm = norm.unit();
    let mut nvec = Vec3::new(vec.x, vec.y, vec.z);
    let t = norm.dot(vec); 
    if t < 0.0 {
        nvec = nvec.add(&norm.mult(t), -2.0);
    }
    return nvec;
}