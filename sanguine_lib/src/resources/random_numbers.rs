use crate::resources::{layout, shapes};
use rand::{thread_rng, Rng};

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
    // println!("h {:?}, v {:?}", horizontal, vertical);
    shapes::Point::new(
        rng.gen_range(horizontal) as f32,
        rng.gen_range(vertical) as f32,
    )
}

pub fn random_value_on_side(start_value: i32, distance: i32, margin: i32) -> f32 {
    let mut rng = thread_rng();

    let range: std::ops::Range<i32> = std::ops::Range {
        start: start_value + margin,
        end: start_value + distance - margin,
    };

    rng.gen_range(range) as f32  
}

pub fn array_of_values(start_value: i32, distance: i32, margin: i32) -> [f32; 10] {
    let mut array = [0.0;10];
    for i in 0..10 {
        array[i] = random_value_on_side(start_value,distance,margin)
    }
    array = repl_duplicates(array, start_value, distance, margin);
    array
}
pub fn coordinates_on_border(field: &layout::Field) -> [[shapes::Point; 10]; 4] {
    let mut coordinates: [[shapes::Point; 10]; 4] = [[shapes::Point::new(0.0, 0.0); 10]; 4];

    for i in 0..10 {
        let x = random_value_on_side(field.x, field.column_width,1);
        coordinates[0][i] = shapes::Point::new(x, field.y as f32);
        // println!("coordinates:{:?}", coordinates[side][i]);
    }

    for i in 0..10 {
        let y = random_value_on_side(field.y, field.row_height,1);
        coordinates[1][i] = shapes::Point::new(field.x as f32, y);
        // println!("coordinates:{:?}", coordinates[side][i]);
    }

    for i in 0..10 {
        let x = random_value_on_side(field.x, field.column_width,1);
        coordinates[2][i] = shapes::Point::new(x, field.y as f32 + field.row_height as f32);
        // println!("coordinates:{:?}", coordinates[side][i]);
    }

    for i in 0..10 {
        let y = random_value_on_side(field.y, field.row_height,1);
        coordinates[3][i] = shapes::Point::new(field.x as f32 + field.column_width as f32, y);
        // println!("coordinates:{:?}", coordinates[side][i]);
    }
    coordinates
}
pub fn repl_duplicates(mut array: [f32;10], start_value: i32, distance: i32, margin: i32) -> [f32;10] {

        for i in 1..=10 {
            if array[i..].contains(&array[i - 1]) {
                array[i-1] = random_value_on_side(start_value, distance, margin);
                array = repl_duplicates(array, start_value, distance, margin);
            }
        }
        
        array
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

// assert_eq!(has_dup(&[1, 2, 3, 2, 5, 6]), true);
//     assert_eq!(has_dup(&[1, 2, 3, 4, 5, 6]), false);

    // #[cfg(test)]
    // use crate::resources;
    // #[test]
    
    // fn duplicates() {
    //     println!("{:?}", check_duplicates_repl([1, 2, 3, 2, 5, 6, 7, 8, 9, 10]));
    // }