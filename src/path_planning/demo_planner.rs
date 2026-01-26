// TODO: Path planning 알고리즘 구현 예정

pub struct Planner {
    pub start: (f64, f64),
    pub goal: (f64, f64),
}

impl Planner {
    pub fn new(start: (f64, f64), goal: (f64, f64)) -> Self {
        Self { start, goal }
    }

    pub fn plan(&self) {
        println!("Planning from {:?} to {:?}", self.start, self.goal);
        println!("TODO: 구현 예정");
    }
}
