use crate::PointStruct;
use sdl2::pixels::Color;

pub fn init_map(map: &mut Vec<PointStruct>, matrix: &Vec<Vec<u32>>) {
    let mut x;
    let mut y;

    x = 5;
    y = 5;
    for line in matrix {
        for num in line {
            let point = PointStruct {
                point: sdl2::rect::Point::new(x, y),
                color: Color::RGB(0, 255, 0),
                value: *num,
            };
            map.push(point);
            x += 10;
        }
        x = 5;
        y += 10;
    }
}
