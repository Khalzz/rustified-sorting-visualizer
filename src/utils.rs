use sdl2::{pixels::Color, rect::Rect, render::{Canvas}, video::Window};
use rand::Rng;



pub fn render_list(pos_x: &mut i32,numbers: &mut [i32; 100], canvas: &mut Canvas<Window>, special: Option<i32>, special2: Option<i32>) {
    canvas.set_draw_color(Color::RGB(50, 50, 50)); // i change the background color
    canvas.clear();
    *pos_x = 2;
    
    for _num in 0..100 {
        if _num <= special.unwrap_or(-1) {
            canvas.set_draw_color(Color::RGB(0, 200, 0));
        } else if _num == special2.unwrap_or(-1) {
            canvas.set_draw_color(Color::RGB(200, 0, 0));
        } else {
            canvas.set_draw_color(Color::RGB(200, 200, 200));
        }
        canvas.fill_rect(Rect::new(*pos_x, (200 - numbers[_num as usize]).into(), 5, (2 * numbers[_num as usize]).try_into().unwrap())).unwrap();
        *pos_x += 7;
    }
    canvas.present();
}

pub fn randomize() -> [i32;100] {
    let mut array: [i32;100] = [0;100];
    for i in 0..100 {
        array[i] = rand::thread_rng().gen_range(1..100);
    }

    
    return array;
}
