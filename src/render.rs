use crate::vec3::{ Vec3, Float, Dot };
use crate::color::Color;
use crate::raytracer::*;

use rand::prelude::*;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

type Hit<'a> = (&'a dyn Intersect, Vec3);

pub struct Line(u32, Vec<Vec3>);

#[derive(Debug,Clone,Copy)]
struct Ray {
    orig: Vec3,
    dir: Vec3,
}

pub fn render_frame(scene: &Scene) -> Vec<Color> {
    let cam = scene.camera;
    let mut frame = vec![BLACK ; (cam.width*cam.height) as usize];
    let (tx, rx) = mpsc::channel();
    thread::scope(|s| {
	for i in 0..THREADS {
	    let tx_clone = tx.clone();
	    s.spawn(move || render_slice(scene, i, tx_clone));
	}
    
	let mut pc = 0;
	for n in 0..cam.height {
            let Line(y, colors) = rx.recv().unwrap();
            frame[(y*cam.width) as usize .. (cam.width+y*cam.width) as usize]
		.clone_from_slice(&colors);
            pc = update_pc(pc, cam.height, n);
	}
    });
    println!("100%");    
    frame
}

fn update_pc(pc : u32, total : u32, n : u32) -> u32 {
    let new_pc = 100*n/total;
    if new_pc/10 > pc/10 {
        println!("{}%", new_pc);
    }
    new_pc
}

fn render_slice(scene: &Scene, id: u32, tx: Sender<Line>) {
    let cam = scene.camera;
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
                let ray = Ray { orig, dir: dir + rnd };
                col = col + render_pixel(scene, ray, scene.reflections);
                }
            line[x as usize] = col / SUBSAMPLE as Float;
        }
        tx.send(Line(y, line)).unwrap();
    }
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
                let col2 = render_pixel(scene, ray3, n-1);
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

fn cast_ray(objs: &[Box<dyn Intersect>], ray: Ray) -> Option<Hit> {
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
                     }).map(| (obj, z) | (obj, ray.dir * z + ray.orig))
}
