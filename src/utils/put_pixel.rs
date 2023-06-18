extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub fn put_pixel(canvas: &mut Canvas<Window>, point: Point, color: Color) {
    canvas.set_draw_color(color);
    canvas.draw_point(point).unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
}
