use rustfft::{FftPlanner, num_complex::Complex};

fn main() {
    let mut planner = FftPlanner::<f32>::new();

    let fft = planner.plan_fft_forward(123);

    let mut buffer = vec![Complex{re: 0.0, im: 0.0}; 123];

    fft.process(&mut buffer);

    println!("{:?}", buffer);
}