use std::rc::Rc;
use super::vec3;
use super::ray::Ray;
use super::color::Color;
use super::hittable::HitRecord;
use super::rtweekend;
use super::texture::{
  Texture,
  SolidColor,
};
use super::pdf::{
  Pdf,
  NonePdf,
  CosinePdf,
  SpherePdf,
};

pub struct ScatterRecord {
  pub attenuation: Color,
  pub pdf: Box<dyn Pdf>,
  pub skip_pdf: bool,
  pub skip_pdf_ray: Ray,
}

impl Default for ScatterRecord {
  fn default() -> Self {
    Self {
      attenuation: Color::default(),
      pdf: Box::new(NonePdf{}),
      skip_pdf: false,
      skip_pdf_ray: Ray::default(),
    }
  }
}

pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool;
  fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: vec3::Point3) -> Color {
    Color::new(0.0, 0.0, 0.0)
  }
  fn albedo(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: vec3::Point3) -> Color {
    Color::new(1.0, 1.0, 1.0)
  }
  fn normal(&self, _r_in: &Ray, rec: &HitRecord, _u: f64, _v: f64, _p: vec3::Point3) -> Color {
    rec.normal
  }
  fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
    0.0
  }
}

pub struct Lambertian {
  pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
  pub fn new(a: Color) -> Self {
    Self {
      albedo: Rc::new(SolidColor::new(a)),
    }
  }

  pub fn new_with_texture(a: Rc<dyn Texture>) -> Self {
    Self {
      albedo: a,
    }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
    srec.attenuation = self.albedo.value(rec.u, rec.v, rec.p);
    srec.pdf = Box::new(CosinePdf::new(rec.normal));
    srec.skip_pdf = false;
    true
  }

  fn albedo(&self, _r_in: &Ray, _rec: &HitRecord, u: f64, v: f64, p: vec3::Point3) -> Color {
    self.albedo.value(u, v, p)
  }

  fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
    let cosine = vec3::dot(rec.normal, vec3::unit_vector(scattered.direction()));
    if cosine < 0.0 {
      0.0
    } else {
      cosine / rtweekend::PI
    }
  }
}

pub struct Metal {
  pub albedo: Color,
  pub fuzz: f64,
}

impl Metal {
  pub fn new(a: Color, f: f64) -> Self {
    Self {
      albedo: a,
      fuzz: if f < 1.0 { f } else { 1.0 },
    }
  }
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
    srec.attenuation = self.albedo;
    srec.skip_pdf = true;
    let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
    srec.skip_pdf_ray = Ray::new_with_time(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere(), r_in.time());
    true
  }

  fn albedo(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: vec3::Point3) -> Color {
    self.albedo
  }
}

pub struct Dielectric {
  pub ir: f64, // 折射指数
}

impl Dielectric {
  pub fn new(index_of_refraction: f64) -> Self {
    Self {
      ir: index_of_refraction,
    }
  }

  fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // 使用 Schlick's approximation 近似计算反射系数
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
  }
}

impl Material for Dielectric {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
    srec.attenuation = Color::new(1.0, 1.0, 1.0);
    srec.skip_pdf = true;
    let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };

    let unit_direction = vec3::unit_vector(r_in.direction());
    let cos_theta = vec3::dot(-unit_direction, rec.normal).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rtweekend::random_double() {
      vec3::reflect(unit_direction, rec.normal)
    } else {
      vec3::refract(unit_direction, rec.normal, refraction_ratio)
    };

    srec.skip_pdf_ray = Ray::new_with_time(rec.p, direction, r_in.time());
    true
  }
}

pub struct DiffuseLight {
  pub emit: Rc<dyn Texture>,
}

impl DiffuseLight {
  pub fn new(a: Rc<dyn Texture>) -> Self {
    Self {
      emit: a,
    }
  }

  pub fn new_with_color(c: Color) -> Self {
    Self {
      emit: Rc::new(SolidColor::new(c)),
    }
  }
}

impl Material for DiffuseLight {
  fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
    false
  }

  fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: vec3::Point3) -> Color {
    if rec.front_face {
      self.emit.value(u, v, p)
    } else {
      Color::default()
    }
  }

  fn albedo(&self, _r_in: &Ray, _rec: &HitRecord, u: f64, v: f64, p: vec3::Point3) -> Color {
    self.emit.value(u, v, p)
  }
}

pub struct Isotropic {
  pub albedo: Rc<dyn Texture>,
}

impl Isotropic {
  pub fn new(a: Rc<dyn Texture>) -> Self {
    Self {
      albedo: a,
    }
  }

  pub fn new_with_color(c: Color) -> Self {
    Self {
      albedo: Rc::new(SolidColor::new(c)),
    }
  }
}

impl Material for Isotropic {
  fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
    srec.attenuation = self.albedo.value(rec.u, rec.v, rec.p);
    srec.pdf = Box::new(SpherePdf{});
    srec.skip_pdf = false;
    true
  }

  fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
    1.0 / (4.0 * rtweekend::PI)
  }

  fn albedo(&self, _r_in: &Ray, _rec: &HitRecord, u: f64, v: f64, p: vec3::Point3) -> Color {
    self.albedo.value(u, v, p)
  }
}