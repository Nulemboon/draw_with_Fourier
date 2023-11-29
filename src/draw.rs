use flo_draw::*;
use flo_canvas::*;

// use rand::*;

use std::thread;
use std::time::{Duration};

// struct Circle {
//     sprite_id: SpriteId,
//     radius:  f32,
// }

// struct Epicycle {

//     circles: Vec<Circle>,
// }






pub fn main() {
    // 'with_2d_graphics' is used to support operating systems that can't run event loops anywhere other than the main thread
    with_2d_graphics(|| {
        // Create a window with a canvas to draw on
        let canvas = create_drawing_window("Epicycle");

        // Clear the canvas to set a background colour
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);
        });
        let mut time: f32 = 0.0;
        let mut wave: Vec<Draw> = vec![];
        loop {
            canvas.draw(|gc| {
                gc.layer(LayerId(0));
                gc.clear_layer();

                let mut x = 200.0;
                let mut y = 500.0;
                let n = 5;

                for i in 0..n {
                    let prevx = x;
                    let prevy = y;
                    let freq = i as f32 * 2.0 + 1.0;
                    let radius = 50.0 * (4.0 / (freq * 3.14));
                    
                    let point_x = radius * (time * freq).cos();
                    let point_y = radius * (time * freq).sin();
                    
                    x += point_x;
                    y += point_y;
                    //Draw circle
                    gc.new_path();
                    gc.circle(prevx, prevy, radius);
                    gc.line_width(2.0);
                    gc.stroke_color(Color::Rgba(1.0, 1.0, 1.0, 0.5));
                    gc.stroke();

                    
                    //Draw line and point
                    gc.new_path();
                    gc.move_to(prevx, prevy);
                    gc.line_to(x, y);
                    gc.stroke();

                    gc.new_path();
                    gc.circle(x, y, 5.0);
                    gc.fill_color(Color::Rgba(1.0, 1.0, 1.0, 1.0));
                    gc.fill();
                }
                

                time += 0.05;
                thread::sleep(Duration::from_millis(16));
            })
        }
        
    });
}
