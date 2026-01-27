use crate::utils::math;

pub struct GoalPosition {}
pub struct ObstaclePosition {}

pub fn use_math() -> i32 {
    let a = math::sum(2, 3);
    println!("{a}");
    a
}
