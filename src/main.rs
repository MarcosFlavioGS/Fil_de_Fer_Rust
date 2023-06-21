extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
//use sdl2::rect::Point;
use std::time::Duration;

use fdf::{PointStruct, Map};
use fdf::utils::draw_lines::draw_lines;
use fdf::utils::map_reader::map_reader;

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
    let     matrix: Vec<Vec<i32>>;
    let     map = Map::Map(Vec::new());
    let mut isometric_map: Vec<Vec<PointStruct>>;
    let mut dist: i32 = 10;
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;

    if let Ok(result_matrix) = map_reader("maps/mars.fdf") {
        matrix = result_matrix;
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
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => dist += 1,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    if dist == 1 {
                        ()
                    } else {
                        dist -= 1;
                    }
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => pos_x -= 5,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => pos_x += 5,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => pos_y += 5,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => pos_y -= 5,
                _ => {}
            }
        }
        isometric_map = map.init_map(&matrix, dist).isometric();
        draw_lines(&mut canvas, &isometric_map, pos_x, pos_y);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
