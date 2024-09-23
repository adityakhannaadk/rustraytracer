use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
use std::rc::Rc;
use crate::material::Material;


#[derive(Clone, Default)]
// This is just the thing thjat you put I geuss


pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Option<Rc<dyn Material>>,
    pub is_front: bool,
}
 

// What on earth does Defailt::default() do?
impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.is_front = vec3::dot(r.direction(), outward_normal) < 0.0;
        // outward normal is standard normal pointing outward 
        self.normal = if self.is_front {
            // if we're intersecting at the front of the sphere, return outward normal
            outward_normal
        } else {
            // if the dot is negative, that means the angle between the ray is more than 90 degrees
            // so that means its the back end of the sphere
            // this is to deal with stuff like glass 
            -outward_normal
        };
    }
}


// Is it hittable?
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}