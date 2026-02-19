use crate::utils::viz::{GridViz, Point};
use std::collections::VecDeque;

// --- Types ---

struct Node {
    x_idx: i32,
    y_idx: i32,
    cost: f32,
    parent_idx: Option<usize>,
}

struct Motion {
    dx: i32,
    dy: i32,
    cost: f32,
}

const SQRT2: f32 = std::f32::consts::SQRT_2;

const MOTIONS: [Motion; 8] = [
    Motion { dx:  1, dy:  0, cost: 1.0 },
    Motion { dx:  0, dy:  1, cost: 1.0 },
    Motion { dx: -1, dy:  0, cost: 1.0 },
    Motion { dx:  0, dy: -1, cost: 1.0 },
    Motion { dx: -1, dy: -1, cost: SQRT2 },
    Motion { dx: -1, dy:  1, cost: SQRT2 },
    Motion { dx:  1, dy: -1, cost: SQRT2 },
    Motion { dx:  1, dy:  1, cost: SQRT2 },
];

// --- Grid Map ---

struct GridMap {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    resolution: f32,
    width: usize,
    height: usize,
    blocked: Vec<Vec<bool>>,
}

impl GridMap {
    fn build(obstacles: &[Point], resolution: f32, robot_radius: f32) -> Self {
        let min_x = obstacles.iter().map(|pt| pt.x).reduce(f32::min).unwrap();
        let min_y = obstacles.iter().map(|pt| pt.y).reduce(f32::min).unwrap();
        let max_x = obstacles.iter().map(|pt| pt.x).reduce(f32::max).unwrap();
        let max_y = obstacles.iter().map(|pt| pt.y).reduce(f32::max).unwrap();

        let width = ((max_x - min_x) / resolution).round() as usize;
        let height = ((max_y - min_y) / resolution).round() as usize;

        let mut blocked = vec![vec![false; height]; width];
        for x_idx in 0..width {
            let x = x_idx as f32 * resolution + min_x;
            for y_idx in 0..height {
                let y = y_idx as f32 * resolution + min_y;
                blocked[x_idx][y_idx] = obstacles
                    .iter()
                    .any(|pt| (pt.x - x).hypot(pt.y - y) <= robot_radius);
            }
        }

        Self { min_x, min_y, max_x, max_y, resolution, width, height, blocked }
    }

    fn pos_to_idx(&self, pos: f32, min: f32) -> i32 {
        ((pos - min) / self.resolution).round() as i32
    }

    fn index_to_pos(&self, index: usize, min: f32) -> f32 {
        index as f32 * self.resolution + min
    }

    fn flat_index(&self, x_idx: i32, y_idx: i32) -> usize {
        y_idx as usize * self.width + x_idx as usize
    }

    fn is_walkable(&self, x_idx: i32, y_idx: i32) -> bool {
        if x_idx < 0 || y_idx < 0 {
            return false;
        }
        let (ux, uy) = (x_idx as usize, y_idx as usize);
        ux < self.width && uy < self.height && !self.blocked[ux][uy]
    }
}

// --- BFS Planning ---

fn bfs_plan(
    grid: &GridMap,
    start: Point,
    goal: Point,
    mut on_explore: impl FnMut(Point),
) -> Vec<Point> {
    let start_x_idx = grid.pos_to_idx(start.x, grid.min_x);
    let start_y_idx = grid.pos_to_idx(start.y, grid.min_y);
    let goal_x_idx = grid.pos_to_idx(goal.x, grid.min_x);
    let goal_y_idx = grid.pos_to_idx(goal.y, grid.min_y);

    let mut nodes = vec![Node {
        x_idx: start_x_idx,
        y_idx: start_y_idx,
        cost: 0.0,
        parent_idx: None,
    }];
    let mut open_queue = VecDeque::new();
    let mut visited: Vec<Option<usize>> = vec![None; grid.width * grid.height];

    open_queue.push_back(0usize);
    visited[grid.flat_index(start_x_idx, start_y_idx)] = Some(0);

    let goal_node_idx = loop {
        let Some(current_idx) = open_queue.pop_front() else {
            return Vec::new();
        };
        let Node { x_idx, y_idx, cost, .. } = nodes[current_idx];

        on_explore(Point::new(
            grid.index_to_pos(x_idx as usize, grid.min_x),
            grid.index_to_pos(y_idx as usize, grid.min_y),
        ));

        if x_idx == goal_x_idx && y_idx == goal_y_idx {
            break current_idx;
        }

        for m in &MOTIONS {
            let (nx, ny) = (x_idx + m.dx, y_idx + m.dy);
            if !grid.is_walkable(nx, ny) {
                continue;
            }

            let flat_idx = grid.flat_index(nx, ny);
            if visited[flat_idx].is_some() {
                continue;
            }

            nodes.push(Node {
                x_idx: nx,
                y_idx: ny,
                cost: cost + m.cost,
                parent_idx: Some(current_idx),
            });
            let new_idx = nodes.len() - 1;
            visited[flat_idx] = Some(new_idx);
            open_queue.push_back(new_idx);
        }
    };

    // Backtrace
    let mut path = Vec::new();
    let mut trace_idx = goal_node_idx;
    loop {
        let node = &nodes[trace_idx];
        path.push(Point::new(
            grid.index_to_pos(node.x_idx as usize, grid.min_x),
            grid.index_to_pos(node.y_idx as usize, grid.min_y),
        ));
        match node.parent_idx {
            Some(idx) => trace_idx = idx,
            None => break,
        }
    }
    path.reverse();
    path
}

// --- Main ---

pub fn main() {
    let resolution = 2.0_f32;
    let robot_radius = 1.0_f32;

    let mut obstacles = Vec::new();
    for i in -10..60 { obstacles.push(Point::new(i as f32, -10.0)); }      // bottom wall
    for i in -10..60 { obstacles.push(Point::new(60.0, i as f32)); }       // right wall
    for i in -10..61 { obstacles.push(Point::new(i as f32, 60.0)); }       // top wall
    for i in -10..61 { obstacles.push(Point::new(-10.0, i as f32)); }      // left wall
    for i in -10..40 { obstacles.push(Point::new(20.0, i as f32)); }       // inner wall 1
    for i in 0..40   { obstacles.push(Point::new(40.0, 60.0 - i as f32)); } // inner wall 2

    let start = Point::new(10.0, 10.0);
    let goal = Point::new(50.0, 50.0);

    let grid = GridMap::build(&obstacles, resolution, robot_radius);

    let mut viz = GridViz::new(
        "BFS Path Planning",
        (grid.min_x, grid.max_x),
        (grid.min_y, grid.max_y),
    );

    viz.draw(&obstacles, start, goal, &[], &[]);

    let mut explored: Vec<Point> = Vec::new();
    let mut explore_count = 0u32;
    let path = bfs_plan(&grid, start, goal, |point| {
        explored.push(point);
        explore_count += 1;
        if explore_count % 10 == 0 {
            viz.draw(&obstacles, start, goal, &explored, &[]);
        }
    });

    println!("Find goal");

    viz.draw(&obstacles, start, goal, &explored, &path);
    viz.wait_close();
}
