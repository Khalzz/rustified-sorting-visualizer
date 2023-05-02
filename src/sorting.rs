use sdl2::{pixels::Color, render::{Canvas}, video::Window};
use crate::{utils};

pub fn selection_sort(pos_x: &mut i32, numbers: &mut [i32; 100], canvas: &mut Canvas<Window>) {
    for i in 0..100 {
        for j in i..100 {
            if numbers[i] > numbers[j] {
                let temp_i = numbers[i];
                numbers[i] = numbers[j];
                numbers[j] = temp_i;
            }

            utils::render_list(pos_x, numbers, canvas, Some(i as i32), Some(j as i32));
        }
    }
}

pub fn bubble_sort(pos_x: &mut i32, numbers: &mut [i32; 100], canvas: &mut Canvas<Window>) {
    for i in 0..100 {
        for j in 0..99 {
            if numbers[j] > numbers[j+1] {
                let temp_j = numbers[j];
                numbers[j] = numbers[j+1];
                numbers[j+1] = temp_j;
                canvas.set_draw_color(Color::RGB(0, 100, 0)); // i change the background color
                canvas.clear();
            }
            utils::render_list(pos_x, numbers, canvas, Some(i as i32), Some(j as i32));
        }
    }
}