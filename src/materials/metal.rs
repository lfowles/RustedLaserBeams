extern crate cgmath;

use cgmath::Vector3;

use super::{Material, ScatterResult};
use super::super::ray::Ray;
use super::super::hittable::HitRecord;
use super::super::random_in_unit_sphere;

use cgmath::InnerSpace;

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}

pub fn reflect(v: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(*normal) * normal
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected = reflect(&r_in.direction.normalize(), &rec.normal);
        let scattered = Ray { origin: rec.point, direction: reflected + self.fuzz * random_in_unit_sphere() };

        if scattered.direction.dot(rec.normal) > 0.0 {
            Some(ScatterResult {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}