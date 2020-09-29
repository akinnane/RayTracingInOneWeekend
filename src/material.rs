use crate::hittable::HitRecord;
use crate::pixel::Pixel;
use crate::point::Point;
use crate::ray::Ray;

//pub struct Material {}
#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Pixel },
    Metal { albedo: Pixel },
}

impl Default for Material {
    fn default() -> Material {
        Material::Lambertian {
            albedo: Pixel::new(0.0, 0.0, 0.0),
        }
    }
}

impl Material {
    pub fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Pixel,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                Self::scatter_lambertian(&albedo, ray, hit_record, attenuation, scattered)
            }
            Material::Metal { albedo } => {
                Self::scatter_metal(&albedo, ray, hit_record, attenuation, scattered)
            }
        }
    }

    fn scatter_lambertian(
        albedo: &Pixel,
        _ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Pixel,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = hit_record.normal + Point::random_unit_vector();
        *scattered = Ray::new(hit_record.point, scatter_direction);
        *attenuation = albedo.clone();
        true
    }

    fn scatter_metal(
        albedo: &Pixel,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Pixel,
        scattered: &mut Ray,
    ) -> bool {
        // let scatter_direction = hit_record.normal + Point::random_unit_vector();
        // *scattered = Ray::new(hit_record.point, scatter_direction);
        // *attenuation = albedo.clone();
        // true
        let reflected = ray.direction.unit_vector().reflect(&hit_record.normal);
        *scattered = Ray::new(hit_record.point, reflected);
        *attenuation = albedo.clone();
        scattered.direction.dot(&hit_record.normal) > 0.0
    }
}
