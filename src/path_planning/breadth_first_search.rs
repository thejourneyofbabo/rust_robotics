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
    fn new(x: f32, y: f32) -> Point {
        Point{x, y}
    }
}

struct Node {
    x_idx: i32,
    y_idx: i32,
    cost: f32,
    parent_idx: i32,
    parent: Option<Box<Node>>,
}

impl Node {

}

struct BFSPlanning {
    obstacle_x: Vec<f32>,
    obstacle_y: Vec<f32>,
    grid_size: f32,
    robot_radius: f32,
}

impl BFSPlanning {
    fn new(obstacle_x: Vec<f32>, obstacle_y: Vec<f32>, grid_size: f32, robot_radius: f32) -> Self {
        Self {
            obstacle_x,
            obstacle_y,
            grid_size,
            robot_radius,
        }
    }

    fn planning(&self, start_position: Point, goal_position: Point) -> Vec<Point> {
        let mut test_vec = Vec::new();
        test_vec.push(start_position);

        test_vec

        /* Breadth First Search based Planning 
         *
         * input: 
         *      start_position, goal_position
         * output:
         *      route_xy: Point list of final path 
         */


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
