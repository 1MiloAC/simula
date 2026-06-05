use glfw::{Action, Context, Key, WindowEvent};
use pixels::{
    Pixels, SurfaceTexture,
    wgpu::{naga::valid::WidthError, wgc::MAX_VERTEX_BUFFERS},
};
use std::{
    thread::current,
    time::{Duration, Instant},
};

use crate::{
    ecs::{Coords, ECS, Velocity},
    physics::{sd, tick},
};

pub mod charge;
pub mod ecs;
pub mod gluon;
pub mod gravity;
pub mod physics;

const WIDTH: u32 = 360;
const HEIGHT: u32 = 240;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let (mut window, events) = glfw
        .create_window(1080, 720, "test", glfw::WindowMode::Windowed)
        .unwrap();

    window.make_current();
    window.set_key_polling(true);
    window.set_size_polling(true);

    let (w, h) = window.get_framebuffer_size();
    let surface_texture = SurfaceTexture::new(w as u32, h as u32, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
    let dt = Duration::from_secs_f64(1.0 / 60.0);
    let max_ft = Duration::from_secs_f64(0.25);

    let mut current_t = Instant::now();
    let mut accumulator = Duration::ZERO;
    let mut t = Duration::ZERO;

    let mut ecs = ECS::new();
    ecs.spawn(
        Coords { x: 180.0, y: 180.0 },
        Some(Velocity { x: 5.0, y: 0.0 }),
        None,
        Some(2.0 / 3.0),
        Some(1.0),
        None,
    );
    ecs.spawn(
        Coords { x: 180.0, y: 120.0 },
        None,
        Some(50_000_000_000_000.0),
        None,
        None,
        None,
    );
    ecs.spawn(
        Coords { x: 200.0, y: 10.0 },
        Some(Velocity { x: -10.0, y: 0.0 }),
        None,
        Some(2.0 / 3.0),
        Some(1.0),
        None,
    );
    ecs.spawn(
        Coords { x: 200.0, y: 180.0 },
        Some(Velocity { x: -10.0, y: 0.0 }),
        None,
        Some(-1.0 / 3.0),
        Some(1.0),
        None,
    );
    ecs.spawn(
        Coords { x: 190.0, y: 190.0 },
        Some(Velocity { x: -1.0, y: -9.0 }),
        None,
        Some(-1.0 / 3.0),
        Some(1.0),
        None,
    );

    while !window.should_close() {
        let new_t = Instant::now();
        let mut ft = new_t - current_t;

        if ft > max_ft {
            ft = max_ft;
        }
        current_t = new_t;
        accumulator += ft;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    pixels.resize_surface(width as u32, height as u32)?;
                }
                _ => {}
            }
        }
        while accumulator >= dt {
            for id in 0..ecs.next_id {
                if let (Some(current), Some(previous)) =
                    (&mut ecs.current[id], &mut ecs.previous[id])
                {
                    *previous = *current;
                }
            }
            tick(&mut ecs, dt.as_secs_f64());

            t += dt;
            accumulator -= dt;
        }
        let alpha = accumulator.as_secs_f64() / dt.as_secs_f64();
        let frame = pixels.frame_mut();
        frame.fill(0);

        for id in 0..ecs.next_id {
            if let (Some(current), Some(previous), Some(string)) =
                (&ecs.current[id], &ecs.previous[id], &ecs.string[id])
            {
                let ax = ((current.x * alpha) + (previous.x * (1.0 - alpha))).round() as i32;
                let ay = ((current.y * alpha) + (previous.y * (1.0 - alpha))).round() as i32;

                for &tid in string {
                    if tid < ecs.next_id && id < tid {
                        if let (Some(current), Some(previous)) =
                            (&ecs.current[tid], &ecs.previous[tid])
                        {
                            let bx =
                                ((current.x * alpha) + (previous.x * (1.0 - alpha))).round() as i32;
                            let by =
                                ((current.y * alpha) + (previous.y * (1.0 - alpha))).round() as i32;
                            drawline(frame, ax, bx, ay, by);
                        }
                    }
                }
            }
        }
        for id in 0..ecs.next_id {
            if let (Some(current), Some(previous)) = (&ecs.current[id], &ecs.previous[id]) {
                let ix = (current.x * alpha) + (previous.x * (1.0 - alpha));
                let iy = (current.y * alpha) + (previous.y * (1.0 - alpha));

                let sx = ix.round() as i32;
                let sy = iy.round() as i32;

                if sx >= 0 && sx < WIDTH as i32 && sy >= 0 && sy < HEIGHT as i32 {
                    let idx = ((sy * WIDTH as i32) + sx) as usize * 4;

                    let (r, g, b) = match ecs.charge[id] {
                        Some(c) if c > 0.0 => (0, 200, 255),
                        Some(c) if c < 0.0 => (255, 50, 50),
                        _ => (255, 255, 255),
                    };

                    frame[idx] = r;
                    frame[idx + 1] = g;
                    frame[idx + 2] = b;
                    frame[idx + 3] = 255;
                }
            }
        }
        if let Err(err) = pixels.render() {
            break;
        }
    }

    Ok(())
}
fn drawline(frame: &mut [u8], x0: i32, x1: i32, y0: i32, y1: i32) {
    let x1 = x0 + sd(x0 as f64, x1 as f64, 360.0) as i32;
    let y1 = y0 + sd(y0 as f64, y1 as f64, 360.0) as i32;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        let screen_x = x.rem_euclid(WIDTH as i32);
        let screen_y = y.rem_euclid(WIDTH as i32);
        if screen_x >= 0 && screen_x < WIDTH as i32 && screen_y >= 0 && screen_y < HEIGHT as i32 {
            let idx = ((screen_y * WIDTH as i32) + screen_x) as usize * 4;
            if idx + 3 < frame.len() {
                frame[idx] = 128;
                frame[idx + 1] = 128;
                frame[idx + 2] = 128;
                frame[idx + 3] = 100;
            }
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * error;
        if e2 >= dy {
            error += dy;
            x += sx;
        }
        if e2 <= dx {
            error += dx;
            y += sy;
        }
    }
}
