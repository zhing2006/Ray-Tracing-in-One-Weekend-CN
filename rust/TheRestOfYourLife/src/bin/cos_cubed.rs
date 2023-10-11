use the_rest_of_your_life::rtweekend;

fn f(r2: f64) -> f64 {
  let z = 1.0 - r2;
  let cos_theta = z;
  cos_theta * cos_theta * cos_theta
}

fn pdf() -> f64 {
  1.0 / (2.0 * rtweekend::PI)
}

fn main() {
  const N: usize = 1000000;

  let mut sum = 0.0;
  (0..N).for_each(|_| {
    let r2 = rtweekend::random_double();
    sum += f(r2) / pdf();
  });

  println!("PI/2 = {:.12}", rtweekend::PI / 2.0);
  println!("Estimate = {:.12}", sum / N as f64);
}