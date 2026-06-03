#[derive(Clone, Copy, Debug)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}
pub struct ECS {
    pub current: Vec<Option<Coords>>,
    pub previous: Vec<Option<Coords>>,
    pub velocity: Vec<Option<Velocity>>,
    pub mass: Vec<Option<f64>>,
    pub next_id: usize,
}
impl ECS {
    pub fn new() -> Self {
        Self {
            current: Vec::new(),
            previous: Vec::new(),
            velocity: Vec::new(),
            mass: Vec::new(),
            next_id: 0,
        }
    }
    pub fn spawn(&mut self, current: Coords, vel: Option<Velocity>, mass: f64) -> usize {
        let id = self.next_id;

        self.current.push(Some(current));
        self.previous.push(Some(current));
        self.velocity.push(vel);
        self.mass.push(Some(mass));
        self.next_id += 1;
        id
    }
}
