use crate::utils::math;

//pub struct StartAndGoalPosition {
//    pub start: (f32, f32),
//    pub end: (f32, f32),
//}

#[derive(Debug)]
pub struct StartPosition {
    x: f32,
    y: f32,
}

pub struct GoalPosition {
    x: f32,
    y: f32,
}

pub fn use_math() -> i32 {
    let a = math::sum(2, 3);
    println!("{a}");
    a
}

pub fn main() {
    let start_position = StartPosition { x: 10.0, y: 10.0 };
    let goal_position = GoalPosition { x: 50.0, y: 50.0 };

    //println!("start = {:?}", start_position);

    let mut obstacle_x = Vec::new();
    let mut obstacle_y = Vec::new();

    for i in -10..=60 {
        obstacle_x.push(i as f32);
        obstacle_y.push(-10.0);
    }

    println!("obstacle_x = {:?}", obstacle_x);
}
