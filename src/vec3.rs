use std::ops;

pub type Float = f32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3(pub Float, pub Float, pub Float);

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<Float> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Float) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Div<Float> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Float) -> Vec3 {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)        
    }
}

pub trait Dot {
    fn dot(&self, rhs : &Vec3) -> Float;
}

impl Dot for Vec3 {
    fn dot(&self, v : &Vec3) -> Float {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }
}

pub trait Norm {
    fn norm(&self) -> Float;
    fn normalized(self) -> Vec3;    
}

impl Norm for Vec3 {
    fn norm(&self) -> Float {
        (self.0.powf(2.0) + self.1.powf(2.0) + self.2.powf(2.0)).sqrt()
    }
    fn normalized(self) -> Vec3 {
        self / self.norm()
    }
}
    
