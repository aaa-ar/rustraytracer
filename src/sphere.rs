use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    fn hit_record(&self, r: &Ray, temp: f32) -> HitRecord {
        let t = temp;
        let p = r.point_at_parameter(t);
        let normal = (p - self.center) / self.radius;
        HitRecord { t, p, normal }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;

        let a = r.direction.squared_length();
        let b = oc.dot(&r.direction);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant <= 0.0 {
            return None;
        }

        let mut temp = (-b - discriminant.sqrt()) / a;

        if temp <= t_min || t_max <= temp {
            temp = (-b + discriminant.sqrt()) / a;
        }

        if temp <= t_min || t_max <= temp {
            return None;
        }

        Some(self.hit_record(r, temp))
    }
}
