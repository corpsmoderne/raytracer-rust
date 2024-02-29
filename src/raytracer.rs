pub const SUBSAMPLE : u32 = 25;
pub const REFLECTIONS : u32 = 10;
pub const BLACK : Color = Vec3(0.0, 0.0, 0.0);
pub const THREADS : u32 = 8;

use std::option::Option;
use crate::color::{Color, Lights};
use crate::vec3::{ Vec3, Float };
use crate::color::{Material};

#[derive(Clone,Copy,PartialEq,Eq)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub depth: u32
}

impl Camera {
    pub fn get_frame_size(&self) -> usize {
	(self.width * self.height) as usize
    }
}

pub struct Scene {
    pub camera: Camera,
    pub lights: Lights,
    pub objects: Vec<Box<dyn Intersect>>,
    pub reflections: u32
}

pub trait Intersect : Sync + Send {
    fn intersect(&self, orig : &Vec3, dir : &Vec3) -> Option<Float>;
    fn get_surface(&self, v : &Vec3) -> Vec3;
    fn get_normal(&self, v : &Vec3) -> Vec3;
    fn get_material(&self) -> &dyn Material;
}

