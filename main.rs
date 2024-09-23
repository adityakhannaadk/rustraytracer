mod color;
mod ray;
mod vec3;
mod common;
mod hittable;
mod hitlist;
mod sphere_object;
mod material;

use material::{Dielectric, Lambertian, Metal, RoughMetal};
use hittable::{HitRecord, Hittable};
use hitlist::HittableList;
use sphere_object::Sphere;
 
use std::{arch::x86_64::_mm_permutevar_pd, io, ops::{Mul, Sub}};
use std::rc::Rc;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};
 
fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.9,0.0,0.0);
    }
    let mut rec = HitRecord::new();
    // so we create a random/reflected child ray and that bounces around until what??
    // it hits the background or just keeps bouncing in the base case
    // albedo of current object is multiplied. in the depth > 100 case we just return 0 
    // if hits background or light source as base case, we multiply by albedo of the previously bounced object 
    // e.g. if its (1.0,1.0,1.0) white, then hits a red object (1.0,0,0) then bounces into camera, we see red

    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        // diffuse reflection
        // normal plus random unit vector 
        // rec.p is the point at which the ray hit
        //let direction = rec.normal + vec3::random_unit_vector();
        //let direction = 2.0*vec3::unit_vector(rec.normal+0.01*vec3::random_unit_vector()) + vec3::unit_vector(r.direction());
        //return 0.5 * ray_color(&Ray::new(rec.p, direction), world, depth-1);
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        if rec
            .mat
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        // if it doesnt hit anything and just goes off into space then you return black, no matter the albedoes (albedi?) of the previous objects hit by the ray in the callstack
        // itll be blacc bc by then the light energy woulda dropped off to 0 anyway
        return Color::new(0.0, 0.0, 0.0);
    }else{
        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        // BACKGROUND SCENES! 
        return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.2, 0.0, 1.0);
        // return Color::new(0.9, 0.9, 0.9);
    }
}
 
fn main() {
    // Image
 
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 200;
    // World
 
    let mut world = HittableList::new();

    let material_metal = Rc::new(RoughMetal::new(Color::new(0.7,0.4,0.4),0.05));
    let material_lambertian = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_lambertian_2 = Rc::new(RoughMetal::new(Color::new(0.2, 0.3, 0.7),0.3));
    // let material_rough = Rc::new(RoughMetal::new(Color::new(0.2, 0.2, 0.2),0.3));
    let material_rough = Rc::new(Dielectric::new(1.41));
    let material_rough_2 = Rc::new(RoughMetal::new(Color::new(0.97, 0.8, 0.8),0.4));
    let material_glass = Rc::new(Dielectric::new(1.41));
    let material_glass_2 = Rc::new(Dielectric::new(1.41));
    
    

    world.add(Box::new(Sphere::new(
        Point3::new(-0.8, 0.0, -1.0),
        0.5,
        material_rough,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-0.8, 0.0, -1.4),
        0.5,
        material_rough_2,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.3, 0.0, -1.34),
        0.5,
        material_lambertian,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.4),
        0.5,
        material_metal,
    )));


    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_lambertian_2,
    )));


    world.add(Box::new(Sphere::new(
        Point3::new(0.4, 0.1, -0.6),
        0.2,
        material_glass,
    )));




 
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
 
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let u = Vec3::default();
    let V = Vec3::default();    
    let lens_radius = 0.0;


    // Render
 
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for _ in 0..SAMPLES_PER_PIXEL{
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = Ray::new(
                    origin,
                    lower_left_corner + u * horizontal + v * vertical - origin,
                );
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
 
 
    eprint!("\nDone.\n");
}