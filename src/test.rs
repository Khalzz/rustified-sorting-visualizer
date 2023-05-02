use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

fn event_thread(sender: Sender<Event>) {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    sender.send(event).unwrap();
                    return;
                }
                _ => sender.send(event).unwrap(),
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("My SDL2 window", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let (sender, receiver): (Sender<Event>, Receiver<Event>) = channel();

    let event_thread_handle = thread::spawn(move || {
        event_thread(sender);
    });

    let mut i = 0;
    'running: loop {
        // Handle input events
        for event in receiver.try_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {
                    // Handle other events here, if necessary
                }
            }
        }

        // Update the screen
        i += 1;
        canvas.set_draw_color(Color::RGB(i % 255, 100, 100));
        canvas.clear();
        canvas.fill_rect(Rect::new(10, 10, 50, 50)).unwrap();
        canvas.present();

        // Sleep for a short amount of time to avoid using up all the CPU
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    event_thread_handle.join().unwrap();
}