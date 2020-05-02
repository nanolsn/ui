mod common;
mod graphics;
mod ui;

use common::Rect;
use graphics::Window;
use ui::image::Image;

fn main() {
    let window = Window::new("App", (600, 400));

    let rect = Rect::new((30, 30), (80, 40));
    let img = Image::new("data/1.png", window.render());
    let hello = String::from("this repository understand your project by adding a README.");

    window.run(move |render| {
        render.draw(&rect);
        render.draw(&img);
        render.draw(&hello);
    });
}
