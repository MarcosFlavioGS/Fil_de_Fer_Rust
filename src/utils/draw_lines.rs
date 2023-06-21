extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::PointStruct;

fn put_line(canvas: &mut Canvas<Window>, a: Point, b: Point) {
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.draw_line(a, b).unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
}

pub fn draw_lines(canvas: &mut Canvas<Window>, map: &Vec<Vec<PointStruct>>) {
    for line in map {
        for i in 0..line.len() - 1 {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.draw_line(line[i].point, line[i + 1].point).unwrap();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
        }
    }
    for i in 0..map.len() - 1 {
        for j in 0..map[i].len() {
            put_line(canvas, map[i][j].point, map[i + 1][j].point);
        }
    }
}
