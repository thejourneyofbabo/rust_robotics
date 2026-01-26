// TODO: Implement path planning algorithm

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
        println!("TODO: Not implemented yet");
    }
}
