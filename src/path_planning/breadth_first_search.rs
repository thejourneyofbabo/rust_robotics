use crate::utils::viz::{GridViz, Point};
use std::collections::VecDeque;

// --- Types ---

struct Node {
    idx_x: i32,
    idx_y: i32,
    f_cost: f32,
    idx_parent: Option<usize>,
}

struct Motion {
    dx: i32,
    dy: i32,
    f_cost: f32,
}

const SQRT2: f32 = std::f32::consts::SQRT_2;

const MOTIONS: [Motion; 8] = [
    Motion { dx:  1, dy:  0, f_cost: 1.0 },
    Motion { dx:  0, dy:  1, f_cost: 1.0 },
    Motion { dx: -1, dy:  0, f_cost: 1.0 },
    Motion { dx:  0, dy: -1, f_cost: 1.0 },
    Motion { dx: -1, dy: -1, f_cost: SQRT2 },
    Motion { dx: -1, dy:  1, f_cost: SQRT2 },
    Motion { dx:  1, dy: -1, f_cost: SQRT2 },
    Motion { dx:  1, dy:  1, f_cost: SQRT2 },
];

// --- Grid Map ---

struct GridMap {
    f_min_x: f32,
    f_min_y: f32,
    f_max_x: f32,
    f_max_y: f32,
    f_resolution: f32,
    n_width: usize,
    n_height: usize,
    vv_blocked: Vec<Vec<bool>>,
}

impl GridMap {
    fn build(v_obs: &[Point], f_resolution: f32, f_radius: f32) -> Self {
        let f_min_x = v_obs.iter().map(|pt| pt.x).reduce(f32::min).unwrap();
        let f_min_y = v_obs.iter().map(|pt| pt.y).reduce(f32::min).unwrap();
        let f_max_x = v_obs.iter().map(|pt| pt.x).reduce(f32::max).unwrap();
        let f_max_y = v_obs.iter().map(|pt| pt.y).reduce(f32::max).unwrap();

        let n_width = ((f_max_x - f_min_x) / f_resolution).round() as usize;
        let n_height = ((f_max_y - f_min_y) / f_resolution).round() as usize;

        let mut vv_blocked = vec![vec![false; n_height]; n_width];
        for idx_x in 0..n_width {
            let fx = idx_x as f32 * f_resolution + f_min_x;
            for idx_y in 0..n_height {
                let fy = idx_y as f32 * f_resolution + f_min_y;
                vv_blocked[idx_x][idx_y] = v_obs
                    .iter()
                    .any(|pt| (pt.x - fx).hypot(pt.y - fy) <= f_radius);
            }
        }

        Self { f_min_x, f_min_y, f_max_x, f_max_y, f_resolution, n_width, n_height, vv_blocked }
    }

    fn pos_to_idx(&self, f_pos: f32, f_min: f32) -> i32 {
        ((f_pos - f_min) / self.f_resolution).round() as i32
    }

    fn idx_to_pos(&self, idx: usize, f_min: f32) -> f32 {
        idx as f32 * self.f_resolution + f_min
    }

    fn flat_index(&self, idx_x: i32, idx_y: i32) -> usize {
        idx_y as usize * self.n_width + idx_x as usize
    }

    fn is_walkable(&self, idx_x: i32, idx_y: i32) -> bool {
        if idx_x < 0 || idx_y < 0 {
            return false;
        }
        let (ux, uy) = (idx_x as usize, idx_y as usize);
        ux < self.n_width && uy < self.n_height && !self.vv_blocked[ux][uy]
    }
}

// --- BFS Planning ---

