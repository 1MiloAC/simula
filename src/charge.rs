use crate::physics::sd;
use crate::{Coords, ecs::ECS};
pub fn charge(ecs: &mut ECS, i: usize, pos_a: Coords) -> (f64, f64) {
    const K: f64 = 8.987e3;
    let mut fx = 0.0;
    let mut fy = 0.0;

    let charge_a = match ecs.charge[i] {
        Some(c) => c,
        None => return (0.0, 0.0),
    };
    for j in 0..ecs.current.len() {
        if i == j {
            continue;
        }

        if let (Some(pos_b), Some(charge_b)) = (ecs.current[j], ecs.charge[j]) {
            let dx = sd(pos_a.x, pos_b.x, 360.0);
            let dy = sd(pos_a.y, pos_b.y, 240.0);

            let distance = dx.powi(2) + dy.powi(2) + 10.0;
            let distance_norm = distance.sqrt();

            let force = -K * (charge_a * charge_b) / distance;

            fx += force * (dx / distance_norm);
            fy += force * (dy / distance_norm);
        }
    }
    (fx, fy)
}
