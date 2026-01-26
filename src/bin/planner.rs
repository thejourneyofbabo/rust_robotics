use rust_robotics::path_planning::demo_planner::Planner;

fn main() {
    println!("=== Path Planner ===");

    let p = Planner::new((0.0, 0.0), (10.0, 10.0));
    p.plan();
}
