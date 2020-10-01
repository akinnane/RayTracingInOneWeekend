use crate::hittable::HitRecord;
use crate::pixel::Pixel;
use crate::point::Point;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Pixel },
    Metal { albedo: Pixel, fuzz: f64 },
    Dielectric { ref_idx: f64 },
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
            Material::Metal { albedo, fuzz } => {
                Self::scatter_metal(&albedo, fuzz, ray, hit_record, attenuation, scattered)
            }
            Material::Dielectric { ref_idx } => {
                Self::scatter_dielectric(ref_idx, ray, hit_record, attenuation, scattered)
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
        fuzz: f64,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Pixel,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray.direction.unit_vector().reflect(&hit_record.normal);
        *scattered = Ray::new(
            hit_record.point,
            reflected + Point::random_in_unit_sphere() * fuzz,
        );
        *attenuation = albedo.clone();
        scattered.direction.dot(&hit_record.normal) > 0.0
    }

    fn scatter_dielectric(
        ref_idx: f64,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Pixel,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Pixel::new(1.0, 1.0, 1.0);
        let etai_over_etat = if hit_record.front_face {
            1.0 / ref_idx
        } else {
            ref_idx
        };
        let unit_direction = ray_in.direction.unit_vector();
        let refracted = unit_direction.refract(&hit_record.normal, etai_over_etat);
        *scattered = Ray::new(hit_record.point, refracted);
        true
    }
}
