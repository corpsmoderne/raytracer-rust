use std::f32::consts::PI;
use crate::vec3::{Float, Vec3, Dot};

pub type Color = Vec3;

pub fn new_color(r: Float, g: Float, b: Float) -> Color {
    Vec3(r,g,b)
}

pub struct Lights {
    pub dir: Vec3,
    pub ambiant: Float,
    pub bg: Color
}

pub trait Material {
    fn get_color(self, p : &Vec3, n : &Vec3, lights : &Lights) -> Color;
}

pub struct Solid {
    pub color : Color,
    pub specular : (Float,Float)
}

impl Material for Solid {
    fn get_color(self, _p : &Vec3, n : &Vec3, lights : &Lights) -> Color {
        let spec = ((n.clone()*-1.0).dot(&lights.dir).acos() /
                    (2.0 * PI * 1.0/3.0)).powf(self.specular.0);
        self.color * (self.specular.1 * spec + lights.ambiant)
    }
}

pub struct Checker {
    pub colors : (Color,Color),
    pub specular : (Float,Float)
}
