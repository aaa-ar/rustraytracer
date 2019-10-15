use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HitList<'a> {
    pub vec: Vec<&'a (Hit + 'a)>,
}

impl<'a> HitList<'a> {
    pub fn push<'b>(&'b mut self, elem: &'a Hit) {
        self.vec.push(elem);
    }
}

impl<'a> Hit for HitList<'a> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.vec.len() {
            if let Some(t) = self.vec[i].hit(r, t_min, closest_so_far) {
                hit_anything = true;
                temp_rec = t;
                closest_so_far = temp_rec.t;
            }
        }
        if !hit_anything {
            return None;
        }
        Some(temp_rec)
    }
}
