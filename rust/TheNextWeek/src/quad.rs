use std::rc::Rc;

use super::vec3::{
  self,
  Vec3,
  Point3,
};
use super::material::Material;
use super::aabb::Aabb;
use super::hittable::{
  HitRecord,
  Hittable,
};
use super::interval::Interval;
use super::ray::Ray;

pub struct Quad {
  q: Point3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  normal: Vec3,
  d: f64,
  mat: Rc<dyn Material>,
  bbox: Aabb,
}

impl Quad {
  pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
    let n = vec3::cross(u, v);
    let normal = vec3::unit_vector(n);
    Self {
      q,
      u,
      v,
      w: n / vec3::dot(n, n),
      normal,
      d: vec3::dot(normal, q),
      mat,
      bbox: Aabb::new_with_point(
        &q, &(q + u + v)
      ),
    }
  }

  pub fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
    // 给定平面坐标中的击中点，如果它在基元之外，则返回false，否则设置击中记录的UV坐标并返回true。
    if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b) {
      return false;
    }

    rec.u = a;
    rec.v = b;

    true
  }
}

impl Hittable for Quad {
  fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
    let denom = vec3::dot(self.normal, r.direction());

    // 如果射线与平面平行，则没有相交。
    if denom.abs() < 1e-8 {
      return false;
    }

    // 如果相交点参数 t 在射线区间之外，则返回 false。
    let t = (self.d - vec3::dot(self.normal, r.origin())) / denom;
    if !ray_t.contains(t) {
      return false;
    }

    // 使用平面坐标确定击中点是否在平面形状内部。
    let intersection = r.at(t);
    let planar_hitpt_vector = intersection - self.q;
    let alpha = vec3::dot(self.w, vec3::cross(planar_hitpt_vector, self.v));
    let beta = vec3::dot(self.w, vec3::cross(self.u, planar_hitpt_vector));

    if !self.is_interior(alpha, beta, rec) {
      return false;
    }

    // 光线击中了2D形状；设置剩余的击中记录并返回true。
    rec.t = t;
    rec.p = intersection;
    rec.mat = Some(Rc::clone(&self.mat));
    rec.set_face_normal(r, self.normal);

    true
  }

  fn bounding_box(&self) -> &Aabb {
    &self.bbox
  }
}
