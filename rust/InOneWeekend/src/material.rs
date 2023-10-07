use super::vec3;
use super::ray::Ray;
use super::color::Color;
use super::hittable::HitRecord;

pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
  pub albedo: Color,
}

impl Lambertian {
  pub fn new(a: &Color) -> Self {
    Self {
      albedo: *a,
    }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
    let mut scatter_direction = rec.normal + vec3::random_unit_vector();

    // 捕捉退化的散射方向
    if scatter_direction.near_zero() {
      scatter_direction = rec.normal;
    }

    *scattered = Ray::new(&rec.p, &scatter_direction);
    *attenuation = self.albedo;
    true
  }
}

pub struct Metal {
  pub albedo: Color,
  pub fuzz: f64,
}

impl Metal {
  pub fn new(a: &Color, f: f64) -> Self {
    Self {
      albedo: *a,
      fuzz: if f < 1.0 { f } else { 1.0 },
    }
  }
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
    let reflected = vec3::reflect(&vec3::unit_vector(r_in.direction()), &rec.normal);
    *scattered = Ray::new(&rec.p, &(reflected + self.fuzz * vec3::random_in_unit_sphere()));
    *attenuation = self.albedo;
    vec3::dot(scattered.direction(), &rec.normal) > 0.0
  }
}