use std::rc::Rc;

use super::rtweekend;
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
use super::hittable_list::HittableList;

pub struct Quad {
  q: Point3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  normal: Vec3,
  d: f64,
  mat: Rc<dyn Material>,
  bbox: Aabb,
  area: f64,
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
      area: n.length(),
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

  fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
    let mut rec = HitRecord::default();
    if !self.hit(&Ray::new(origin, direction), &Interval::new(0.0001, f64::INFINITY), &mut rec) {
      return 0.0;
    }

    let distance_squared = rec.t * rec.t * direction.length_squared();
    let cosine = (vec3::dot(direction, rec.normal) / direction.length()).abs();

    distance_squared / (cosine * self.area)
  }

  fn random(&self, origin: Point3) -> Vec3 {
    let p = self.q + (rtweekend::random_double() * self.u) + (rtweekend::random_double() * self.v);
    p - origin
  }
}

pub fn make_box(a: Point3, b: Point3, mat: Rc<dyn Material>) -> Rc<HittableList> {
  // 返回一个包含两个对角顶点a和b的3D盒子（六个面）。

  let mut sides = HittableList::default();

  // 构造两个对角顶点，具有最小和最大的坐标。
  let min = Point3::new(
    a.x().min(b.x()),
    a.y().min(b.y()),
    a.z().min(b.z()),
  );
  let max = Point3::new(
    a.x().max(b.x()),
    a.y().max(b.y()),
    a.z().max(b.z()),
  );

  let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
  let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
  let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

  sides.add(Rc::new(
    Quad::new(Point3::new(min.x(), min.y(), max.z()), dx, dy, Rc::clone(&mat))
  ));
  sides.add(Rc::new(
    Quad::new(Point3::new(max.x(), min.y(), max.z()), -dz, dy, Rc::clone(&mat))
  ));
  sides.add(Rc::new(
    Quad::new(Point3::new(max.x(), min.y(), min.z()), -dx, dy, Rc::clone(&mat))
  ));
  sides.add(Rc::new(
    Quad::new(Point3::new(min.x(), min.y(), min.z()), dz, dy, Rc::clone(&mat))
  ));
  sides.add(Rc::new(
    Quad::new(Point3::new(min.x(), max.y(), max.z()), dx, -dz, Rc::clone(&mat))
  ));
  sides.add(Rc::new(
    Quad::new(Point3::new(min.x(), min.y(), min.z()), dx, dz, Rc::clone(&mat))
  ));

  Rc::new(sides)
}
