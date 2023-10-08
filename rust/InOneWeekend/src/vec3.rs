use std::ops::{
  AddAssign,
  DivAssign,
  Index,
  IndexMut,
  MulAssign,
  Neg,
  Add,
  Sub,
  Mul,
  Div,
};

use super::rtweekend::{
  random_double,
  random_double_range,
};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
  pub e: [f64; 3]
}

impl Default for Vec3 {
  fn default() -> Self {
    Self { e: [0.0, 0.0, 0.0] }
  }
}

impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
  }
}

impl Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, i: usize) -> &Self::Output {
    &self.e[i]
  }
}

impl IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, i: usize) -> &mut Self::Output {
    &mut self.e[i]
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, other: Self) {
    self.e[0] += other.e[0];
    self.e[1] += other.e[1];
    self.e[2] += other.e[2];
  }
}

impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, t: f64) {
    self.e[0] *= t;
    self.e[1] *= t;
    self.e[2] *= t;
  }
}

impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, t: f64) {
    *self *= 1.0 / t;
  }
}

impl Add for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Vec3 { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Vec3 { e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
  }
}

impl Mul for Vec3 {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Vec3 { e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
  }
}

impl Mul<f64> for Vec3 {
  type Output = Self;

  fn mul(self, t: f64) -> Self::Output {
    Vec3 { e: [self.e[0] * t, self.e[1] * t, self.e[2] * t] }
  }
}

impl Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, v: Vec3) -> Self::Output {
    v * self
  }
}

impl Div<f64> for Vec3 {
  type Output = Self;

  fn div(self, t: f64) -> Self::Output {
    (1.0 / t) * self
  }
}

impl std::fmt::Display for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
  }
}

impl Vec3 {
  pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
    Self { e: [e0, e1, e2] }
  }

  pub fn x(&self) -> f64 { self.e[0] }
  pub fn y(&self) -> f64 { self.e[1] }
  pub fn z(&self) -> f64 { self.e[2] }

  pub fn length(&self) -> f64 {
    (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
  }

  pub fn near_zero(&self) -> bool {
    let s = 1e-8;
    self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
  }

  pub fn random() -> Self {
    Self { e: [random_double(), random_double(), random_double()] }
  }

  pub fn random_range(min: f64, max: f64) -> Self {
    Self { e: [random_double_range(min, max), random_double_range(min, max), random_double_range(min, max)] }
  }
}

pub type Point3 = Vec3;

pub fn dot(u: Vec3, v: Vec3) -> f64 {
  u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
  Vec3 { e: [
    u.e[1] * v.e[2] - u.e[2] * v.e[1],
    u.e[2] * v.e[0] - u.e[0] * v.e[2],
    u.e[0] * v.e[1] - u.e[1] * v.e[0],
  ]}
}

pub fn unit_vector(v: Vec3) -> Vec3 {
  v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
  loop {
    let p = Vec3::random_range(-1.0, 1.0);
    if p.length_squared() < 1.0 {
      return p;
    }
  }
}

pub fn random_unit_vector() -> Vec3 {
  unit_vector(random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
  let on_unit_sphere = random_in_unit_sphere();
  if dot(on_unit_sphere, normal) > 0.0 { // In the same hemisphere as the normal
    on_unit_sphere
  } else {
    -on_unit_sphere
  }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
  let cos_theta = dot(-uv, n).min(1.0);
  let r_out_perp = etai_over_etat * (uv + cos_theta * n);
  let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
  r_out_perp + r_out_parallel
}