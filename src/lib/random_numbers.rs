use crate::lib::layout;
use rand::{thread_rng, Rng};

pub fn coordinate(field: &layout::Field, margin: i32) -> (i32, i32) {
    let mut rng = thread_rng();

    let horizontal: std::ops::Range<i32> = std::ops::Range {
        start: field.x + margin,
        end: field.x + field.column_width - margin,
    };

    let vertical: std::ops::Range<i32> = std::ops::Range {
        start: field.y + margin,
        end: field.y + field.row_height - margin,
    };
    
    let randoms = (rng.gen_range(horizontal), rng.gen_range(vertical));
    randoms
}

pub fn coordinates_on_border(field: &layout::Field) -> [(i32, i32); 40] {

    let mut coordinates: [(i32, i32); 40] = [(0,0);40];
    
    for i in 0..10 {
        let x = coordinate(field, 0).0;
        coordinates[i] = (x, field.y);
    }

    for i in 10..20 {
        let y = coordinate(field, 0).1;
        coordinates[i] = (field.x, y);
    }

    for i in 20..30 {
        let x = coordinate(field, 0).0;
        coordinates[i] = (x, field.y + field.row_height);
    }

    for i in 30..40 {
        let y = coordinate(field, 0).1;
        coordinates[i] = (field.x + field.column_width, y);
    }

    coordinates
}

// pub fn point_on_circle(cx: i32, cy: i32, radius: i32) -> (i32, i32) {
    
// }