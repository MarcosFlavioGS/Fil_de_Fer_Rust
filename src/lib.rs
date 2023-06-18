pub mod utils;

use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct PointStruct {
    pub point: Point,
    pub color: Color,
    pub value: u32,
}

pub enum Map {
    Map(Vec<Vec<PointStruct>>),
}

impl Map {
    pub fn init_map(self, matrix: &Vec<Vec<u32>>) -> Map {
        match self {
            Map::Map(mut map) => {
                let mut tmp: Vec<PointStruct>;
                let mut x = 5;
                let mut y = 5;
                let     dist: i32 = 10;

                for line in matrix.iter() {
                    tmp = Vec::new();
                    for value in line.iter() {
                        let point = PointStruct {
                            point: Point::new(x, y),
                            color: Color::RGB(255, 255, 255),
                            value: (*value * dist as u32) / 3,
                        };
                        tmp.push(point);
                        x += dist;
                    }
                    x = 5;
                    y += dist;
                    map.push(tmp);
                }
                Map::Map(map)
            }
        }
    }

    pub fn isometric(self) ->Vec<Vec<PointStruct>> {
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
