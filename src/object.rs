use std::option::Option;
use crate::vec3::{ Vec3, Float };
use crate::color::{Color, Material};

pub trait Intersect {
    fn intersect(&self, orig : &Vec3, dir : &Vec3) -> Option<Float>;
    fn get_surface(&self, v : &Vec3) -> Vec3;
    fn get_normal(&self, v : &Vec3) -> Vec3;
    fn get_color(&self) -> Color;
}

