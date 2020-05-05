use super::{
    super::common::Vec2D,
    Render,
    UsedShader,
    font::Font,
};

#[derive(Debug)]
pub struct FontRender {
    font: Font,
}

impl FontRender {
    pub fn new() -> Self {
        let font = Font::default();
        FontRender { font }
    }

    pub fn print(&self, render: &mut Render, text: &str, pos: Vec2D<f32>) {
        for (n, code) in text.chars().map(|c| c as u32).enumerate() {
            match self.font.page(code) {
                None => continue,
                Some(texture) => render.set_texture(texture),
            }

            let char_rect = self.font.char_rect(n as u32, pos);
            let st_rect = self.font.st_rect(code);

            render.draw_rect_accept(UsedShader::Font, char_rect, Some(st_rect));
        }
    }

    pub fn text_size(&self, text: &str) -> Vec2D<u32> { self.font.text_size(text) }
}