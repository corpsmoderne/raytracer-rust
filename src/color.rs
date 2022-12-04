use std::f32::consts::PI;
use crate::vec3::{Float, Vec3, Dot};

pub type Color = Vec3;

pub fn new_color(r: Float, g: Float, b: Float) -> Color {
    Vec3(r, g, b)
}

#[derive(Debug,Clone,Copy)]
pub struct Lights {
    pub dir: Vec3,
    pub ambiant: Float,
    pub bg: Color
}

pub trait Material : Sync + Send {
    fn get_color(&self, p : &Vec3, n : &Vec3, lights : &Lights) -> Color;
    fn get_reflection(&self) -> Float;
    fn get_specular(&self, specular: (Float, Float),
                    n: &Vec3, light: Vec3) -> Float {
        specular.1 *
            ((*n * -1.0).dot(&light).acos() /
             (2.0 * PI * 1.0/3.0)).powf(specular.0)
    }
    fn clone_box(&self) -> Box<dyn Material>;
}

#[derive(Clone)]
pub struct Solid {
    pub color : Color,
    pub specular : (Float,Float),
    pub reflection : Float
}

impl Material for Solid {
    fn get_color(&self, _p : &Vec3, n : &Vec3, lights : &Lights) -> Color {
        let spec = self.get_specular(self.specular, n, lights.dir);
        self.color * (spec + lights.ambiant)
    }    
    fn get_reflection(&self) -> Float {
        self.reflection
    }
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct Checker {
    pub colors : (Color,Color),
    pub uv : i32,
    pub specular : (Float,Float),
    pub reflection : Float        
}

impl Material for Checker {
    fn get_color(&self, p : &Vec3, n : &Vec3, lights : &Lights) -> Color {
        let spec = self.get_specular(self.specular, n, lights.dir);
        // I'm pretty sure there's a bug in here...
        let check =
            (p.0 as i32 % self.uv * 2 <
             if p.0 > 0.0 { self.uv } else { -self.uv }) ^
            (p.1 as i32 % self.uv * 2 <
             if p.1 > 0.0 { self.uv } else { -self.uv }) ^
            (p.2 as i32 % self.uv * 2 <
             if p.2 > 0.0 { self.uv } else { -self.uv });
        let color = if check {
            self.colors.0
        } else {
            self.colors.1
        };
        color * (self.specular.1 * spec + lights.ambiant)
    }
    
    fn get_reflection(&self) -> Float {
        self.reflection
    }

    fn clone_box(&self) -> Box<dyn Material>{
        Box::new(self.clone())
    }
}
