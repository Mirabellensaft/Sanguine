use crate::lib::{layout, math};
use rand::{thread_rng, Rng};

pub fn coordinate(field: &layout::Field, margin: i32) -> math::Point {
    let mut rng = thread_rng();

    let horizontal: std::ops::Range<i32> = std::ops::Range {
        start: field.x + margin,
        end: field.x + field.column_width - margin,
    };

    let vertical: std::ops::Range<i32> = std::ops::Range {
        start: field.y + margin,
        end: field.y + field.row_height - margin,
    };
    
    math::Point::new(rng.gen_range(horizontal) as f32, rng.gen_range(vertical) as f32)
}

pub fn coordinates_on_border(field: &layout::Field) -> [math::Point; 40] {

    let mut coordinates: [math::Point; 40] = [math::Point::new(0.0, 0.0);40];
    
    for i in 0..10 {
        let x = coordinate(field, 0).x;
        coordinates[i] = math::Point::new(x, field.y as f32);
    }

    for i in 10..20 {
        let y = coordinate(field, 0).y;
        coordinates[i] = math::Point::new(field.x as f32, y);
    }

    for i in 20..30 {
        let x = coordinate(field, 0).x;
        coordinates[i] = math::Point::new(x, field.y as f32 + field.row_height as f32);
    }

    for i in 30..40 {
        let y = coordinate(field, 0).y;
        coordinates[i] = math::Point::new(field.x as f32 + field.column_width as f32, y);
    }

    coordinates
}

// pub fn point_on_circle(cx: i32, cy: i32, radius: i32) -> (i32, i32) {
    
// }