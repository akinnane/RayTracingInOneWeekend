use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Point,
    pub t: f64,
    pub front_face: bool,
    pub material: Box<Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Point) {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
