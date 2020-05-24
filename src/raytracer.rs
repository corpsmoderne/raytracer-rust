pub const SUBSAMPLE : u32 = 25;
pub const REFLECTIONS : u32 = 10;
pub const BLACK : Color = Vec3(0.0, 0.0, 0.0);
pub const THREADS : u32 = 8;

use crate::vec3::{ Vec3 };
use crate::color::{Color, Lights};
use crate::object::Intersect;

#[derive(Clone,Copy)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub depth: u32
}

pub struct Scene {
    pub lights: Lights,
    pub objects: Vec<Box<dyn Intersect>>
}
