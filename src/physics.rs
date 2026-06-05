use crate::{charge, ecs::ECS, gluon, gravity};
pub fn tick(ecs: &mut ECS, dt: f64) {
    for id in 0..ecs.current.len() {
        if let (Some(current), Some(previous)) = (&ecs.current[id], &mut ecs.previous[id]) {
            *previous = *current;
        }
    }

    for i in 0..ecs.current.len() {
        let pos_a = match ecs.current[i] {
            Some(p) => p,
            None => continue,
        };

        let mut fx = 0.0;
        let mut fy = 0.0;

        let (gx, gy) = gravity::gravity(ecs, i, pos_a);
        fx += gx;
        fy += gy;
        let (qx, qy) = gluon::gluon(ecs, i, pos_a);
        fx += qx;
        fy += qy;
        let (cx, cy) = charge::charge(ecs, i, pos_a);
        fx += cx;
        fy += cy;

        if let Some(vel) = &mut ecs.velocity[i] {
            vel.x += fx * dt;
            vel.y += fy * dt;

            if let Some(current) = &mut ecs.current[i] {
                current.x += vel.x * dt;
                current.y += vel.y * dt;

                if current.x < 0.0 {
                    current.x += 360.0;
                }
                if current.x >= 360.0 {
                    current.x -= 360.0;
                }
                if current.y < 0.0 {
                    current.y += 240.0;
                }
                if current.y >= 240.0 {
                    current.y -= 240.0;
                }
            }
        }
    }
}
pub fn sd(pos_a: f64, pos_b: f64, size: f64) -> f64 {
    let mut sd = pos_b - pos_a;

    if sd > size * 0.5 {
        sd -= size;
    } else if sd < -size * 0.5 {
        sd += size;
    }
    sd
}
