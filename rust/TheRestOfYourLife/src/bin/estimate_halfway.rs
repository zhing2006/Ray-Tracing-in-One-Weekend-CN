use the_rest_of_your_life::rtweekend::{
  random_double_range, self,
};

struct Sample {
  pub x: f64,
  pub p_x: f64,
}

fn compare_by_x(a: &Sample, b: &Sample) -> std::cmp::Ordering {
  a.x.partial_cmp(&b.x).unwrap()
}

fn main() {
  const N: usize = 10000;
  let mut sum = 0.0;

  // iterate through all of our samples
  let mut samples = Vec::new();
  (0..N).for_each(|_| {
    // Get the area under the curve
    let x = random_double_range(0.0, 2.0 * rtweekend::PI);
    let sin_x = x.sin();
    let p_x = (-x / (2.0 * rtweekend::PI)).exp() * sin_x * sin_x;
    sum += p_x;
    // store this sample
    samples.push(Sample { x, p_x });
  });

  // Sort the samples by x
  samples.sort_by(compare_by_x);

  // Find out the sample at which we have half of our area
  let half_sum = sum / 2.0;
  let mut halfway_point = 0.0;
  let mut accum = 0.0;
  for sample in samples.iter() {
    accum += sample.p_x;
    if accum >= half_sum {
      halfway_point = sample.x;
      break;
    }
  }

  println!("Average = {:.12}", sum / N as f64);
  println!("Area under curve = {:.12}", 2.0 * rtweekend::PI * sum / N as f64);
  println!("Halfway = {:.12}", halfway_point);
}