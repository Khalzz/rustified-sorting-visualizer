use std::time::Duration;

use sdl2::{keyboard::Keycode, render::{Canvas}, video::{Window}, pixels::Color};
use button_module::Button;

mod sorting;
mod utils;
mod button_module;

fn event_handler(event_pump: &mut sdl2::EventPump, is_running: &mut bool, numbers: &mut [i32; 100], canvas: &mut Canvas<Window>, pos_x: &mut i32, button_selection: &mut Button, button_bubble: &mut Button, button_restart: &mut Button) {
    for event in event_pump.poll_iter() {
        match event { 
            sdl2::event::Event::Quit { .. } | sdl2::event::Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                *is_running = false;
            },
            _ => {}
        }

        // make sure that this calls the function when the user clicks
        if button_selection.on_click(&event) {
            button_selection.active = false;
            button_bubble.active = false;
            sorting::selection_sort(pos_x, numbers, canvas);
        }
        if button_bubble.on_click(&event) {
            sorting::bubble_sort(pos_x, numbers, canvas);
            button_selection.active = false;
            button_bubble.active = false;
        }

        if button_restart.is_clicked(&event) {
            *numbers = utils::randomize();
            utils::render_list(pos_x, numbers, canvas, None, None);
            button_selection.active = true;
            button_bubble.active = true;
        }
    }
}

fn main() {
    // window  generation
    let sdl_context = sdl2::init().unwrap();
    let video_susbsystem = sdl_context.video().unwrap();
    let _window = video_susbsystem.window("Sorting visualizer", 702, 200).build().unwrap();

    // canvas and events generation
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut _canvas = _window.into_canvas().build().unwrap();
    
    // game loop state
    let mut is_running = true;

    // list values
    let mut numbers: [i32; 100] = utils::randomize();
    let mut pos_x = 0;

    // ttf init
    let ttf_context = sdl2::ttf::init().unwrap(); // we create a "context"
    let use_font = "./assets/fonts/Inter-Thin.ttf";
    let mut _font = ttf_context.load_font(use_font, 20).unwrap();
    let texture_creator = _canvas.texture_creator();

    // starting state
    utils::render_list(&mut pos_x, &mut numbers, &mut _canvas, None, None);

    // buttons
    let mut button_selection_sort = Button::new(
        10,
        10,
        140,
        30,
        String::from("Selection Sort"),
        Color::RGB(0, 0, 0),
        Color::RGB(0, 200, 0),
        Color::RGB(0, 0, 0),
    );

    let mut button_bubble_sort = Button::new(
        160,
        10,
        140,
        30,
        String::from("Bubble Sort"),
        Color::RGB(0, 0, 0),
        Color::RGB(0, 200, 0),
        Color::RGB(0, 0, 0),
    );

    let mut button_restart = Button::new(
        700 - 150,
        10,
        140,
        30,
        String::from("Reset"),
        Color::RGB(0, 0, 0),
        Color::RGB(0, 200, 0),
        Color::RGB(0, 0, 0),
    );

    // framerate
    const FRAMERATE: u32 = 30; // here we change the framereate
    const FRAME_TIME: u32 = 1000 / FRAMERATE;
    
    while is_running {
        std::thread::sleep(Duration::from_millis(FRAME_TIME as u64));

        button_selection_sort.render(&mut _canvas, &texture_creator, &_font);
        button_bubble_sort.render(&mut _canvas, &texture_creator, &_font);
        button_restart.render(&mut _canvas, &texture_creator, &_font);
        event_handler(&mut event_pump, &mut is_running, &mut numbers, &mut _canvas,&mut pos_x, &mut button_selection_sort, &mut button_bubble_sort, &mut button_restart);
    }
}

// For a button system we will

    // make a list of buttons each one of them will have x, y, height, width, color, text
    // the button list will be rendered on the 