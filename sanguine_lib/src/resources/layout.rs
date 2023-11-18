#[derive(Debug, Clone)]
pub struct Format {
    pub height: i32,
    pub width: i32,
    pub margin: i32,
    pub rows:  i32,
    pub columns: i32,
    pub field_container: Vec<Vec<Field>>,
}


#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Field {
    pub x: i32, 
    pub y: i32,
    pub column_width: i32,
    pub row_height: i32,
}

impl Format {
    pub fn new(height: i32, width: i32, margin: i32, rows:  i32, columns: i32) -> Self {

        let column_width = width / columns;
        let row_height = height / rows;

        let mut fields = Vec::new();

        for row in 0..rows {
            let mut inner = Vec::new();
            for col in 0..columns {
                let field = Field {
                    x: column_width * col,
                    y: row_height * row,
                    column_width: column_width,
                    row_height: row_height,
                };

                inner.push(field)
            }
            fields.push(inner)
        }

        let work  = Format {
            height: height,
            width: width,
            margin: margin,
            rows:  rows,
            columns: columns,
            field_container: fields,

        };

        work
    }
}


