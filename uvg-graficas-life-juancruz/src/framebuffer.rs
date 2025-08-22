use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    #[inline]
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
    #[inline]
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        rl: &RaylibThread,
    ) {
        if let Ok(texture) = window.load_texture_from_image(rl, &self.color_buffer) {
            let win_w = window.get_screen_width() as f32;
            let win_h = window.get_screen_height() as f32;
            let src = Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32);
            let dst = Rectangle::new(0.0, 0.0, win_w, win_h);
            let mut d = window.begin_drawing(rl);
            d.clear_background(Color::BLACK);
            d.draw_texture_pro(&texture, src, dst, Vector2::zero(), 0.0, Color::WHITE);
        }
    }

    pub fn save(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
}