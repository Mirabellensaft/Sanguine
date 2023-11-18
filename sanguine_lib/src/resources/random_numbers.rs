use crate::resources::{layout, shapes};
use rand::{thread_rng, Rng};

use super::layout::Orientation;

pub fn coordinate(field: &layout::Field, margin: i32) -> shapes::Point {
    let mut rng = thread_rng();

    let horizontal: std::ops::Range<i32> = std::ops::Range {
        start: field.x + margin,
        end: field.x + field.column_width - margin,
    };

    let vertical: std::ops::Range<i32> = std::ops::Range {
        start: field.y + margin,
        end: field.y + field.row_height - margin,
    };
    
    shapes::Point::new(rng.gen_range(horizontal) as f32, rng.gen_range(vertical) as f32)
}

pub fn coordinates_on_border(field: &layout::Field) -> [shapes::Point; 40] {

    let mut coordinates: [shapes::Point; 40] = [shapes::Point::new(0.0, 0.0);40];
    
    for i in 0..10 {
        let x = coordinate(field, 1).x;
        coordinates[i] = shapes::Point::new(x, field.y as f32);
    }

    for i in 10..20 {
        let y = coordinate(field, 1).y;
        coordinates[i] = shapes::Point::new(field.x as f32, y);
    }

    for i in 20..30 {
        let x = coordinate(field, 1).x;
        coordinates[i] = shapes::Point::new(x, field.y as f32 + field.row_height as f32);
    }

    for i in 30..40 {
        let y = coordinate(field, 1).y;
        coordinates[i] = shapes::Point::new(field.x as f32 + field.column_width as f32, y);
    }

    coordinates
}

pub fn tesselating_coordinates_on_border(field: &layout::Field, orientation: Orientation) -> [shapes::Point; 10] {

    let mut coordinates: [shapes::Point; 10] = [shapes::Point::new(0.0, 0.0);10];

    match orientation {
        Orientation::Top => {
            for i in 0..10 {
                let x = coordinate(field, 1).x;
                coordinates[i] = shapes::Point::new(x, field.y as f32);
            }
        },
        Orientation::Bottom => {
            for i in 0..10 {
                let x = coordinate(field, 1).x;
                coordinates[i] = shapes::Point::new(x, field.y as f32 + field.row_height as f32);
            }
        },
        Orientation::Left => {
            for i in 0..10 {
                let y = coordinate(field, 1).y;
                coordinates[i] = shapes::Point::new(field.x as f32, y);
            }
        },
        Orientation::Right => {
            for i in 0..10 {
                let y = coordinate(field, 1).y;
                coordinates[i] = shapes::Point::new(field.x as f32 + field.column_width as f32, y);
            }
        },
    }

    coordinates
}

// pub fn point_on_circle(cx: i32, cy: i32, radius: i32) -> (i32, i32) {
    
// }