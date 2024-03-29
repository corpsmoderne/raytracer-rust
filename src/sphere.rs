use crate::vec3::{Vec3, Float, Dot, Norm};
use crate::color::Material;
use crate::raytracer::Intersect;
use std::option::Option;

pub struct Sphere {
    pos : Vec3,
    radius : Float,
    r2 : Float,
    mat : Box<dyn Material>
}

pub fn new_sphere(p : Vec3, r : Float, m : Box<dyn Material>) -> Box<Sphere> {
    Box::new(Sphere { pos:p, radius:r, r2: r.powf(2.0), mat:m })
}
    
enum Solution {
    Zero,
    One(Float),
    Two(Float, Float)
}

fn solve_quadratic(a : Float, b : Float, c : Float) -> Solution {
    let discr = b * b - 4.0 * a * c;
    match discr {
        x if x < 0.0 => Solution::Zero,
        x if x > 0.0 => {
            let q = if b > 0.0 {
                -0.5 * (b + discr.sqrt())
            } else {
                -0.5 * (b - discr.sqrt())
            };
            Solution::Two(q/a, c/q)
        },
        _ => Solution::One(-0.5 * b / a)
    }
    
}

impl Intersect for Sphere {
    fn intersect(&self, orig : &Vec3, dir : &Vec3) -> Option<Float> {
        let l = *orig - self.pos;
        let a = dir.dot(dir);
        let b = 2.0 * l.dot(dir);
        let c = l.dot(&l) - self.r2;
        
        match solve_quadratic(a, b, c) {
            Solution::One(x) if x > 0.0 => Some(x),
            Solution::Two(x, y) if y < 0.0 && x > 0.0 => Some(x),
            Solution::Two(x, y) if x < 0.0 && y > 0.0 => Some(y),
            Solution::Two(x, y) if x > 0.0 && y > 0.0 => Some(x.min(y)),
            _ => None
        }
    }

    fn get_surface(&self, v : &Vec3) -> Vec3 {
        let n = (*v - self.pos).normalized();
        self.pos + n * self.radius * 1.0001
    }

    fn get_normal(&self, v : &Vec3) -> Vec3 {
        (*v - self.pos).normalized()
    }
    
    fn get_material(&self) -> &dyn Material {
        &*self.mat
    }
}

