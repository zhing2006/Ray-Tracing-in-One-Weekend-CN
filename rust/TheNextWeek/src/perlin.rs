use super::vec3::{
  self,
  Vec3,
  Point3,
};
use super::rtweekend;

const POINT_COUNT: usize = 256;

pub struct Perlin {
  ranvec: Vec<Vec3>,
  perm_x: Vec<i32>,
  perm_y: Vec<i32>,
  perm_z: Vec<i32>,
}

impl Default for Perlin {
  fn default() -> Self {
    let mut ranvec = Vec::with_capacity(POINT_COUNT);
    (0..POINT_COUNT).for_each(|_| {
      ranvec.push(vec3::unit_vector(Vec3::random_range(-1.0, 1.0)));
    });
    let perm_x = Self::perlin_generate_perm();
    let perm_y = Self::perlin_generate_perm();
    let perm_z = Self::perlin_generate_perm();
    Self {
      ranvec,
      perm_x,
      perm_y,
      perm_z,
    }
  }
}

impl Perlin {
  pub fn noise(&self, p: Point3) -> f64 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();
    let i = p.x().floor() as i32;
    let j = p.y().floor() as i32;
    let k = p.z().floor() as i32;
    let mut c = [[[Vec3::default(); 2]; 2]; 2];

    (0..2).for_each(|di| {
      (0..2).for_each(|dj| {
        (0..2).for_each(|dk| {
          c[di][dj][dk] = self.ranvec[
            self.perm_x[((i + di as i32) & 255) as usize] as usize ^
            self.perm_y[((j + dj as i32) & 255) as usize] as usize ^
            self.perm_z[((k + dk as i32) & 255) as usize] as usize
          ];
        })
      })
    });

    Self::trilinear_interp(&c, u, v, w)
  }

  pub fn turb(&self, p: Point3, depth: i32) -> f64 {
    let mut accum = 0.0;
    let mut temp_p = p;
    let mut weight = 1.0;

    for _ in 0..depth {
      accum += weight * self.noise(temp_p);
      weight *= 0.5;
      temp_p *= 2.0;
    }

    accum.abs()
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

  fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    (0..2).for_each(|i| {
      (0..2).for_each(|j| {
        (0..2).for_each(|k| {
          let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
          accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                 * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                 * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                 * vec3::dot(c[i][j][k], weight_v);
        })
      })
    });
    accum
  }
}