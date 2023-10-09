use super::vec3::Point3;
use super::rtweekend;

const POINT_COUNT: usize = 256;

pub struct Perlin {
  ranfloat: Vec<f64>,
  perm_x: Vec<i32>,
  perm_y: Vec<i32>,
  perm_z: Vec<i32>,
}

impl Default for Perlin {
  fn default() -> Self {
    let mut ranfloat = Vec::with_capacity(POINT_COUNT);
    for _ in 0..POINT_COUNT {
      ranfloat.push(rtweekend::random_double());
    }
    let perm_x = Self::perlin_generate_perm();
    let perm_y = Self::perlin_generate_perm();
    let perm_z = Self::perlin_generate_perm();
    Self {
      ranfloat,
      perm_x,
      perm_y,
      perm_z,
    }
  }
}

impl Perlin {
  pub fn noise(&self, p: Point3) -> f64 {
    let i = ((4.0 * p.x()) as i32) & 255;
    let j = ((4.0 * p.y()) as i32) & 255;
    let k = ((4.0 * p.z()) as i32) & 255;

    self.ranfloat[
      self.perm_x[i as usize] as usize ^
      self.perm_y[j as usize] as usize ^
      self.perm_z[k as usize] as usize
    ]
  }
  fn perlin_generate_perm() -> Vec<i32> {
    let mut p = Vec::with_capacity(POINT_COUNT);
    for i in 0..POINT_COUNT {
      p.push(i as i32);
    }
    Self::permute(&mut p, POINT_COUNT);
    p
  }

  fn permute(p: &mut [i32], n: usize) {
    for i in (0..n).rev() {
      let target = rtweekend::random_int(0, i as i32);
      p.swap(i, target as usize);
    }
  }
}