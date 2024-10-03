use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use crate::common;
#[derive(Copy, Clone, Default)]


// This intilialises a datatype Vec3 which has 3 64 bit floating point values (3D vector)
// This is a really simple helper lib to manipulate 3D vectors
// e.g. add them, multiply them with scalars, alias them to a 3D point, cross them, dot them, get each of their index values. 
pub struct Vec3 {
    e: [f64; 3],
}



 // Main implementation on the struct Vec3
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
 
    pub fn random() -> Vec3 {
        Vec3::new(
            common::random_double(),
            common::random_double(),
            common::random_double(),
        )
    }
 
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            common::random_double_range(min, max),
            common::random_double_range(min, max),
            common::random_double_range(min, max),
        )
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    // These fns just return the values at particular index
    pub fn y(&self) -> f64 {
        self.e[1]
    }
 
    pub fn z(&self) -> f64 {
        self.e[2]
    }
 
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    
    // Find length squared using pythag
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    // just incase the scattered vector is realy close t 0
    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;
        // Return true if the vector is close to zero in all dimensions
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }
}
 
// Type alias
pub type Point3 = Vec3;
 // Alias for Vec3 in case we want to describe a point in 3D space instead of a 3D vector
// Output formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
 
// -Vec3
// Returns a new Vec3 object which points in the opposite direction to the original one
impl Neg for Vec3 {
    type Output = Vec3;
 
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}
 
// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}
 
// Vec3 *= f64
// If you have a vector v and you want to find v*t where t is a scalar, you can do it like this
// Also in this case it updates it because we use &mut self so this time self is modified. 
// Rust is slowly beginning to enter my thick skull

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}
 
// Vec3 /= f64
// Like the above but this time its *1/t
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}
 
// Vec3 + Vec3
// Adds two vectors
impl Add for Vec3 {
    type Output = Vec3;
 
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        // just keep generating random vectors until they have a length > 1.0
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}


// generate random vector in a unit disk with z = 0 so the vector lies on the plane z = 0 with bound of r = 1

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            common::random_double_range(-1.0, 1.0),
            common::random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}


pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}
/* 
pub fn refract(rdir: Vec3, n: Vec3, rel: f64) -> Vec3{
    //rel = n1/n2
    let v = unit_vector(rdir);
    let dotprod = dot(v, n);
    // n1sin(theta) = n2sin(90) is critical angle
    // sin(theta) = n2/n1
    // 1-sin^2(theta) = cos^2(theta) = 1 - n2^2/n1^2 (thetacrit)
    let critcos2thet = 1.0 - 1.0/rel*rel;
    // cos2theta is a decreasing function with theta upto pi/2 rads
    // the normal will always point in anyway
    // we want to reflect if the angle is greater than theta_crit
    // if cos^2(theta) > cos^2(theta_crit)
    // that means theta < thetacrit and we should refract
    // else reflect
    if(dotprod*dotprod > critcos2thet){
        // the minus here is ok bc the normal points opp dirn to incident ray 
        //eprint!("Ray was refracted");
        let R_perp = v - dotprod*n;
        let R_par = -f64::sqrt(1.0-(rel*rel)*(1.0-dotprod*dotprod))*n;
        return R_par + R_perp;

    }else{
        //eprint!("Ray was reflected");
        return reflect(v, n);
    }
}

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_lambertian_2,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.3, 0.0, -1.0),
        0.5,
        material_lambertian,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_metal,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_metal,
    )));

*/



pub fn refract(rdir: Vec3, n: Vec3, rel: f64) -> Vec3{
        let v = unit_vector(rdir);
        let dotprod = dot(v, n);
        // the minus here is ok bc the normal points opp dirn to incident ray 
        let R_perp = v - dotprod*n;
        let R_par = -f64::sqrt(1.0-(rel*rel)*(1.0-dotprod*dotprod))*n;
        return R_par + R_perp;
}


// Vec3 - Vec3
// Subtracts two vectors
impl Sub for Vec3 {
    type Output = Vec3;
 
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}
 

// Vec3 * Vec3
// If you have a Vec3 [a,b,c], initialised as v1, and a v2 [d,e,f], v1.Mul(v2) returns [a*d, b*e, c*f]
impl Mul for Vec3 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}
 
// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}
 
// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
 
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}
 
// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;
 
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}
 
pub fn t_mul(a: Vec3, t: f64) -> Vec3 {
    Vec3::new(a.x() * t, a.y() * t, a.z()* t)
}


 // Find the dot product innit
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
 
// Now we find the cross product
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}
 
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}