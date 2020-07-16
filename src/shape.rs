use super::linal::Vec3;
use std::cmp::{max,min};

pub enum Shape {
    Sphere(Vec3,f32),
    Spheres(Vec3,f32),
    Cube(Vec3,f32)
}


pub fn dist_from(shape:&Shape,location:&Vec3) -> f32{
    match shape {
        Shape::Sphere(loc,rad) => sphere_dist(loc, location, rad),
        Shape::Spheres(loc,rad) => spheres_dist(loc, location, rad),
        Shape::Cube(loc,len) => cube_dist(loc,location,len)
    }
}

fn sphere_dist(sphere_loc:&Vec3,location:&Vec3,radius:&f32)->f32{
    return sphere_loc.dist_between(&location) - radius;
}

fn spheres_dist(sphere_loc:&Vec3,location:&Vec3,radius:&f32)->f32{
    return sphere_loc.dist_between(&location.modu(2.0)) - radius;
}

fn cube_dist(cube_loc:&Vec3,loc:&Vec3,len:&f32) -> f32 {

    let dist = Vec3::new(
        (loc.x - cube_loc.x).abs()  - len, 
        (loc.y -  cube_loc.y).abs()  - len, 
        (loc.z - cube_loc.z).abs() - len
    );

   

    return dist.x.max(dist.y.max(dist.z).max(0.0));

}