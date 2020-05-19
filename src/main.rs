mod vec3;
mod color;
mod sphere;
mod plane;
mod object;

use std::f32::consts::PI;
use rand::prelude::*;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

use vec3::{ Vec3, Float, Norm, Dot };
use color::{Color, new_color, Lights, Material, Solid};
use sphere::new_sphere;
use plane::new_plane;
use object::Intersect;

const SUBSAMPLE : u32 = 50;
const REFLECTIONS : u32 = 50;
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
    let cam = Camera { width:1200, height:800, depth:700 };
    let s1 = new_sphere(Vec3(-2.0, -5.0, 30.0), 5.0,
                        new_color(255.0, 100.0, 100.0));
    let s2 = new_sphere(Vec3(8.0, 1.0, 30.0), 5.0,
                        new_color(255.0, 100.0, 100.0));
    let s3 = new_sphere(Vec3(8.0, -10.0, 20.0), 5.0,
                        new_color(255.0, 100.0, 100.0));
    let s4 = new_sphere(Vec3(-3.0, 2.0, 10.0), 3.0,
                        new_color(255.0, 100.0, 100.0));
    let p1 = new_plane(Vec3(0.0, 3.0, 0.0), Vec3(0.0, 1.0, 0.0),
                       new_color(100.0, 100.0, 150.0));
    let scene = Scene {
        lights: Lights { dir: (Vec3(-0.5, -1.0, -0.75)).normalized(),
                         ambiant: 0.2,
                         bg: new_color(10.0, 10.0, 15.0) },
        objects: vec![&s1, &s2, &s3, &s4, &p1]
    };
    let mut frame = vec![BLACK ; (cam.width*cam.height) as usize];
    
    println!("rendering...");
    let now = Instant::now();
    render(&mut frame, scene, &cam);
    let time = now.elapsed().as_millis() as f32 / 1000.0;
    match write_image(&frame, &cam, "out.ppm") {
        Ok(()) => println!("out.ppm written, done in {} seconds.", time),
        Err(err) => println!("Error: {}", err)
    }
}

fn write_image(frame: &Vec<Color>, cam: &Camera, filename: &str) -> OutResult {
    let mut buffer = File::create(filename)?;
    let header = format!("P6 {} {} 255\n", cam.width, cam.height);
    buffer.write(header.as_bytes())?;
    for c in frame {
        let buf = [ c.0.min(255.0).max(0.0) as u8,
                    c.1.min(255.0).max(0.0) as u8,
                    c.2.min(255.0).max(0.0) as u8 ];
        buffer.write(&buf)?;
    }
    Ok(())
}

fn render(frame: &mut Vec<Color>, scene: Scene, cam: &Camera) {
    let mut rng = rand::thread_rng();
    let orig = Vec3(0.0, 0.0, 0.0);
    let center = Vec3(-(cam.width as Float) / 2.0,
                      -(cam.height as Float) /2.0,
                      cam.depth as Float);
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
    }    
}

fn render_pixel(scene: &Scene, ray: Ray, n: u32) -> Color {
    match cast_ray(&scene.objects, ray) {
        None => scene.lights.bg,
        Some((obj, p)) => {
            let ray2 = Ray { orig: obj.get_surface(&p), dir: scene.lights.dir};
            let spec = ((obj.get_normal(&p)*-1.0)
                        .dot(&scene.lights.dir).acos() /
                        (2.0*PI*1.0/3.0)).powf(8.0);
            let col0 = obj.get_color() * (0.2 * spec + scene.lights.ambiant);
            let col = match cast_ray(&scene.objects, ray2) {
                None => col0,
                _ => col0 * scene.lights.ambiant
            };
            if n > 0 {
                let ray3 = Ray { orig: obj.get_surface(&p),
                                 dir: reflect(p-ray.orig, obj.get_normal(&p)) };
                let col2 = render_pixel(&scene, ray3, n-1);
                col * 0.5 + col2 * 0.5
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
    scene.iter().fold(None, |res, obj| match obj.intersect(&ray.orig, &ray.dir) {
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
