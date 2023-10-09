use std::rc::Rc;

use super::vec3::{
  self,
  Vec3,
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
  center1: Point3,
  radius: f64,
  mat: Rc<dyn Material>,
  is_moving: bool,
  center_vec: Vec3,
}

impl Sphere {
  pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
    Self {
      center1: center,
      radius,
      mat: material,
      is_moving: false,
      center_vec: Vec3::default(),
    }
  }

  pub fn new_with_center2(center1: Point3, center2: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
    Self {
      center1,
      radius,
      mat: material,
      is_moving: true,
      center_vec: center2 - center1,
    }
  }

  fn sphere_center(&self, time: f64) -> Point3 {
    self.center1 + self.center_vec * time
  }
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
    let center = if self.is_moving { self.sphere_center(r.time()) } else { self.center1 };
    let oc = center - r.origin();
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
    let outward_normal = (hit_record.p - self.center1) / self.radius;
    hit_record.set_face_normal(r, outward_normal);
    hit_record.mat = Some(Rc::clone(&self.mat));

    true
  }
}