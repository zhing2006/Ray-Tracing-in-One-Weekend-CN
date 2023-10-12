use std::ops::{
  Index,
  IndexMut,
};

use super::vec3::{self, Vec3};

#[derive(Default)]
pub struct Onb {
  pub axis: [Vec3; 3],
}

impl Onb {
  pub fn u(&self) -> Vec3 {
    self.axis[0]
  }
  pub fn v(&self) -> Vec3 {
    self.axis[1]
  }
  pub fn w(&self) -> Vec3 {
    self.axis[2]
  }

  pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
    a * self.u() + b * self.v() + c * self.w()
  }
  pub fn local_v(&self, a: Vec3) -> Vec3 {
    a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
  }

  pub fn new_from_w(w: Vec3) -> Self {
    let unit_w = vec3::unit_vector(w);
    let a = if unit_w.x().abs() > 0.9 {
      Vec3::new(0.0, 1.0, 0.0)
    } else {
      Vec3::new(1.0, 0.0, 0.0)
    };
    let v = vec3::unit_vector(vec3::cross(unit_w, a));
    let u = vec3::cross(unit_w, v);
    Self { axis: [u, v, unit_w] }
  }
}

impl Index<usize> for Onb {
  type Output = Vec3;

  fn index(&self, i: usize) -> &Self::Output {
    &self.axis[i]
  }
}

impl IndexMut<usize> for Onb {
  fn index_mut(&mut self, i: usize) -> &mut Self::Output {
    &mut self.axis[i]
  }
}