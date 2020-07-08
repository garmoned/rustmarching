use super::linal;

pub struct Shape {

    location:linal::Vec3,
    color:linal::Vec3,
    radius:f32
}

impl Shape {
    pub fn new(location:linal::Vec3, color:linal::Vec3,radius:f32,shapeType:&str) -> Self{
        Self{
            location:location,
            color:color,
            radius:radius
        }
    }

    pub fn dist_from(&self,location:&linal::Vec3)->f32{
        return &self.location.dist_between(&location.modu(2.0)) - self.radius;
    }
}