fn bfs_plan(
    grid: &GridMap,
    pt_start: Point,
    pt_goal: Point,
    mut on_explore: impl FnMut(Point),
) -> Vec<Point> {
    let idx_sx = grid.pos_to_idx(pt_start.x, grid.f_min_x);
    let idx_sy = grid.pos_to_idx(pt_start.y, grid.f_min_y);
    let idx_gx = grid.pos_to_idx(pt_goal.x, grid.f_min_x);
    let idx_gy = grid.pos_to_idx(pt_goal.y, grid.f_min_y);

    let mut v_nodes = vec![Node {
        idx_x: idx_sx,
        idx_y: idx_sy,
        f_cost: 0.0,
        idx_parent: None,
    }];
    let mut q_open = VecDeque::new();
    let mut v_visited: Vec<Option<usize>> = vec![None; grid.n_width * grid.n_height];

    q_open.push_back(0usize);
    v_visited[grid.flat_index(idx_sx, idx_sy)] = Some(0);

    let idx_goal = loop {
        let Some(idx_cur) = q_open.pop_front() else {
            return Vec::new();
        };
        let Node { idx_x, idx_y, f_cost, .. } = v_nodes[idx_cur];

        on_explore(Point::new(
            grid.idx_to_pos(idx_x as usize, grid.f_min_x),
            grid.idx_to_pos(idx_y as usize, grid.f_min_y),
        ));

        if idx_x == idx_gx && idx_y == idx_gy {
            break idx_cur;
        }

        for m in &MOTIONS {
            let (nx, ny) = (idx_x + m.dx, idx_y + m.dy);
            if !grid.is_walkable(nx, ny) {
                continue;
            }

            let idx_flat = grid.flat_index(nx, ny);
            if v_visited[idx_flat].is_some() {
                continue;
            }

            v_nodes.push(Node {
                idx_x: nx,
                idx_y: ny,
                f_cost: f_cost + m.f_cost,
                idx_parent: Some(idx_cur),
            });
            let idx_new = v_nodes.len() - 1;
            v_visited[idx_flat] = Some(idx_new);
            q_open.push_back(idx_new);
        }
    };

    // Backtrace
    let mut v_path = Vec::new();
    let mut idx_trace = idx_goal;
    loop {
        let node = &v_nodes[idx_trace];
        v_path.push(Point::new(
            grid.idx_to_pos(node.idx_x as usize, grid.f_min_x),
            grid.idx_to_pos(node.idx_y as usize, grid.f_min_y),
        ));
        match node.idx_parent {
            Some(idx) => idx_trace = idx,
            None => break,
        }
    }
    v_path.reverse();
    v_path
}

// --- Main ---

pub fn main() {
    let f_resolution = 2.0_f32;
    let f_radius = 1.0_f32;

    let mut v_obs = Vec::new();
    for i in -10..60 { v_obs.push(Point::new(i as f32, -10.0)); }      // bottom wall
    for i in -10..60 { v_obs.push(Point::new(60.0, i as f32)); }       // right wall
    for i in -10..61 { v_obs.push(Point::new(i as f32, 60.0)); }       // top wall
    for i in -10..61 { v_obs.push(Point::new(-10.0, i as f32)); }      // left wall
    for i in -10..40 { v_obs.push(Point::new(20.0, i as f32)); }       // inner wall 1
    for i in 0..40   { v_obs.push(Point::new(40.0, 60.0 - i as f32)); } // inner wall 2

    let pt_start = Point::new(10.0, 10.0);
    let pt_goal = Point::new(50.0, 50.0);

    let grid = GridMap::build(&v_obs, f_resolution, f_radius);

    let mut viz = GridViz::new(
        "BFS Path Planning",
        (grid.f_min_x, grid.f_max_x),
        (grid.f_min_y, grid.f_max_y),
    );

    viz.draw(&v_obs, pt_start, pt_goal, &[], &[]);

    let mut v_explored: Vec<Point> = Vec::new();
    let mut n_count = 0u32;
    let v_path = bfs_plan(&grid, pt_start, pt_goal, |pt| {
        v_explored.push(pt);
        n_count += 1;
        if n_count % 10 == 0 {
            viz.draw(&v_obs, pt_start, pt_goal, &v_explored, &[]);
        }
    });

    println!("Find goal");

    viz.draw(&v_obs, pt_start, pt_goal, &v_explored, &v_path);
    viz.wait_close();
}
