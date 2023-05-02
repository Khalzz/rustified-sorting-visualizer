use sdl2::{render::{Canvas, TextureCreator, TextureQuery}, ttf::Font, video::{Window, WindowContext}, mouse::MouseButton};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Button {
    pub active: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub text: String,
    pub color: Color,
    pub base_color: Color,
    pub hover_color: Color,
    pub clicked_color: Color,
    pub hover: bool,
    pub clicked: bool,
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: String, color: Color, hover_color: Color, clicked_color: Color) -> Self {
        Button {
            active: true,
            x,
            y,
            width,
            height,
            text,
            color,
            base_color: color,
            hover_color,
            clicked_color,
            hover: false,
            clicked: false,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, font: &Font) {
        if self.active == true {
            canvas.set_draw_color(self.color); // it must be a Color::RGB() or other
            canvas.fill_rect(Rect::new(self.x, self.y, self.width, self.height)).unwrap();

            // Render the button text
            let surface = font.render(&self.text).solid(Color::WHITE).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

            // We center the text on the button
            let TextureQuery { width: text_width, height: text_height, .. } = texture.query();
            let text_x = self.x + (self.width as i32 - text_width as i32) / 2;
            let text_y = self.y + (self.height as i32 - text_height as i32) / 2;

            // render
            canvas.copy(&texture, None, Rect::new(text_x, text_y, text_width, text_height)).unwrap();
            canvas.present();
        }
    }

    pub fn is_hover(&mut self, event: &sdl2::event::Event) {
        if self.active {
            match event { 
                sdl2::event::Event::MouseMotion {x, y, .. } => {
                    if (x > &self.x && x < &(self.x + (self.width as i32))) && (y >= &self.y && y <= &(self.y + (self.height as i32))) {
                        self.color = self.hover_color;
                        self.hover = true;
                    } else {
                        self.color = self.base_color;
                        self.hover = false;
                    }
                },
                _ => {} // in every other case we will do nothing
            } 
        } else {
            self.hover = false;
        }
    }

    // this function will only return true or false based on if its pressed or not
    pub fn is_clicked(&mut self, event: &sdl2::event::Event) -> bool {
        self.is_hover(event);
        self.clicked = false;
        if self.active {
            match event { 
                sdl2::event::Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    if self.hover {
                        // (self.onclick)(pos_x, numbers, canvas);
                        self.clicked = true;
                    }
                },
                _ => {} // in every other case we will do nothing
            }   
            return self.clicked;   
        } else {
            return false;
        }
    }

    // this function will return nothing but it will run a function inside of itself so i can deactivate it while it runs
    pub fn on_click(&mut self, event: &sdl2::event::Event) -> bool{
        if self.active {
            self.is_hover(event);
            
            if self.is_clicked(event) {
                self.hover = false;
                self.color = self.base_color;
                return true;
            }

            if self.hover {
                self.color = self.hover_color;
            } else {
                self.color = self.base_color;
            }
        
            
        }
        return false
    }
}