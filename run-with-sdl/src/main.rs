use std::i32;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

struct Engine {
    count: usize,
    frame_buffer: [u8; WIDTH * HEIGHT * 4],
}

impl Engine {
    fn new() -> Self {
        Engine {
            count: 0,
            frame_buffer: [0; WIDTH * HEIGHT * 4],
        }
    }

    #[rustracy::zone_scoped_prefix(Engine)]
    fn do_task(&mut self, repeat: usize) {
        for i in 0..repeat {
            self.count = self.count.wrapping_add(i);
        }
    }

    #[rustracy::zone_scoped]
    fn execute_frame(&mut self) -> Option<&[u8]> {
        // Fill frame buffer...
        Some(self.frame_buffer.as_ref())
    }
}

enum ExecuteResult {
    Continue,
    Exit,
}

#[rustracy::zone_scoped]
fn execute_frame(
    engine: &mut Engine,
    event_pump: &mut sdl2::EventPump,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> ExecuteResult {
    engine.do_task(10000);

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return ExecuteResult::Exit;
            }
            _ => {}
        }
    }

    let _frame_buffer = engine.execute_frame();

    canvas.clear();
    canvas.present();
    match engine.count % 30 {
        0 => canvas.set_draw_color(Color::RGB(255, 0, 0)),
        1 => canvas.set_draw_color(Color::RGB(0, 255, 0)),
        2 => canvas.set_draw_color(Color::RGB(0, 0, 255)),
        _ => canvas.set_draw_color(Color::RGB(0, 0, 0)),
    };

    ExecuteResult::Continue
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("run-with-sdl", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let duration_per_frame = Duration::from_secs_f64(1.0 / 60.0);
    let mut next_frame_time = Instant::now() + duration_per_frame;
    let mut count: i32 = 0;
    let mut event_pump = sdl_context.event_pump()?;

    let mut engine = Engine::new();

    'running: loop {
        rustracy::emit_frame_mark_with_null();
        rustracy::emit_frame_mark("Main Loop\0");
        match execute_frame(&mut engine, &mut event_pump, &mut canvas) {
            ExecuteResult::Exit => break 'running,
            _ => (),
        }
        let n = Instant::now();
        if n < next_frame_time {
            thread::sleep(next_frame_time - n);
        }
        next_frame_time += duration_per_frame;
        count = count.wrapping_add(1);
    }
    Ok(())
}
