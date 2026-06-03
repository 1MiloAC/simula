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
    gravity::tick,
};

pub mod ecs;
pub mod gravity;

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
        Coords { x: 150.0, y: 100.0 },
        Some(Velocity { x: 5.0, y: -12.0 }),
        5_000_000_000_000.0,
    );
    ecs.spawn(
        Coords { x: 180.0, y: 100.0 },
        Some(Velocity { x: 0.0, y: 0.0 }),
        500.0,
    );
    ecs.spawn(
        Coords { x: 180.0, y: 20.0 },
        Some(Velocity { x: 0.0, y: 0.0 }),
        500.0,
    );
    ecs.spawn(
        Coords { x: 250.0, y: 120.0 },
        Some(Velocity { x: -1.0, y: -9.0 }),
        50_000_000_000_000.0,
    );
    ecs.spawn(Coords { x: 180.0, y: 120.0 }, None, 80_000_000_000_000.0);

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
                WindowEvent::Size(width, height) => {
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
            if let (Some(current), Some(previous)) = (&ecs.current[id], &ecs.previous[id]) {
                let ix = (current.x * alpha) + (previous.x * (1.0 - alpha));
                let iy = (current.y * alpha) + (previous.y * (1.0 - alpha));

                let sx = ix.round() as i32;
                let sy = iy.round() as i32;

                if sx >= 0 && sx < WIDTH as i32 && sy >= 0 && sy < HEIGHT as i32 {
                    let idx = ((sy * WIDTH as i32) + sx) as usize * 4;
                    frame[idx] = 255;
                    frame[idx + 1] = 255;
                    frame[idx + 2] = 255;
                    frame[idx + 3] = 255;
                }
            }
        }
        pixels.render()?;
        glfw.poll_events();

        if let Err(err) = pixels.render() {
            break;
        }
    }

    Ok(())
}
