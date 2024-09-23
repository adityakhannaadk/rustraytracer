use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
 
#[derive(Default)]

// it's the ray hitlist
// tings the ray could potentially hit 


/*

Syntax explanation for my own use:
Vec:

This is a standard library type in Rust that represents a dynamic array or a growable list. It can hold multiple elements of the same type and allows for efficient addition and removal of elements.

Box:

Box is a smart pointer in Rust that allocates memory on the heap. When you place an object inside a Box, that object is stored on the heap, and the Box itself, which is small and contains a pointer to the heap-allocated data, is stored on the stack.
Using Box is necessary here because dyn Hittable is a trait object, which is unsized. Unsized types cannot be stored directly in collections like Vec, which require knowing the size of elements at compile time. However, a Box<dyn Hittable> has a known size (the size of a pointer), so it can be stored in a Vec.

dyn Hittable:

dyn Hittable is a trait object. In Rust, traits define shared behavior, and a trait object allows you to work with different types that implement a particular trait through dynamic dispatch.
dyn Hittable means "any type that implements the Hittable trait." When you use dyn before a trait, Rust uses dynamic dispatch to call the appropriate method implementation at runtime rather than at compile time.
This is useful when you need to store different types that implement the same trait in a single collection, like a Vec.

*/
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
 


// Implementation on HittableList so that we can create a new hitlsit, and add things to be hit to the hitlist

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }
 
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
 


impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // takes in the range
        // mutable variables

        // temp_rec = HitRecord::new() initialises a new HitRecord
        // the HitRecord struct is in hittable.rs

        /*
        pub struct HitRecord {
        pub p: Point3,
        pub normal: Vec3,
        pub t: f64,
        pub is_front: bool,
        }
        
         */

        let mut temp_rec = HitRecord::new();
        // this is a bool that is returned, does the ray that we take in actually hit anything?
        let mut hit_anything = false;
        // I still don't ujnderstand the range thing...
        // Hopefully this will illuminate itself eventually
        let mut closest_so_far = t_max;
 
        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
 
        hit_anything
    }
}