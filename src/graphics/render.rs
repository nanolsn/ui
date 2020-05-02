use std::rc::Rc;

use super::{
    super::common::{
        Color,
        Rect,
        Vec2D,
    },
    shaders::ShaderSet,
    shader_data::*,
    rect_render::RectRender,
    font::Font,
    texture::Texture,
    uniform::SharedUniform,
    Draw,
};

#[derive(Debug)]
pub struct Render {
    shaders: ShaderSet,
    size: Vec2D<u32>,
    rect_render: RectRender,
    font_render: Rc<Font>,
    base_data: BaseData,
    font_data: FontData,
    projection: SharedUniform<glm::Mat4>,
    texture0: SharedUniform<i32>,
}

impl Render {
    pub fn new(context: &glutin::WindowedContext<glutin::PossiblyCurrent>) -> Self {
        gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

        Render::set_defaults();

        let mut shaders = ShaderSet::new();

        assert_eq!(shaders.len(), UsedShader::Base as usize);
        shaders.add(
            c_str!(include_str!("../shaders/vs.glsl")),
            c_str!(include_str!("../shaders/fs.glsl")),
        ).unwrap();

        assert_eq!(shaders.len(), UsedShader::Font as usize);
        shaders.add(
            c_str!(include_str!("../shaders/vs.glsl")),
            c_str!(include_str!("../shaders/font_fs.glsl")),
        ).unwrap();

        let base_data = BaseData::new(&mut shaders);
        let font_data = FontData::new(&mut shaders);

        let (w, h): (u32, u32) = context
            .window()
            .inner_size()
            .to_logical::<u32>(context.window().scale_factor())
            .into();

        let projection = Render::make_projection((w as f32, h as f32));

        let shader_data = shaders.get_shader_data(c_str!("projection"), vec![
            UsedShader::Base,
            UsedShader::Font,
        ]);

        let projection = SharedUniform::new(projection, shader_data).unwrap();


        let shader_data = shaders.get_shader_data(c_str!("texture0"), vec![
            UsedShader::Base,
            UsedShader::Font,
        ]);

        let texture0 = SharedUniform::new(0, shader_data).unwrap();

        Render {
            shaders,
            size: (w, h).into(),
            rect_render: RectRender::new(0, 1),
            font_render: Rc::new(Font::new()),
            base_data,
            font_data,
            projection,
            texture0,
        }
    }

    fn set_defaults() {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::Disable(gl::DEPTH_TEST);
            gl::Disable(gl::CULL_FACE);
        }
    }

    fn make_projection<S>(size: S) -> glm::Mat4
        where
            S: Into<Vec2D<f32>>,
    {
        const NEAR: f32 = 0.0;
        const FAR: f32 = 10.0;

        let size = size.into();
        glm::ortho(0.0, size.x, 0.0, size.y, NEAR, FAR)
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Vec2D<u32> { self.size }

    pub fn resize(&mut self, size: Vec2D<u32>) {
        unsafe { gl::Viewport(0, 0, size.x as i32, size.y as i32) }

        self.size = size;

        let projection = Render::make_projection(size.cast::<f32>());
        self.projection.set_value(projection);
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.r(), color.g(), color.b(), color.a());
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_rect(&mut self, shader: UsedShader, rect: Rect<f32>) {
        self.draw_rect_accept(shader, rect, None);
    }

    pub fn draw_rect_st(&mut self, shader: UsedShader, rect: Rect<f32>, st: Rect<f32>) {
        self.draw_rect_accept(shader, rect, Some(st));
    }

    fn draw_rect_accept(&mut self, shader: UsedShader, rect: Rect<f32>, st: Option<Rect<f32>>) {
        self.shaders.use_shader(shader as usize);

        self.projection.accept(&self.shaders);
        self.texture0.accept(&self.shaders);

        if shader == UsedShader::Base {
            self.base_data.draw_texture.accept(&self.shaders);
        }

        self.rect_render.draw(rect, st);
    }

    pub fn draw<D>(&mut self, draw: &D)
        where
            D: Draw,
    { draw.draw(self) }

    pub fn set_texture(&mut self, texture: &Texture) {
        const TEXTURE0_UNIT: i32 = 0;

        self.texture0.set_value(TEXTURE0_UNIT);
        texture.bind(self.texture0.get() as u32);

        self.base_data.draw_texture.set_value(true);
    }

    pub fn unset_texture(&mut self) { self.base_data.draw_texture.set_value(false) }

    pub fn print(&mut self, text: &str) {
        let font = Rc::clone(&self.font_render);

        let half = self.size.half().cast::<f32>();
        let text_half = font.text_size(text).half().cast::<f32>();

        font.print(self, text, half - text_half);
    }
}
