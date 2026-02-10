use crate::utils::math;

//pub struct StartAndGoalPosition {
//    pub start: (f32, f32),
//    pub end: (f32, f32),
//}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
}

struct Node {
    x_idx: i32,
    y_idx: i32,
    cost: f32,
    parent: Option<usize>,
}

impl Node {
    fn new(x_idx: i32, y_idx: i32, cost: f32, parent: Option<usize>) -> Self {
        Self { x_idx, y_idx, cost, parent}
    }
}

struct ObstacleMap {
    obs_min_x: f32,
    obs_min_y: f32, 
    obs_max_x: f32,
    obs_max_y: f32,
    obs_x_width: i32,
    obs_y_width: i32,
    obmap: Vec<Vec<bool>>,
}

struct BFSPlanning {
    obstacle_x: Vec<f32>,
    obstacle_y: Vec<f32>,
    grid_size: f32,
    robot_radius: f32,
    obstacle_map: ObstacleMap,
}

impl BFSPlanning {
    fn new(obstacle_x: Vec<f32>, obstacle_y: Vec<f32>, grid_size: f32, robot_radius: f32) -> Self {
        let obstacle_map = Self::calc_obstacle_map(&obstacle_x, &obstacle_y, grid_size, robot_radius);
        Self {
            obstacle_x,
            obstacle_y,
            grid_size,  // resolution
            robot_radius,
            obstacle_map,
        }
    }

    fn calc_grid_position(index: usize, min_val: &f32, grid_size: &f32) -> f32 {
        let pos = index as f32 * grid_size + min_val;
        pos
    }

    fn calc_obstacle_map(ox: &[f32], oy: &[f32], grid_size: f32, robot_radius: f32) -> ObstacleMap {  // Need to Fix
        let obs_min_x = ox.iter().min().unwrap();
        let obs_min_y = oy.iter().min().unwrap();
        let obs_max_x = ox.iter().max().unwrap();
        let obs_max_y = oy.iter().max().unwrap();

        let obs_x_width = ((obs_max_x - obs_min_x) / grid_size).round();
        let obs_y_width = ((obs_max_y - obs_min_y) / grid_size).round();

        let mut obmap = vec![vec![false; obs_y_width]; obs_x_width];    // Initialize vector
        for ix in 0..obs_x_width {
            let x = Self::calc_grid_position(ix, &obs_min_x, &grid_size);
            for iy in 0..obs_y_width {
                let y = Self::calc_grid_position(iy, &obs_min_y, &grid_size);
                for (&iox, &ioy) in ox.iter().zip(oy.iter()) {
                    let d = ((iox - x).powi(2) + (ioy - y).powi(2)).sqrt();
                    if d <= robot_radius {
                        obmap[ix][iy] = true;
                        break;
                    }
                }

            }
        }

        ObstacleMap {
            obs_min_x,
            obs_min_y,
            obs_max_x,
            obs_max_y,
            obs_x_width,
            obs_y_width,
            obmap,
        }

    }

    fn calc_xyindex(self, position: Point,  ){}

    fn planning(&self, start_position: Point, goal_position: Point) -> Vec<Point> {
        let mut test_vec = Vec::new();
        test_vec.push(start_position);

        /* Breadth First Search based Planning 
         *
         * input: 
         *      start_position, goal_position
         * output:
         *      route_xy: Point list of final path 
         */
        
        let mut nodes: Vec<Node> = Vec::new();


        
        test_vec


    }

}


fn use_math() -> i32 {
    let a = math::sum(2, 3);
    println!("{a}");
    a
}

pub fn main() {
    // Initialization
    let grid_size = 2.0;                // [m]
    let robot_radius = 1.0;             // [m]
    let mut obstacle_x = Vec::new();    // Obstacles x position
    let mut obstacle_y = Vec::new();    // Obstacles y position

    // Locate Obstacles
    for i in -10..=60 {
        obstacle_x.push(i as f32);
        obstacle_y.push(-10.0);
    }
    for i in -10..=60 {
        obstacle_x.push(60.0);
        obstacle_y.push(i as f32);
    }
    for i in -10..=61 {
        obstacle_x.push(i as f32);
        obstacle_y.push(60.0);
    }
    for i in -10..=61 {
        obstacle_x.push(-10.0);
        obstacle_y.push(i as f32);
    }
    for i in -10..=40 {
        obstacle_x.push(20.0);
        obstacle_y.push(i as f32);
    }
    for i in 0..=40 {
        obstacle_x.push(40.0);
        obstacle_y.push(60.0 - i as f32);
    }

    let start_position = Point::new(10.0, 10.0);
    let goal_position = Point::new(50.0, 50.0);
    let scenario_1 = BFSPlanning::new(obstacle_x, obstacle_y, grid_size, robot_radius);
    // println!("scenario_1: grid_size {}", scenario_1.grid_size);

    // let mut route_xy: Vec<Point> = Vec::new();
    let route_xy: Vec<Point> = scenario_1.planning(start_position, goal_position);



    // println!("use math test {}", use_math());
    // println!("obstacle_x = {:?}", obstacle_x);
}
