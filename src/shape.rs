use super::linal::Vec3;


pub enum Shape {
    Sphere(Vec3,f32),
    Spheres(Vec3,f32)
}


pub fn dist_from(shape:&Shape,location:&Vec3) -> f32{
    match shape {
        Shape::Sphere(loc,rad) => sphere_dist(loc, location, rad),
        Shape::Spheres(loc,rad) => spheres_dist(loc, location, rad)
    }
}

fn sphere_dist(sphere_loc:&Vec3,location:&Vec3,radius:&f32)->f32{
    return sphere_loc.dist_between(&location) - radius;
}

fn spheres_dist(sphere_loc:&Vec3,location:&Vec3,radius:&f32)->f32{
    return sphere_loc.dist_between(&location.modu(2.0)) - radius;
}
