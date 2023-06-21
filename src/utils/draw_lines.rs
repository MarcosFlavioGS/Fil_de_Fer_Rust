use crate::PointStruct;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn put_line(canvas: &mut Canvas<Window>, a: Point, b: Point) {
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.draw_line(a, b).unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
}

pub fn draw_lines(
    canvas: &mut Canvas<Window>,
    map: &Vec<Vec<PointStruct>>,
    pos_x: i32,
    pos_y: i32,
) {
    // Bresenham's line algorithm
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if x < row.len() - 1 {
                let mut x0 = cell.point.x + pos_x;
                let mut y0 = cell.point.y + pos_y;
                let x1 = row[x + 1].point.x + pos_x;
                let y1 = row[x + 1].point.y + pos_y;
                let dx = (x1 - x0).abs();
                let sx = if x0 < x1 { 1 } else { -1 };
                let dy = -(y1 - y0).abs();
                let sy = if y0 < y1 { 1 } else { -1 };
                let mut err = dx + dy;
                loop {
                    if x0 > 0 || x0 < 1400 || y0 > 0 || y0 < 800 {
                        put_line(canvas, Point::new(x0, y0), Point::new(x0, y0));
                    }
                    if x0 == x1 && y0 == y1 {
                        break;
                    }
                    let e2 = 2 * err;
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
                let mut x0 = cell.point.x + pos_x;
                let mut y0 = cell.point.y + pos_y;
                let x1 = map[y + 1][x].point.x + pos_x;
                let y1 = map[y + 1][x].point.y + pos_y;
                let dx = (x1 - x0).abs();
                let sx = if x0 < x1 { 1 } else { -1 };
                let dy = -(y1 - y0).abs();
                let sy = if y0 < y1 { 1 } else { -1 };
                let mut err = dx + dy;
                loop {
                    if x0 > 0 || x0 < 1400 || y0 > 0 || y0 < 800 {
                        put_line(canvas, Point::new(x0, y0), Point::new(x0, y0));
                    }
                    if x0 == x1 && y0 == y1 {
                        break;
                    }
                    let e2 = 2 * err;
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
