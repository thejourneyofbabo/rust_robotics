use crate::utils::math;

pub struct StartAndGoalPosition {
    pub start: (f32, f32),
    pub end: (f32, f32),
}

pub struct ObstaclePosition<T> {
    pub ox: Vec<T>,
    pub oy: Vec<T>,
}

pub fn use_math() -> i32 {
    let a = math::sum(2, 3);
    println!("{a}");
    a
}
