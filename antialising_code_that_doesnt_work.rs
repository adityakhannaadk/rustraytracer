mod color;
mod ray;
mod vec3;
mod common;
mod hittable;
mod hitlist;
mod sphere_object;
mod camera;


use hittable::{HitRecord, Hittable};
use hitlist::HittableList;
use sphere_object::Sphere;
 
use std::{io, ops::{Mul, Sub}};
use camera::Camera;
use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};
 
fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, common::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }else{
        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        // BACKGROUND SCENES! 
        return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
    }
}
 
/* 
fn intersects(r: &Ray, orig: &Point3, radius: &f64) -> f64 {
    let a = r.origin();
    let b = r.direction();
    let c = orig;
    // This is very bad code (because it's written by me) and I have no idea if I'm using the whole borrow rules thing right
    // Oh well... 
    // IM GONNA CHANGE THIS EVNETUNALY I SWEARJ
    let discrim_b: f64 = 2.0*(&b.x()*(a.x() - &c.x())+&b.y()*(a.y() - &c.y())+&b.z()*(a.z() - &c.z()));
    let discrim_c = &c.x()*&c.x()+c.y()*c.y()+c.z()*c.z() - radius;
    let discrim_a = b.x()*b.x()+b.y()*b.y()+b.z()*b.z();

    let discriminant = discrim_b*discrim_b - 4.0*discrim_a*discrim_c;
    if discriminant < 0.0 {
        return -1.0;
    }else{
        return (-discrim_b-f64::sqrt(discriminant))/(2.0*discrim_a);
    }

}


I get yoinked every time I try to write my own code wtf


fn intersects(r: &Ray, orig: &Point3, radius: f64) -> f64 {
    // there's probably still a lot wrong with this one but I don't know enough rust to tell right now
    // time will tell
    // time has told that I don't know enough
    let a = r.origin();
    let b = r.direction();
    let c = orig;
    let discrim_b_1 = Vec3::sub(a, *c);
    let discrim_b_2 = Vec3::mul(b, 2.0);
    let discrim_b = vec3::dot(discrim_b_1, discrim_b_2);
    let discrim_c = vec3::dot(*c,*c) - radius*radius;
    let discrim_a = vec3::dot(b,b);
    let discriminant = discrim_b*discrim_b - 4.0*discrim_a*discrim_c;
    if discriminant < 0.0 {
        return -1.0;
    }else{
        return (-discrim_b-f64::sqrt(discriminant))/(2.0*discrim_a);
    }   


}


*/
fn main() {
    // Image
 
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;


    // ANTIALISING 
    const SAMPLES_PER_PIXEL: i32 = 100; 

    
    // World
 
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, 4.0), 0.5)));
    //world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))); 
    // Camera
    
    let new_cam = Camera::new();
    // Render
 
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for i in 0..SAMPLES_PER_PIXEL{
                let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
                let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
                let r = new_cam.get_ray(u,v);
                pixel_color += ray_color(&r, &world);

            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
 
    eprint!("\nDone.\n");
}