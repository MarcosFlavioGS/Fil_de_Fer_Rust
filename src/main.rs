extern crate sdl2;

mod put_pixel;
mod map_reader;
mod init_map;
mod isometric;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

use crate::put_pixel::put_pixel;
use crate::map_reader::map_reader;
use crate::init_map::init_map;
use crate::isometric::isometric;

pub struct PointStruct {
    point: Point,
    color: Color,
    value: u32,
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
    let mut map: Vec<Vec<PointStruct>> = Vec::new();

    if let Ok(matrix) = map_reader("maps/42.fdf") {
        init_map(&mut map, &matrix);
    } else {
        panic!("File not found !");
    }
    isometric(&mut map);
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
        for line in &map {
            for point in line {
                put_pixel(&mut canvas, point.point, point.color);
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
