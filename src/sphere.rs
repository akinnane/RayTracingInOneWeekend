use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::point::Point;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp > t_min && temp < t_max {
                hit.t = temp;
                hit.point = ray.at(hit.t);
                hit.normal = (hit.point - self.center) / self.radius;
                let outward_normal = (hit.point - self.center) / self.radius;
                hit.set_face_normal(ray, outward_normal);
                return true;
            }

            let temp = (-half_b + root) / a;
            if temp > t_min && temp < t_max {
                hit.t = temp;
                hit.point = ray.at(hit.t);
                hit.normal = (hit.point - self.center) / self.radius;
                let outward_normal = (hit.point - self.center) / self.radius;
                hit.set_face_normal(ray, outward_normal);
                return true;
            }
        }
        false
    }
}
