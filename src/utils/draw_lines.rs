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

pub fn draw_lines(canvas: &mut Canvas<Window>, map: &Vec<Vec<PointStruct>>, pos_x: i32, pos_y: i32) {
   // Bresenham's line algorithm

    let mut x0: i32;
    let mut y0: i32;
    let mut x1: i32;
    let mut y1: i32;
    let mut dx: i32;
    let mut dy: i32;
    let mut sx: i32;
    let mut sy: i32;
    let mut err: i32;
    let mut e2: i32;
    
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if x < map[y].len() - 1 {
                x0 = map[y][x].point.x + pos_x;
                y0 = map[y][x].point.y + pos_y;
                x1 = map[y][x + 1].point.x + pos_x;
                y1 = map[y][x + 1].point.y + pos_y;
                dx = (x1 - x0).abs();
                sx = if x0 < x1 { 1 } else { -1 };
                dy = -(y1 - y0).abs();
                sy = if y0 < y1 { 1 } else { -1 };
                err = dx + dy;
                loop {
                    put_line(canvas, Point::new(x0, y0), Point::new(x0, y0));
                    if x0 == x1 && y0 == y1 {
                        break;
                    }
                    e2 = 2 * err;
                    if e2 >= dy {
                        err += dy;
                        x0 += sx;
                    }
                    if e2 <= dx {
                        err += dx;
                        y0 += sy;
                    }
                }
            }
            if y < map.len() - 1 {
                x0 = map[y][x].point.x + pos_x;
                y0 = map[y][x].point.y + pos_y;
                x1 = map[y + 1][x].point.x + pos_x;
                y1 = map[y + 1][x].point.y + pos_y;
                dx = (x1 - x0).abs();
                sx = if x0 < x1 { 1 } else { -1 };
                dy = -(y1 - y0).abs();
                sy = if y0 < y1 { 1 } else { -1 };
                err = dx + dy;
                loop {
                    put_line(canvas, Point::new(x0, y0), Point::new(x0, y0));
                    if x0 == x1 && y0 == y1 {
                        break;
                    }
                    e2 = 2 * err;
                    if e2 >= dy {
                        err += dy;
                        x0 += sx;
                    }
                    if e2 <= dx {
                        err += dx;
                        y0 += sy;
                    }
                }
            }
        }
    }
}
