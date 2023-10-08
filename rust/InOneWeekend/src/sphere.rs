use std::rc::Rc;

use super::vec3::{
  self,
  Point3,
};
use super::ray::Ray;
use super::hittable::{
  HitRecord,
  Hittable,
};
use super::interval::Interval;
use super::material::Material;

pub struct Sphere {
  center: Point3,
  radius: f64,
  mat: Rc<dyn Material>,
}

impl Sphere {
  pub fn new(center: &Point3, radius: f64, material: Rc<dyn Material>) -> Self {
    Self {
      center: *center,
      radius,
      mat: material,
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
    let oc = self.center - r.origin();
    let a = r.direction().length_squared();
    let h = vec3::dot(r.direction(), oc);
    let c = oc.length_squared() - self.radius * self.radius;

    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
      return false;
    }
    let sqrtd = discriminant.sqrt();

    // Find the nearest root that lies in the acceptable range.
    let mut root = (h - sqrtd) / a;
    if !ray_t.surrounds(root) {
      root = (h + sqrtd) / a;
      if !ray_t.surrounds(root) {
        return false;
      }
    }

    hit_record.t = root;
    hit_record.p = r.at(hit_record.t);
    let outward_normal = (hit_record.p - self.center) / self.radius;
    hit_record.set_face_normal(r, outward_normal);
    hit_record.mat = Some(Rc::clone(&self.mat));

    true
  }
}