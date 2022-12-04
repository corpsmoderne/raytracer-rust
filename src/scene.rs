use std::fs;
use crate::expr::{parse_all, tokenize, Expr, car, cdr};
use crate::vec3::{ Vec3, Norm, Float };
use crate::color::{new_color, Color, Lights, Solid, Checker, Material};
use crate::sphere::{Sphere, new_sphere};
use crate::plane::{Plane, new_plane};
use std::collections::HashMap;
use crate::raytracer::*;

fn get_float(expr: &Expr) -> Option<Float> {
    match expr {
        Expr::Float(f) => Some(*f as Float),
        _ => None
    }
}

fn get_num(expr: &Expr) -> Option<i64> {
    match expr {
        Expr::Num(n) => Some(*n),
        _ => None
    }
}

fn get_symbol(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Symbol(s) => Some(s.clone()),
        _ => None
    }
}

fn get_cam(expr : &Expr) -> Option<Camera> {
    let w = get_num(&car(expr))?;
    let h = get_num(&car(&cdr(expr)))?;
    let d = get_num(&car(&cdr(&cdr(expr))))?;
    Some(Camera { width: w as u32,
                  height: h as u32,
                  depth: d as u32 })
}

fn get_vec(expr: &Expr) -> Option<Vec3> {
    let x = get_float(&car(expr))?;
    let y = get_float(&car(&cdr(expr)))?;
    let z = get_float(&car(&cdr(&cdr(expr))))?;
    Some(Vec3(x as f32, y as f32, z as f32))
}

fn get_sphere(expr: &Expr,
              materials: &HashMap<String, Box<dyn Material>>)
              -> Option<Box<Sphere>> {
    let v = get_vec(&car(expr))?;
    let r = get_float(&car(&cdr(expr)))?;
    let mat_name = get_symbol(&car(&cdr(&cdr(expr))))?;
    let mat = materials.get(&mat_name)?;
    let s = new_sphere(v, r, mat.clone_box());
    Some(s)
}

fn get_plane(expr: &Expr,
             materials: &HashMap<String, Box<dyn Material>>)
             -> Option<Box<Plane>> {
    let v = get_vec(&car(expr))?;
    let n = get_vec(&car(&cdr(expr)))?;
    let mat_name = get_symbol(&car(&cdr(&cdr(expr))))?;
    let mat = materials.get(&mat_name)?;
    let p = new_plane(v, n, mat.clone_box());
    Some(p)
}

fn is_symbol(expr: &Expr, label: &str) -> Option<()> {
    let s = get_symbol(expr)?;
    if s.as_str() == label {
        Some(())
    } else {
        None
    }
}

fn get_color(expr: &Expr) -> Option<Color> {
    is_symbol(&car(expr), "color")?;
    get_vec(&cdr(expr))
}

fn get_specular(expr: &Expr) -> Option<(Float,Float)> {
    is_symbol(&car(expr), "spec")?;
    let y = get_float(&car(&cdr(expr)))?;
    let z = get_float(&car(&cdr(&cdr(expr))))?;
    Some((y, z))
}

fn get_reflection(expr: &Expr) -> Option<Float> {
    is_symbol(&car(expr), "reflection")?;
    get_float(&car(&cdr(expr)))
}

fn get_uv(expr: &Expr) -> Option<i64> {
    is_symbol(&car(expr), "uv")?;
    get_num(&car(&cdr(expr)))
}

fn get_solid(expr: &Expr) -> Option<Box<dyn Material>> {
    let color = get_color(&car(expr))?;
    let spec = get_specular(&car(&cdr(expr)))?;
    let refl = get_reflection(&car(&cdr(&cdr(expr))))?;
    Some(Box::new(Solid { color,
                          specular: spec,
                          reflection: refl }))
}

fn get_checkboard(expr: &Expr) -> Option<Box<dyn Material>> {
    let color1 = get_color(&car(expr))?;
    let color2 = get_color(&car(&cdr(expr)))?;
    let uv = get_uv(&car(&(cdr(&cdr(expr)))))?;
    let spec = get_specular(&car(&cdr(&cdr(&cdr(expr)))))?;
    let refl = get_reflection(&car(&cdr(&cdr(&cdr(&cdr(expr))))))?;
    Some(Box::new(Checker
                  { colors: (color1, color2),
                    uv: uv as i32, specular: spec, reflection: refl }))
}

fn get_material(expr: &Expr) -> Option<(String, Box<dyn Material>)> {
    let name = get_symbol(&car(expr))?;
    let shader = get_symbol(&car(&cdr(expr)))?;
    let m : Option<Box<dyn Material>> = match shader.as_str() {
        "solid" => get_solid(&cdr(&cdr(expr))),
        "checkboard" => get_checkboard(&cdr(&cdr(expr))),
        _ => None
    };
    m.map(|x| (name, x))
}

pub fn load_scene(filename: &str) -> Option<Scene> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let tokens = tokenize(contents.as_str());
    let exprs = parse_all(&tokens).unwrap();
    let mut cam : Option<Camera> = None;
    let mut objects : Vec<Box<dyn Intersect>> = Vec::new();
    let mut materials : HashMap<String, Box<dyn Material>> =
        HashMap::new();
    let mut reflections : i64 = i64::from(REFLECTIONS);
    
    for expr in exprs {
        if let Expr::Cons(e_car, e_cdr) = expr {
            if let Expr::Symbol(car_symb) = *e_car {
                match car_symb.as_str() {
                    "camera" => { cam = get_cam(&e_cdr) },
                    "sphere" => {
                        let s = get_sphere(&e_cdr, &materials)?;
                        objects.push(s);
                    },
                    "plane" => {
                        let p = get_plane(&e_cdr, &materials)?;
                        objects.push(p);
                    },
                    "mat" => {
                        let (name, mat) = get_material(&e_cdr)?;
                        materials.insert(name, mat);
                    },
                    "reflections" => {
                        reflections = get_num(&car(&e_cdr))?;
                        println!("reflections: {:?}", reflections);
                    },
                    unparsed => {
                        println!("Parse error: {:?}", unparsed);
                        return None;
                    }
                }
            }
        }
    }

    Some(Scene {
        camera: cam?,
        lights: Lights { dir: (Vec3(-0.5, -1.0, -0.75)).normalized(),
                         ambiant: 0.2,
                         bg: new_color(20.0, 20.0, 30.0) },
        objects,
        reflections: reflections as u32
    })
}
