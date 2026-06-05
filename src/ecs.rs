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
    pub charge: Vec<Option<f64>>,
    pub chroma: Vec<Option<f64>>,
    pub string: Vec<Option<Vec<usize>>>,
    pub next_id: usize,
}
impl ECS {
    pub fn new() -> Self {
        Self {
            current: Vec::new(),
            previous: Vec::new(),
            velocity: Vec::new(),
            mass: Vec::new(),
            charge: Vec::new(),
            chroma: Vec::new(),
            string: Vec::new(),
            next_id: 0,
        }
    }
    pub fn spawn(
        &mut self,
        current: Coords,
        vel: Option<Velocity>,
        mass: Option<f64>,
        charge: Option<f64>,
        chroma: Option<f64>,
        string: Option<Vec<usize>>,
    ) -> usize {
        let id = self.next_id;

        self.current.push(Some(current));
        self.previous.push(Some(current));
        self.velocity.push(vel);
        self.mass.push(mass);
        self.charge.push(charge);
        self.chroma.push(chroma);
        self.string.push(string);
        self.next_id += 1;
        id
    }
}
