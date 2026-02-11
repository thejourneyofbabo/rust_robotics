use minifb::{Key, Window, WindowOptions};
use plotters::prelude::*;

const WIN_W: u32 = 800;
const WIN_H: u32 = 800;

// --- Shared types ---

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

// --- Grid Visualizer ---

pub struct GridViz {
    window: Window,
    buf_rgb: Vec<u8>,
    buf_u32: Vec<u32>,
    sz_title: String,
    f_x_range: (f64, f64),
    f_y_range: (f64, f64),
}

impl GridViz {
    pub fn new(sz_title: &str, f_x_range: (f32, f32), f_y_range: (f32, f32)) -> Self {
        let n_pixels = (WIN_W * WIN_H) as usize;
        let window = Window::new(
            sz_title,
            WIN_W as usize,
            WIN_H as usize,
            WindowOptions::default(),
        )
        .expect("Failed to create window");

        Self {
            window,
            buf_rgb: vec![0u8; n_pixels * 3],
            buf_u32: vec![0u32; n_pixels],
            sz_title: sz_title.to_string(),
            f_x_range: (f_x_range.0 as f64, f_x_range.1 as f64),
            f_y_range: (f_y_range.0 as f64, f_y_range.1 as f64),
        }
    }

    pub fn draw(
        &mut self,
        v_obs: &[Point],
        pt_start: Point,
        pt_goal: Point,
        v_explored: &[Point],
        v_path: &[Point],
    ) {
        {
            let root =
                BitMapBackend::with_buffer(&mut self.buf_rgb, (WIN_W, WIN_H)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let mut chart = ChartBuilder::on(&root)
                .margin(10)
                .caption(&self.sz_title, ("sans-serif", 20))
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(
                    self.f_x_range.0..self.f_x_range.1,
                    self.f_y_range.0..self.f_y_range.1,
                )
                .unwrap();

            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(
                    v_obs
                        .iter()
                        .map(|pt| Circle::new((pt.x as f64, pt.y as f64), 2, BLACK.filled())),
                )
                .unwrap();

            chart
                .draw_series(
                    v_explored
                        .iter()
                        .map(|pt| Circle::new((pt.x as f64, pt.y as f64), 3, CYAN.filled())),
                )
                .unwrap();

            chart
                .draw_series(std::iter::once(Circle::new(
                    (pt_start.x as f64, pt_start.y as f64),
                    6,
                    GREEN.filled(),
                )))
                .unwrap();

            chart
                .draw_series(std::iter::once(Circle::new(
                    (pt_goal.x as f64, pt_goal.y as f64),
                    6,
                    BLUE.filled(),
                )))
                .unwrap();

            if v_path.len() > 1 {
                chart
                    .draw_series(LineSeries::new(
                        v_path.iter().map(|pt| (pt.x as f64, pt.y as f64)),
                        RED.stroke_width(2),
                    ))
                    .unwrap();
            }

            root.present().unwrap();
        }

        for (i, pixel) in self.buf_u32.iter_mut().enumerate() {
            let r = self.buf_rgb[i * 3] as u32;
            let g = self.buf_rgb[i * 3 + 1] as u32;
            let b = self.buf_rgb[i * 3 + 2] as u32;
            *pixel = (r << 16) | (g << 8) | b;
        }

        self.window
            .update_with_buffer(&self.buf_u32, WIN_W as usize, WIN_H as usize)
            .unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn wait_close(&mut self) {
        while self.is_open() {
            self.window
                .update_with_buffer(&self.buf_u32, WIN_W as usize, WIN_H as usize)
                .unwrap();
        }
    }
}
