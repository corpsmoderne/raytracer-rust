mod vec3;
mod color;
mod sphere;
mod plane;
mod object;
mod render;
mod raytracer;

use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::sync::Arc;

use vec3::{ Vec3, Norm };
use color::{Color, new_color, Lights, Solid, Checker};
use sphere::new_sphere;
use plane::new_plane;
use raytracer::*;
use render::render_frame;

type OutResult = Result<(), std::io::Error>;

fn make_scene() -> Scene {
    let m1 = Solid { color: new_color(255.0, 100.0, 100.0),
                     specular: (8.0, 0.4), reflection: 0.75 };
                     
    let m2 = Checker { colors: (new_color(150.0, 150.0, 225.0),
                                new_color(200.0, 200.0, 300.0)),
                       uv: 10, specular: (4.0, 0.4), reflection: 0.5 };
    Scene {
        lights: Lights { dir: (Vec3(-0.5, -1.0, -0.75)).normalized(),
                         ambiant: 0.2,
                         bg: new_color(20.0, 20.0, 30.0) },
        objects: vec![
            new_sphere(Vec3(-2.0, -5.0, 30.0), 5.0, Box::new(m1.clone())),
            new_sphere(Vec3(8.0, 1.0, 30.0), 5.0, Box::new(m1.clone())),
            new_sphere(Vec3(8.0, -10.0, 20.0), 5.0, Box::new(m1.clone())),
            new_sphere(Vec3(-3.0, 2.0, 10.0), 3.0, Box::new(m1.clone())),
            new_plane(Vec3(0.0, 3.001, 0.0), Vec3(0.0, 1.0, 0.0), Box::new(m2))
        ]
    }
}

fn main() {
    let filename = "out.ppm";
    let cam = Camera { width:2400, height:1600, depth:1400 };
    let scene = make_scene();
    println!("rendering...");
    let now = Instant::now();
    let frame = render_frame(Arc::new(scene), Arc::new(cam));
    let time = now.elapsed().as_millis() as f32 / 1000.0;
    println!("done in {} seconds.", time);
    match write_image(frame_to_image(&frame), cam, filename) {
        Ok(()) => println!("{} written.", filename),
        Err(err) => println!("Error: {}", err)
    }
}

fn frame_to_image(frame: &Vec<Color>) -> Vec<u8> {
    let mut buffer = vec![0 as u8 ; frame.len() * 3];    
    for (i, c) in frame.iter().enumerate() {
        buffer[i*3+0] = c.0.min(255.0).max(0.0) as u8;
        buffer[i*3+1] = c.1.min(255.0).max(0.0) as u8;
        buffer[i*3+2] = c.2.min(255.0).max(0.0) as u8;
    }
    buffer
}

fn write_image(img: Vec<u8>, cam: Camera, filename: &str) -> OutResult {
    let header = format!("P6 {} {} 255\n", cam.width, cam.height);    
    let mut file = File::create(filename)?;
    file.write(header.as_bytes())?;
    file.write(&img)?;
    Ok(())
}
