use crate::vec3::{ Vec3, Float, Dot };
use crate::color::Color;
use crate::raytracer::*;

use rand::prelude::*;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;

type Hit<'a> = (&'a dyn Intersect, Vec3);

pub struct Line(u32, Vec<Vec3>);
unsafe impl Send for Line {}

#[derive(Debug,Clone,Copy)]
struct Ray {
    orig: Vec3,
    dir: Vec3,
}

pub fn render_frame(scene: Arc<Scene>, cam: Arc<Camera>) -> Vec<Color>{
    let mut frame = vec![BLACK ; (cam.width*cam.height) as usize];
    let (tx, rx) = mpsc::channel();
    for i in 0..THREADS {
        render_slice(scene.clone(), cam.clone(), i, tx.clone());
    }
    let mut pc = 0;
    for n in 0..cam.height {
        let Line(y, colors) = rx.recv().unwrap();
        frame[(y*cam.width) as usize .. (cam.width+y*cam.width) as usize]
            .clone_from_slice(&colors);
        let new_pc = 100*n/cam.height;
        if new_pc/10 > pc/10 {
            println!("{}%", new_pc);
        }
        pc = new_pc;
    }
    println!("100%");
    frame
}

fn render_slice<'a>(scene: Arc<Scene>, cam: Arc<Camera>,
                    id: u32, tx: Sender<Line>) {
    thread::spawn(move || {
        println!("spwan thread #{}", id);
        let mut rng = rand::thread_rng();
        let orig = Vec3(0.0, 0.0, 0.0);
        let center = Vec3(-(cam.width as Float) / 2.0,
                          -(cam.height as Float) /2.0,
                          cam.depth as Float);
        for yy in 0..cam.height/THREADS {
            let y = yy*THREADS+id;
            let mut line = vec![BLACK ; cam.width as usize];
            for x in 0..cam.width {
                let dir = Vec3(x as Float, y as Float, 0.0) + center;
                let mut col = BLACK;
                for _ in 0..SUBSAMPLE {
                    let rnd = Vec3(rng.gen(), rng.gen(), 0.0);
                    let ray = Ray { orig: orig, dir: dir + rnd };
                    col = col + render_pixel(&scene, ray, REFLECTIONS);
                }
                line[x as usize] = col / SUBSAMPLE as Float;
            }
            tx.send(Line(y, line)).unwrap();
        }
    });
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

fn cast_ray<'a>(objs: &'a [Box<dyn Intersect>], ray: Ray)
                -> Option<Hit<'a>> {
    objs.iter().fold(None,
                     |res, obj|
                     match obj.intersect(&ray.orig, &ray.dir) {
                         None => res,
                         Some(z) => {
                             match res {
                                 None => Some((obj.as_ref(), z)),
                                 Some((_,i)) if z < i =>
                                     Some((obj.as_ref(), z)),
                                 _ => res
                             }
                         }
                     }).map(| (obj, z) | (&*obj, ray.dir * z + ray.orig))
}
