use crate::vec3::{Point3, Vec3};
 // Load vec3 with the structs Point3 and Vec3

#[derive(Default)]


// Create a ray, a ray is basically line a line with an origin and a direction 
// e.g. a + \lambda*b where a and b are vectors and \lambda is a scalar

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}


// Quick rust note I'm making here so I don't get rusty in rust
// We use &self because we are immutably borrowing self, we don't want to take ownership of it and then it becomes into the bounds of the function
// Then it'd get destroyed once the function is run, hence destroying self
// This is called self destruction. I think I should start using &self daily to prevent this. 

impl Ray {
    // initialise a new ray
    // enter an origin for the ray and enter a direction, and it will return a ray object 
    /*
    Example usage:
        let r = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );
    
    In this case the camera location is at the origin (i.e. a point) and then we want to find what colour to make a pixel that a ray from the origin goes through
     */
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    
    // Once we have r, we can do stuff like r.origin which returns 
    pub fn origin(&self) -> Point3 {
        self.orig
    }
 
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    
    // At a dist t, where is the ray at? 
    // wya ray? 
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}