use crate::Velocity;
use crate::physics::sd;
use std::f64::consts::PI;

use crate::{Coords, ecs::ECS};
pub fn gluon(ecs: &mut ECS, i: usize, pos_a: Coords) -> (f64, f64) {
    const SIGMA: f64 = 1.0;
    const SNAP: f64 = 10.0;
    let mut fx = 0.0;
    let mut fy = 0.0;

    let mut gluon_buff = Vec::new();
    let mut hadron = Vec::new();
    for j in 0..ecs.current.len() {
        if i == j {
            continue;
        }
        if let (Some(pos_b), Some(_chroma)) = (ecs.current[j], ecs.chroma[j]) {
            let dx = sd(pos_a.x, pos_b.x, 360.0);
            let dy = sd(pos_a.y, pos_b.y, 240.0);

            let distance = dx.powi(2) + dy.powi(2);
            let distance_norm = distance.sqrt();
            if let Some(string) = &ecs.string[j] {
                for &x in string {
                    if x == i {
                        if distance_norm > 1e-9 {
                            fx += SIGMA * (dx / distance_norm);
                            fy += SIGMA * (dy / distance_norm);
                        }

                        if distance_norm >= SNAP && i < j {
                            hadron.push((j, distance_norm));
                        }
                    }
                }
            }
            if ecs.charge[i].is_some() && ecs.charge[j].is_some() {
                if distance <= 0.22 / SIGMA.sqrt() {
                    let bonded = if let (Some(string_i), Some(string_j)) =
                        (&ecs.string[i], &ecs.string[j])
                    {
                        string_i.iter().any(|s| string_j.contains(s))
                    } else {
                        false
                    };
                    if !bonded && i < j {
                        gluon_buff.push((pos_a, pos_b, j));
                    }
                }
            }
        }
    }
    for (gid, distance) in hadron {
        if let Some(string) = &ecs.string[gid] {
            let q2 = string.iter().find(|&&idx| idx != i);
            if let (Some(&q2), Some(gpos)) = (q2, ecs.current[gid]) {
                let gdx = sd(pos_a.x, gpos.x, 360.0);
                let gdy = sd(pos_a.y, gpos.y, 240.0);

                let dist = (gdx.powi(2) + gdy.powi(2)).sqrt();
                let nx = gdx / dist;
                let ny = gdy / dist;
                let force = distance * 0.15;
                let offset = 2.5;

                let new_q = ecs.spawn(
                    Coords {
                        x: gpos.x + offset,
                        y: gpos.y + offset,
                    },
                    Some(Velocity {
                        x: -nx * force,
                        y: -ny * force,
                    }),
                    None,
                    Some(-1.0 / 3.0),
                    Some(1.0),
                    Some(vec![q2]),
                );
                let new_aq = ecs.spawn(
                    Coords {
                        x: gpos.x - offset,
                        y: gpos.y - offset,
                    },
                    Some(Velocity {
                        x: -nx * force,
                        y: -ny * force,
                    }),
                    None,
                    Some(2.0 / 3.0),
                    Some(1.0),
                    Some(vec![i]),
                );

                if let Some(links) = &mut ecs.string[i] {
                    if let Some(pos) = links.iter().position(|&x| x == gid) {
                        links[pos] = new_aq;
                    } else {
                        links.push(new_aq);
                    }
                } else {
                    ecs.string[i] = Some(vec![new_aq]);
                }
                if let Some(links) = &mut ecs.string[q2] {
                    if let Some(pos) = links.iter().position(|&x| x == gid) {
                        links[pos] = new_q;
                    } else {
                        links.push(new_q);
                    }
                } else {
                    ecs.string[q2] = Some(vec![new_aq]);
                }

                ecs.current[gid] = None;
                ecs.previous[gid] = None;
                ecs.velocity[gid] = None;
                ecs.chroma[gid] = None;
                ecs.charge[gid] = None;
                ecs.string[gid] = None;
            }
        }
    }
    for (pa, pb, j) in gluon_buff {
        let gluon = ecs.spawn(
            Coords {
                x: (pb.x + pa.x) * 0.5,
                y: (pb.y + pa.y) * 0.5,
            },
            Some(Velocity { x: 0.0, y: 0.0 }),
            None,
            None,
            Some(1.0),
            Some(vec![i, j]),
        );
        if let Some(links) = &mut ecs.string[i] {
            links.push(gluon);
        } else {
            ecs.string[i] = Some(vec![gluon])
        }
        if let Some(links) = &mut ecs.string[j] {
            links.push(gluon)
        } else {
            ecs.string[j] = Some(vec![gluon])
        }
    }
    (fx, fy)
}

fn string_len(width: f64, sigma: f64) -> f64 {
    let cutoff = 0.22 / sigma.sqrt();
    let exp = 2.0 * PI * sigma * width.powf(2.0);
    let string_len = cutoff * exp.exp();
    string_len
}
