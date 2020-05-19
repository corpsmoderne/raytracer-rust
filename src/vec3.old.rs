use std::fmt;
use std::ops;

pub type Float = f32;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x : Float,
    pub y : Float,
    pub z : Float
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "V({},{},{})", self.x, self.y, self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + rhs.x,
               y: self.y + rhs.y,
               z: self.z + rhs.z }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x - rhs.x,
               y: self.y - rhs.y,
               z: self.z - rhs.z }
    }
}

impl ops::Mul<Float> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Float) -> Vec3 {
        Vec3 { x: self.x * rhs,
               y: self.y * rhs,
               z: self.z * rhs }
    }
}

impl ops::Div<Float> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Float) -> Vec3 {
        Vec3 { x: self.x / rhs,
               y: self.y / rhs,
               z: self.z / rhs }
    }
}

pub trait Dot {
    fn dot(&self, rhs : &Vec3) -> Float;
}

impl Dot for Vec3 {
    fn dot(&self, v : &Vec3) -> Float {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

pub trait Norm {
    fn norm(&self) -> Float;
    fn normalized(self) -> Vec3;    
}

impl Norm for Vec3 {
    fn norm(&self) -> Float {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
    fn normalized(self) -> Vec3 {
        self / self.norm()
    }
}
    
