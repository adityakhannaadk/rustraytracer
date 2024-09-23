use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray; 
use crate::{common};
use crate::vec3::{self, unit_vector, Vec3};

pub trait Material {
    // what does it mean if a trait is public??
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,

    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a}
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();
        // if random unit vector generated is directly oposite normal vector , the two will sum to zero, which will result in a zero scatter direction vector
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, scatter_direction);
        true
    }
}

pub struct Metal {
    albedo: Color,
}
 
impl Metal {
    pub fn new(a: Color) -> Metal {
        Metal { albedo: a }
    }
}
 
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
 
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, reflected);
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

pub struct RoughMetal {
    albedo: Color,
    roughness: f64

}

impl RoughMetal {
    pub fn new(a: Color, b: f64) -> RoughMetal {
        RoughMetal { albedo: a, roughness: b}
    }

}

impl Material for RoughMetal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal) +  vec3::t_mul(vec3::random_unit_vector(),self.roughness);
        //deref attenuation and set to self.albedo
        *attenuation = self.albedo;
        // deref scattered and set to a new ray starting at rec.p (point of intersection) and in the direction of reflected
        
        *scattered = Ray::new(rec.p, reflected);
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }

}


/* 
pub struct Dielectric {
    albedo: Color,
    refractive_index: f64
}


impl Dielectric {
    pub fn new(a: Color, b: f64) -> Dielectric {
        Dielectric { albedo: a, refractive_index: b}
    }

}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refrat = if rec.is_front{
            1.0/self.refractive_index;
        }else{
            self.refractive_index;
        };
        let reflected = vec3::refract(r_in.direction(), unit_vector(rec.normal), self.refractive_index);
        //deref attenuation and set to self.albedo
        *attenuation = Color::new(1.0, 1.0, 1.0);
        // deref scattered and set to a new ray starting at rec.p (point of intersection) and in the direction of reflected
        
        *scattered = Ray::new(rec.p, reflected);
        //eprint!("Scattered ray: {}",scattered.direction());
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }

}


// (^ CODE THAT DOESNT WORK ?!!?!?!?! WHY SOPT THE DIFFERENCE)
*/

pub struct Dielectric {
    ir: f64, // Index of refraction
}
 
impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }


    // Schlick approxinmaton code 
    // basically glass acts as a mirror at some angles 
    // this is Fresnel's reflectance equation
    // Schlick's approximation is an approximation to that to make it easeir to compute
    // https://en.wikipedia.org/wiki/Fresnel_equations

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}
 
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_ratio = if rec.is_front {
            1.0 / self.ir
        } else {
            self.ir
        };
 
        let unit_direction = vec3::unit_vector(r_in.direction());
        let refracted = vec3::refract(unit_direction, rec.normal, refraction_ratio);
 


        let cos_theta = f64::min(vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        


        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        // check critical angle


        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > common::random_double()
        {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };
 
        *attenuation = Color::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(rec.p, direction);
        true
    }
}