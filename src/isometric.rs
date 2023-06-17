use crate::PointStruct;

pub fn isometric(map: &mut Vec<Vec<PointStruct>>) {
    for line in map {
        for point in line {
            let x = point.point.x + 500;
            let y = point.point.y + 300;
            point.point.x = (x - y) * 2;
            point.point.y = (x + y) / 2 - point.value as i32;
        }
    }
}
