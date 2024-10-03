use std::io::Write;
use crate::common;
use crate::vec3::Vec3;
 
// we be representing colours using vectors 
pub type Color = Vec3;


/* CODE THAT DOESNT WORK! 
pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = (255.999 * pixel_color.x());
    let mut g = (255.999 * pixel_color.y());
    let mut b = (255.999 * pixel_color.z());

    // when we do the antialising, we add random bits to the ray so that its slightly to the up down left etc. of where it usually would go 
    // but then in the antialising script in main.rs the way we do it is using a for loop
    // over the number of samples and doing += colour of the randomly gen ray 
    // so now we need to scale the colour contributions of each of the rays

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
 
    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;
 
    // Write the translated [0, 255] value of each color component
    writeln!(
        out,
        "{} {} {}",
        (256.0 * common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}

*/


// SPOT THE DIFFERENCE BETWEEN THE ABOVE CODE AND THE BELOW CODE

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
 

    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);
   
    writeln!(
        out,
        "{} {} {}",
        (256.0 * common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}