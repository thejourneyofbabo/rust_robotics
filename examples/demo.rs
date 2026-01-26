use rust_robotics::path_planning::demo_planner::Planner;

fn main() {
    println!("=== Demo ===");
    let p = Planner::new((0.0, 0.0), (5.0, 5.0));
    p.plan();
}
