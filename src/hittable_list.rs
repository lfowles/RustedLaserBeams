use hittable::Hittable;

pub struct HittableList {
    pub list: Vec<Box<Hittable>>,
}

use hittable::HitRecord;
use ray::Ray;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest = t_max;
        for hittable in &self.list {
            if let Some(record) = hittable.hit(ray, t_min, closest) {
                closest = record.t;
                closest_hit = Some(record);
            }
        }

        closest_hit
    }
}

