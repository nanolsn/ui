use crate::{
    common::Position,
    graphics::{
        Render,
        Draw,
        DrawParameters,
    },
};

#[derive(Debug)]
pub struct Pos<U> {
    position: Position,
    ui: U,
}

#[allow(dead_code)]
impl<U> Pos<U>
    where
        U: Draw,
{
    pub fn new(position: Position, ui: U) -> Self { Pos { position, ui } }

    pub fn center(ui: U) -> Self { Pos::new(Position::Center, ui) }
    pub fn left(pad: i32, ui: U) -> Self { Pos::new(Position::Left(pad), ui) }
    pub fn right(pad: i32, ui: U) -> Self { Pos::new(Position::Right(pad), ui) }
    pub fn bot(pad: i32, ui: U) -> Self { Pos::new(Position::Bot(pad), ui) }
    pub fn top(pad: i32, ui: U) -> Self { Pos::new(Position::Top(pad), ui) }

    pub fn left_bot(l: i32, b: i32, ui: U) -> Self { Pos::new(Position::LeftBot(l, b), ui) }
    pub fn left_top(l: i32, t: i32, ui: U) -> Self { Pos::new(Position::LeftTop(l, t), ui) }
    pub fn right_bot(r: i32, b: i32, ui: U) -> Self { Pos::new(Position::RightBot(r, b), ui) }
    pub fn right_top(r: i32, t: i32, ui: U) -> Self { Pos::new(Position::RightTop(r, t), ui) }
}

impl<U> Draw for Pos<U>
    where
        U: Draw,
{
    fn draw(&self, render: &mut Render, mut params: DrawParameters) {
        params.position = self.position;
        self.ui.draw(render, params);
    }
}

impl<U> std::ops::Deref for Pos<U> {
    type Target = U;

    fn deref(&self) -> &Self::Target { &self.ui }
}

impl<U> std::ops::DerefMut for Pos<U> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.ui }
}
