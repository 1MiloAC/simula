use crate::ecs::{Coords, ECS, Velocity};
pub fn tick(ecs: &mut ECS, dt: f64) {
    const G: f64 = 6.6743 * 1e-11;

    for id in 0..ecs.current.len() {
        if let (Some(current), Some(previous), Some(vel), Some(mass)) = (
            &ecs.current[id],
            &mut ecs.previous[id],
            &ecs.velocity[id],
            &ecs.mass[id],
        ) {
            *previous = *current;
        }
    }
    for i in 0..ecs.current.len() {
        if ecs.current[i].is_none() || ecs.velocity[i].is_none() {
            continue;
        }

        let pos_a = ecs.current[i].unwrap();
        let mut fx = 0.0;
        let mut fy = 0.0;

        for j in 0..ecs.current.len() {
            if i == j {
                continue;
            }

            if let (Some(pos_b), Some(mass)) = (ecs.current[j], ecs.mass[j]) {
                let dx = pos_b.x - pos_a.x;
                let dy = pos_b.y - pos_a.y;

                let distance = dx.powi(2) + dy.powi(2);
                let distance_norm = distance.sqrt();

                let force = G * (mass as f64) / distance;

                fx += force * (dx / distance_norm);
                fy += force * (dy / distance_norm);
            }
        }
        if let Some(vel) = &mut ecs.velocity[i] {
            vel.x += fx * dt;
            vel.y += fy * dt;
        }
    }
    for id in 0..ecs.current.len() {
        if let (Some(current), Some(vel)) = (&mut ecs.current[id], &ecs.velocity[id]) {
            current.x += vel.x * dt;
            current.y += vel.y * dt;
        }
    }
}
