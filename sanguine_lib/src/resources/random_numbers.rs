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

    shapes::Point::new(
        rng.gen_range(horizontal) as f32,
        rng.gen_range(vertical) as f32,
    )
}

pub fn coordinates_on_border(field: &layout::Field) -> [[shapes::Point; 10]; 4] {
    let mut coordinates: [[shapes::Point; 10]; 4] = [[shapes::Point::new(0.0, 0.0); 10]; 4];

    for i in 0..10 {
        let x = coordinate(field, 1).x;
        coordinates[0][i] = shapes::Point::new(x, field.y as f32);
        // println!("coordinates:{:?}", coordinates[side][i]);
    }

    for i in 0..10 {
        let y = coordinate(field, 1).y;
        coordinates[1][i] = shapes::Point::new(field.x as f32, y);
        // println!("coordinates:{:?}", coordinates[side][i]);
    }

    for i in 0..10 {
        let x = coordinate(field, 1).x;
        coordinates[2][i] = shapes::Point::new(x, field.y as f32 + field.row_height as f32);
        // println!("coordinates:{:?}", coordinates[side][i]);
    }

    for i in 0..10 {
        let y = coordinate(field, 1).y;
        coordinates[3][i] = shapes::Point::new(field.x as f32 + field.column_width as f32, y);
        // println!("coordinates:{:?}", coordinates[side][i]);
    }

    // println!(
    //     "COORDS {:?}",
    //     coordinates
    // );

    coordinates
}

pub fn all_border_coordinates(format: &layout::Format) -> Vec<Vec<[[shapes::Point; 10]; 4]>> {
    let mut vec = Vec::new();

    for row in 0..format.rows {
        let mut inner = Vec::new();
        for col in 0..format.columns {
            inner.push(coordinates_on_border(
                &format.field_container[row as usize][col as usize],
            ));
        }
        vec.push(inner);
    }

    vec
}

// pub fn point_on_circle(cx: i32, cy: i32, radius: i32) -> (i32, i32) {

// }
