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
        Point{
            x: x,
            y: y,
        }
    }
}

struct BFSPlanning {
    start_position: Point,
    goal_position: Point,
}

impl BFSPlanning {

}


fn use_math() -> i32 {
    let a = math::sum(2, 3);
    println!("{a}");
    a
}

pub fn main() {
    let scenario_1 = BFSPlanning {
        start_position: Point::new(10.0, 10.0),
        goal_position: Point::new(50.0, 50.0),
    };

    // println!("start = {:?}", scenario_0.start_position);

    let mut obstacle_x = Vec::new();    // Obstacles x position
    let mut obstacle_y = Vec::new();    // Obstacles y position
    let grid_size = 2.0;                // [m]
    let robot_radius = 1.0;             // [m]

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




    // println!("use math test {}", use_math());
    // println!("obstacle_x = {:?}", obstacle_x);
}
