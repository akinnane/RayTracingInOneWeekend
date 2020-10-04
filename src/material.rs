use crate::hittable::HitRecord;
use crate::pixel::Pixel;
use crate::point::Point;
use crate::ray::Ray;
use rand::Rng;

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
        *attenuation = *albedo;
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
        *attenuation = *albedo;
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
        let refraction_ratio = if hit_record.front_face {
            1.0 / ref_idx
        } else {
            ref_idx
        };
        let unit_direction = ray_in.direction.unit_vector();

        let cos_theta = -unit_direction.dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();
        let direction = if cannot_refract || Material::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0, 1.0) {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, refraction_ratio)
        };

        *scattered = Ray::new(hit_record.point, direction);
        true
    }

    fn reflectance(cosine:f64, ref_idx:f64) -> f64{
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 *= r0;
        r0 + (1.0-r0) * (1.0 - cosine).powi(5)
    }
}
