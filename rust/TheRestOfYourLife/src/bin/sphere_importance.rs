use the_rest_of_your_life::rtweekend;
use the_rest_of_your_life::vec3::{self, Vec3};

fn f(d: Vec3) -> f64 {
  d.z() * d.z()
}

fn pdf(_d: Vec3) -> f64 {
  1.0 / (4.0 * rtweekend::PI)
}

fn main() {
  const N: usize = 1000000;
  let mut sum = 0.0;
  (0..N).for_each(|_| {
    let d = vec3::random_unit_vector();
    let f_d = f(d);
    sum += f_d / pdf(d);
  });
  println!("I = {:.12}", sum / N as f64);
}