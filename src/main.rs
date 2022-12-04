mod vec3;
mod color;
mod sphere;
mod plane;
mod render;
mod raytracer;
mod expr;
mod scene;

use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::env;

use color::Color;
use raytracer::*;
use render::render_frame;
use scene::load_scene;

type OutResult = Result<(), std::io::Error>;

fn main() {
    let args: Vec<String> = env::args().collect();    
    if args.len() < 2 {
        println!("needs one argument");
        return;
    }
    let filename = "out.ppm";
    if let Some(scene) = load_scene(args[1].as_str()) {
        let cam = scene.camera;
        println!("rendering...");
        let now = Instant::now();
        let frame = render_frame(&scene); //Arc::new(scene));
        let time = now.elapsed().as_millis() as f32 / 1000.0;
        println!("done in {} seconds.", time);
        match write_image(frame_to_image(&frame), cam, filename) {
            Ok(()) => println!("{} written.", filename),
            Err(err) => println!("Error: {}", err)
        }
    } else {
        println!("unable to load scene.");
    }
}

fn frame_to_image(frame: &Vec<Color>) -> Vec<u8> {
    let mut buffer = vec![0_u8 ; frame.len() * 3];    
    for (i, c) in frame.iter().enumerate() {
        buffer[i*3  ] = c.0.min(255.0).max(0.0) as u8;
        buffer[i*3+1] = c.1.min(255.0).max(0.0) as u8;
        buffer[i*3+2] = c.2.min(255.0).max(0.0) as u8;
    }
    buffer
}

fn write_image(img: Vec<u8>, cam: Camera, filename: &str) -> OutResult {
    let header = format!("P6 {} {} 255\n", cam.width, cam.height);    
    let mut file = File::create(filename)?;
    _ = file.write(header.as_bytes())?;
    _ = file.write(&img)?;
    Ok(())
}
