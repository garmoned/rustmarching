pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Matrix {
    pub mat: [f32; 9],
}

impl Matrix {
    pub fn new_rot(rad: f32) -> Self {
        let mat = [
            rad.cos(),
            0.0,
            rad.sin(),

            0.0,
            1.0,
            0.0,

            -rad.sin(),
            0.0,
            rad.cos(),
        ];
        Self { mat: mat }
    }


    pub fn new_rot_z(rad: f32) -> Self {
        let mat = [
            rad.cos(),
            -rad.sin(),
            0.0,

            rad.sin(),
            rad.cos(),
            0.0,

            0.0,
            0.0,
            1.0,
        ];
        Self { mat: mat }
    }


    pub fn mult(&self,vec:&Vec3)-> Vec3 {
        return Vec3::new(
            vec.x*self.mat[0] + vec.y*self.mat[1] + vec.z*self.mat[2], 
            vec.x*self.mat[3] + vec.y*self.mat[4] + vec.z*self.mat[5],
            vec.x*self.mat[6] + vec.y*self.mat[7] + vec.z*self.mat[8],
        )
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn dist_between(&self, location: &Vec3) -> f32 {
        let xdist = self.x - location.x;
        let ydist = self.y - location.y;
        let zdist = self.z - location.z;
        let dist = xdist.powf(2.0) + ydist.powf(2.0) + zdist.powf(2.0);
        return dist.powf(0.5);
    }

    pub fn unit(&self) -> Vec3 {
        let mag = self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0);
        let mag = mag.powf(0.5);
        let uvec = Vec3::new(self.x / mag, self.y / mag, self.z / mag);
        return uvec;
    }

    pub fn mag(&self) -> f32 {
        return self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0);
    }

    pub fn add(&self, vec: &Vec3, mult: f32) -> Vec3 {
        let new_vec = Vec3::new(
            self.x + vec.x * mult,
            self.y + vec.y * mult,
            self.z + vec.z * mult,
        );
        return new_vec;
    }

    pub fn modu(&self, constant: f32) -> Vec3 {
        let new_vec = Vec3::new(
            modulo(self.x, constant),
            modulo(self.y, constant),
            modulo(self.z, constant),
        );
        return new_vec;
    }

    pub fn dot(&self, vec: &Vec3) -> f32 {
        return self.x * vec.x + self.y * vec.y + self.z * vec.z;
    }

    pub fn mult(&self, c: f32) -> Vec3 {
        return Vec3::new(self.x * c, self.y * c, self.z * c);
    }
}

pub fn modulo(num: f32, c: f32) -> f32 {
    return (num % c + c) % c;
}
