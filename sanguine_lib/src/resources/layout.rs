use svg::{node::{self, element::{Path, path::Data}}, Node};

#[derive(Debug, Clone)]
pub struct Format {
    pub height: i32,
    pub width: i32,
    pub margin: i32,
    pub rows: usize,
    pub columns: usize,
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
    pub fn new(height: i32, width: i32, margin: i32, rows: usize, columns: usize) -> Self {
        let column_width = width / columns as i32;
        let row_height = height / rows as i32;

        let mut fields = Vec::new();

        for row in 0..rows {
            let mut inner = Vec::new();
            for col in 0..columns {
                let field = Field {
                    x: column_width * col as i32,
                    y: row_height * row as i32,
                    column_width: column_width,
                    row_height: row_height,
                };

                inner.push(field)
            }
            fields.push(inner)
        }

        let work = Format {
            height: height,
            width: width,
            margin: margin,
            rows: rows,
            columns: columns,
            field_container: fields,
        };

        work
    }
}

pub fn backgound(layout: &Format) -> node::element::Group {

        let mut graph = node::element::Group::new();
   

        let data = node::element::path::Data::new()
            .move_to((0,0))
            .line_to((0,layout.height))
            .line_to(
                (layout.width,
                layout.height,)
            )
            .line_to((layout.width, 0)
            )
            .close();
    
        let path = path(data);
        graph.append(path);
        graph


}

fn path(data: Data) -> Path {
    let path = Path::new()
        .set("fill", "white")
        .set("stroke", "none")
        .set("stroke-width", 1)
        .set("d", data);

    path
}
