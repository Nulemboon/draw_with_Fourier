
use flo_draw::*;
use flo_canvas::*;
use flo_canvas::Draw::Path;
use flo_canvas::PathOp::{BezierCurve, Line};
use std::f32::consts::PI;
use svg2pts_lib::get_path_from_file;
use num::complex::Complex;
use std::thread;
use std::time::Duration;
use rustfft::{FftPlanner, num_complex::Complex as CComplex};

struct Coefs {
    amplitude: f32,
    phase: f32,
    freq: f32
}

fn fourier(points: &Vec<(f32, f32)>) -> Vec<Coefs> {
    points.iter().enumerate()
        .map(|(k, _)| {
            let mut sum: Complex<f32> = Complex::new(0.0, 0.0);
            for i in 0..points.len() {
                let (x, y) = points[i];
                let phi = (2.0 * PI * (k as f32) * (i as f32)) / points.len() as f32;
                let temp = Complex::new(x, y) * Complex::new(phi.cos(), -phi.sin());
                sum += temp;
            }
            
            sum.re /= points.len() as f32;
            sum.im /= points.len() as f32;   

            Coefs {
                amplitude: (sum.re * sum.re + sum.im * sum.im).sqrt(),
                phase: sum.im.atan2(sum.re),
                freq: k as f32,
            }

        })
        .collect()
}

fn fast_fourier(points: &Vec<(f32, f32)>) -> Vec<Coefs> {
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(points.len());

    let mut buffer = points.iter()
        .map(|(x, y)| {
            CComplex {
                re: *x,
                im: *y
            }
        }).collect::<Vec<CComplex<f32>>>();

    fft.process(&mut buffer);
    
    buffer.iter()
        .enumerate()
        .map(|(k, &x)| {
            let re = x.re / points.len() as f32;
            let im = x.im / points.len() as f32;
            Coefs {
                amplitude: (re * re + im * im).sqrt(),
                phase: im.atan2(re),
                freq: k as f32,
            }
        })
        .collect()
}

fn parse_svg(file_path: &str, point_num: u64, point_dist: f64) -> Vec<(f32, f32)> {
    get_path_from_file(file_path, point_num, point_dist)
        .iter()
        .map(|(x, y)| (*x as f32, *y as f32))
        .collect()
}

fn main() {
    let point_num = 500;
    let points = parse_svg("assets/exp.svg", point_num, 1.0);

    let mut drawing: Vec<Coefs> = fast_fourier(&points);
    drawing.sort_by(|d0, d1| d1.amplitude.partial_cmp(&d0.amplitude).unwrap());
    let n = drawing.len();
    // 'with_2d_graphics' is used to support operating systems that can't run event loops anywhere other than the main thread
    with_2d_graphics(move || {
        // Create a window with a canvas to draw on
        let canvas = create_drawing_window("Epicycle");

        // Clear the canvas to set a background colour
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
            gc.canvas_height(2000.0);
            gc.center_region(0.0, 0.0, 2000.0, 2000.0);
        });
        let mut time: f32 = 0.0;
        let mut wave: Vec<(f32, f32)> = vec![];
        // let mut wave: Vec<f32> = vec![];
        loop {
            canvas.draw(|gc| {
                gc.layer(LayerId(0));
                gc.clear_layer();

                let mut x = 1000.0;
                let mut y = 1000.0;

                for i in 1..n {
                    let prev_x = x;
                    let prev_y = y;
                    
                    let freq = drawing[i].freq;
                    let radius = drawing[i].amplitude * 1.1;
                    let phase = drawing[i].phase;
                    let point_x = radius * (time * freq + phase).cos();
                    let point_y = radius * (time * freq + phase).sin();
                    
                    x += point_x;
                    y += point_y;

                    //Draw circle
                    gc.new_path();
                    gc.circle(prev_x, prev_y, radius);
                    gc.line_width(3.0);
                    gc.stroke_color(Color::Rgba(1.0, 1.0, 1.0, 0.2));
                    gc.stroke();

                    
                    //Draw line and point
                    gc.new_path();
                    gc.move_to(prev_x, prev_y);
                    gc.line_to(x, y);
                    gc.stroke_color(Color::Rgba(1.0, 1.0, 1.0, 0.9));
                    gc.stroke();

                    gc.new_path();
                    gc.circle(x, y, 3.0);
                    gc.fill_color(Color::Rgba(1.0, 1.0, 1.0, 0.8));
                    gc.fill();

                    
                }


                wave.insert(0, (x, y));
                // wave.insert(0, y);

                // gc.new_path();
                // gc.move_to(x, y);
                // gc.line_to(750.0, wave[0]);
                // gc.stroke();

                // gc.new_path();
                // for i in 0..wave.len() {
                //     gc.draw(Path(Line(i as f32, wave[i])));
                // }
                // gc.stroke();

                
                gc.new_path();
                for i in 0..wave.len() {
                    gc.draw(Path(Line(wave[i].0 as f32, wave[i].1 as f32)));
                    // gc.draw(Path())
                }
                gc.stroke_color(Color::Rgba(0.19, 0.4, 0.6, 1.0));
                gc.line_width(5.0);
                gc.stroke();

                time += (2.0 * PI) / n as f32;

                if time >= 4.0 * 3.14 {
                    time = 0.0;
                    wave.clear();
                }
                thread::sleep(Duration::from_millis(17));
            })
        }
    });
}


