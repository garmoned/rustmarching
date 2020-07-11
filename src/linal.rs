pub struct Vec3 {
   pub x:f32,
   pub y:f32,
   pub z:f32,
}


impl Vec3 {
    pub fn new(x:f32,y:f32,z:f32) -> Self{
        Self{
            x:x,
            y:y,
            z:z
        }
    }

    pub fn dist_between(&self,location:&Vec3) -> f32{
        let xdist = self.x - location.x;
        let ydist = self.y - location.y;
        let zdist = self.z - location.z;
        let dist = xdist.powf(2.0) + ydist.powf(2.0) + zdist.powf(2.0);
        return dist.powf(0.5);
    }

    pub fn unit(&self) -> Vec3{
        let mag = self.x.powf(2.0) +self.y.powf(2.0) + self.z.powf(2.0);
        let mag = mag.powf(0.5);
        let uvec = Vec3::new(self.x/mag,self.y/mag, self.z/mag);
        return uvec;
    }

    pub fn add(&self,vec:&Vec3,mult:f32) -> Vec3 {  
        let newVec = Vec3::new(self.x+vec.x*mult, self.y+vec.y*mult, self.z+vec.z*mult);
        return newVec;
    }

    pub fn modu(&self,constant:f32) -> Vec3{
        let new_vec = Vec3::new(
        modulo(self.x, constant),
        modulo(self.y, constant), 
        modulo(self.z, constant));
        return new_vec;
    }

}

pub fn modulo(num:f32,c:f32) -> f32{
    return (num % c + c) % c
}