use crate::PointStruct;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub fn init_map(map: &mut Vec<Vec<PointStruct>>, matrix: &Vec<Vec<u32>>) {
    let mut line_vec: Vec<PointStruct>;
    let mut x;
    let mut y;

    x = 5;
    y = 5;
    for line in matrix {
        line_vec = Vec::new();
        for num in line {
            let point = PointStruct {
                point: Point::new(y, x),
                color: Color::RGB(0, 255, 0),
                value: *num,
            };
            line_vec.push(point);
            y += 10;
        }
        map.push(line_vec);
        y = 5;
        x += 10;
    }
}
