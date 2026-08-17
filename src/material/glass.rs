use crate::{hittable::HitResult, material::Material, ray::Ray};

#[derive(Debug, Clone, Copy)]
pub struct Glass {
    refraction_index: f32, // vacuum:medium ratio
}
impl Glass {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}
impl Material for Glass {
    fn scatter_ray(&self, hit_result: &HitResult) -> Ray {
        //snell's law
        let n = hit_result.normal;
        let incident = hit_result.ray.dir;
        let incident_perp = (incident - incident.dot(n) * n).normalize_or_zero();
        let exiting_vacuum = hit_result.ray.dir.dot(n) > 0.0;
        let cos_theta = incident.dot(n).abs(); // incident.dot(n) can be -ve
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let (n1, n2) = if exiting_vacuum {
            (self.refraction_index, 1.0)
        } else {
            (1.0, self.refraction_index)
        };
        let sin_theta_refracted = n1 / n2 * sin_theta;
        fn reflect_chance(n1: f32, n2: f32, cos_theta: f32) -> f32 {
            // schlick's approximation
            let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
            r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
        }
        let ray_dir = if sin_theta_refracted > 1.0
            || rand::random::<f32>() < reflect_chance(n1, n2, cos_theta)
        {
            // total internal reflection
            incident.reflect(hit_result.normal)
        } else {
            let refract_perp = incident_perp * sin_theta_refracted;
            let refract_par = (1.0 - refract_perp.length_squared()).max(0.0).sqrt()
                * if exiting_vacuum { n } else { -n };
            (refract_perp + refract_par).normalize_or(incident)
        };
        Ray::new(hit_result.point, ray_dir, hit_result.ray.attenuation)
    }

    fn clone_mat(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}
