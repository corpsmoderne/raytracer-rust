use crate::vec3::{Vec3, Float, Dot, Norm};
use crate::color::Material;
use crate::object::Intersect;
use std::option::Option;

#[derive(Clone)]
pub struct Plane<'a> {
    pos: Vec3,
    normal: Vec3,
    mat: &'a Material
}

pub fn new_plane(p : Vec3, n : Vec3, m : &Material) -> Plane {
    Plane { pos: p, normal: n, mat: m }
}

impl<'a> Intersect for Plane<'a> {
    fn intersect(&self, orig : &Vec3, dir : &Vec3) -> Option<Float> {
        let denom = dir.dot(&self.normal);
        let p = self.pos - orig.clone();
        if denom < 0.000001 {
            None
        } else {
            let pt = p.dot(&self.normal) / denom;
            Some(pt)
        }
    }

    fn get_normal(&self, _v : &Vec3) -> Vec3 {
        self.normal.normalized() * -1.0
    }
    
    fn get_surface(&self, v : &Vec3) -> Vec3 {
        v.clone() + self.normal.normalized() * -0.0001
    }

    fn get_material(&self) -> &Material {
        self.mat
    }

}

