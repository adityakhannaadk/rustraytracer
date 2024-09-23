mod color;
mod ray;
mod vec3;
 
use std::io;
 
use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};
 
fn ray_color(r: &Ray, orig: &Point3) -> Color {
    let radiu = 0.78;
    if intersects(r,orig,&radiu){
        return Color::new(1.0, 0.7, 0.2);
    }
    else{
        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
    }
}
 
fn intersects(r: &Ray, orig: &Point3, radius: &f64) -> bool {
    let a = r.origin();
    let b = r.direction();
    let c = orig;
    // This is very bad code (because it's written by me) and I have no idea if I'm using the whole borrow rules thing right
    // Oh well... 
    let discrim_b: f64 = 2.0*(&b.x()*(a.x() - &c.x())+&b.y()*(a.y() - &c.y())+&b.z()*(a.z() - &c.z()));
    let discrim_c = &c.x()*&c.x()+c.y()*c.y()+c.z()*c.z() - radius;
    let discrim_a = b.x()*b.x()+b.y()*b.y()+b.z()*b.z();

    return discrim_b*discrim_b - 4.0*discrim_a*discrim_c >= 0.0;

}

fn main() {
    // Image
 
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
 
    // Camera
 
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
 
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
 
    // Render
 
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let origin_2 = Point3::new(0.0,0.0,2.7);
            let pixel_color = ray_color(&r, &origin_2);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }
 
    eprint!("\nDone.\n");
}