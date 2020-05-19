mod vec3;
mod color;
mod sphere;
mod plane;
mod object;

use rand::prelude::*;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

use vec3::{ Vec3, Float, Norm, Dot };
use color::{Color, new_color, Lights, Solid, Checker};
use sphere::new_sphere;
use plane::new_plane;
use object::Intersect;

const SUBSAMPLE : u32 = 25;
const REFLECTIONS : u32 = 10;
const BLACK : Color = Vec3(0.0, 0.0, 0.0);

type Hit<'a> = (&'a Intersect, Vec3);
type OutResult = Result<(), std::io::Error>;

#[derive(Debug,Clone,Copy)]
struct Ray {
    orig: Vec3,
    dir: Vec3,
}

struct Scene<'a> {
    lights: Lights,
    objects: Vec<&'a Intersect>
}

struct Camera {
    width: u32,
    height: u32,
    depth: u32
}

fn main() {
    let filename = "out.ppm";
    let cam = Camera { width:1200, height:800, depth:700 };
    let m1 = Solid { color: new_color(255.0, 100.0, 100.0),
                     specular: (8.0, 0.4), reflection: 0.75 };
                     
    let m2 = Checker { colors: (new_color(150.0, 150.0, 225.0),
                                new_color(200.0, 200.0, 300.0)),
                       uv: 10, specular: (4.0, 0.4), reflection: 0.5 };
    let s1 = new_sphere(Vec3(-2.0, -5.0, 30.0), 5.0, &m1);
    let s2 = new_sphere(Vec3(8.0, 1.0, 30.0), 5.0, &m1);
    let s3 = new_sphere(Vec3(8.0, -10.0, 20.0), 5.0, &m1);
    let s4 = new_sphere(Vec3(-3.0, 2.0, 10.0), 3.0, &m1);
    let p1 = new_plane(Vec3(0.0, 3.001, 0.0), Vec3(0.0, 1.0, 0.0), &m2);
    let scene = Scene {
        lights: Lights { dir: (Vec3(-0.5, -1.0, -0.75)).normalized(),
                         ambiant: 0.2,
                         bg: new_color(20.0, 20.0, 30.0) },
        objects: vec![&s1, &s2, &s3, &s4, &p1]
    };
    println!("rendering...");
    let now = Instant::now();
    let frame = render_frame(scene, &cam);
    let time = now.elapsed().as_millis() as f32 / 1000.0;
    println!("done in {} seconds.", time);
    match write_image(&frame_to_image(&frame), &cam, filename) {
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

fn write_image(img: &Vec<u8>, cam: &Camera, filename: &str) -> OutResult {
    let header = format!("P6 {} {} 255\n", cam.width, cam.height);    
    let mut file = File::create(filename)?;
    file.write(header.as_bytes())?;
    file.write(&img)?;
    Ok(())
}

fn render_frame(scene: Scene, cam: &Camera) -> Vec<Color>{
    let mut frame = vec![BLACK ; (cam.width*cam.height) as usize];
    let mut rng = rand::thread_rng();
    let orig = Vec3(0.0, 0.0, 0.0);
    let center = Vec3(-(cam.width as Float) / 2.0,
                      -(cam.height as Float) /2.0,
                      cam.depth as Float);
    let mut pc = 0;
    for y in 0..cam.height {
        for x in 0..cam.width {
            let dir = Vec3(x as Float, y as Float, 0.0) + center;
            let mut col = BLACK;
            for _ in 0..SUBSAMPLE {
                let rnd = Vec3(rng.gen(), rng.gen(), 0.0);
                let ray = Ray { orig: orig, dir: dir + rnd };
                col = col + render_pixel(&scene, ray, REFLECTIONS);
            }
            frame[(x + y * cam.width) as usize] = col / SUBSAMPLE as Float;
        }
        let p = 100*y/cam.height;
        if p/10 > pc/10 {
            println!("{}%", p);
        }
        pc = p;
    }
    println!("100%");
    frame
}

fn render_pixel(scene: &Scene, ray: Ray, n: u32) -> Color {
    match cast_ray(&scene.objects, ray) {
        None => scene.lights.bg,
        Some((obj, p)) => {
            let surfp = obj.get_surface(&p);
            let np = obj.get_normal(&p);
            let ray2 = Ray { orig: surfp, dir: scene.lights.dir};
            let m = obj.get_material();
            let col0 = m.get_color(&p, &np, &scene.lights); 
            let col = cast_ray(&scene.objects, ray2)
                .map_or(col0, |_| col0 * scene.lights.ambiant );
            let reflection = m.get_reflection();
            if n > 0 && reflection > 0.0 {
                let ray3 = Ray { orig: surfp,
                                 dir: reflect(p-ray.orig, np) };
                let col2 = render_pixel(&scene, ray3, n-1);
                col * (1.0-reflection) + col2 * reflection
            } else {
                col
            }
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(&n) * 2.0
}

fn cast_ray<'a>(scene: &Vec<&'a Intersect>, ray: Ray) -> Option<Hit<'a>> {
    scene.iter().fold(None, |res, obj|
                      match obj.intersect(&ray.orig, &ray.dir) {
                          None => res,
                          Some(z) => {
                              match res {
                                  None => Some((*obj, z)),
                                  Some((_,i)) if z < i => Some((*obj, z)),
                                  _ => res
                              }
                          }
                      }).map(| (obj, z) | (obj, ray.dir * z + ray.orig))
}
