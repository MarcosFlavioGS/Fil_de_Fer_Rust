extern crate sdl2;

mod put_pixel;
mod map_reader;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

use crate::put_pixel::put_pixel;
use crate::map_reader::map_reader;

pub struct PointStruct {
    point: Point,
    color: Color,
    value: u32,
}

pub enum Map {
    Map(Vec<Vec<PointStruct>>),
}

impl Map {
    fn init_map(self, matrix: &Vec<Vec<u32>>) -> Map {
        match self {
            Map::Map(mut map) => {
                let mut x = 5;
                let mut y = 5;
                for line in matrix.iter() {
                    let mut tmp: Vec<PointStruct> = Vec::new();
                    for value in line.iter() {
                        let point = PointStruct {
                            point: Point::new(x, y),
                            color: Color::RGB(255, 255, 255),
                            value: *value,
                        };
                        tmp.push(point);
                        x += 10;
                    }
                    x = 5;
                    y += 10;
                    map.push(tmp);
                }
                Map::Map(map)
            }
        }
    }
}

impl Map {
    fn isometric(self) ->Vec<Vec<PointStruct>> {
        match self {
            Map::Map(mut map) => {
                for line in map.iter_mut() {
                    for point in line.iter_mut() {
                        let x = point.point.x;
                        point.point.x = (x - point.point.y) * 2;
                        point.point.y = (x + point.point.y) / 2 - point.value as i32;
                        point.point.x += 600;
                        point.point.y += 300;
                    }
                }
                map
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Fil de Fer", 1400, 800)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let map = Map::Map(Vec::new());
    let isometric_map: Vec<Vec<PointStruct>>;

    if let Ok(matrix) = map_reader("maps/42.fdf") {
        isometric_map = map.init_map(&matrix).isometric();
    } else {
        panic!("File not found !");
    }
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        for line in &isometric_map {
            for point in line {
                put_pixel(&mut canvas, point.point, point.color);
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
