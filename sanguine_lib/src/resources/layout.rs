use svg::{node::{self, element::{Path, path::Data}}, Node};

/// This module contains types that describe the fundamental properties of the work.
/// 
/// Currently everything is grid based. 

/// Format contains the works's properties and a container for the grid.
#[derive(Debug, Clone)]
pub struct Format {
    /// Work height in pixels
    pub height: i32,
    /// Work width in pixels
    pub width: i32,
    /// A margin, value is currently never applied anywhere.
    pub margin: i32,
    /// Number of rows the grid has.
    pub rows: usize,
    /// Number of columns grid has.
    pub columns: usize,
    /// Vector that contains the grid.
    pub field_container: Vec<Vec<Field>>,
}

/// Orientation type, not implemented for use.
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
}

/// Measurement properties of one field. 
/// Maybe width and height can go somewhere else, because its the same for every field?
#[derive(Debug, Copy, Clone)]
pub struct Field {
    /// x value of the given field
    pub x: i32,
    /// y value of the given field
    pub y: i32,
    /// Column width
    pub column_width: i32,
    /// Row height
    pub row_height: i32,
}

impl Format {
    /// Creates a new grid.
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

/// Adds a white background so png conversion is easier.
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
/// Helper function for the background path.
fn path(data: Data) -> Path {
    let path = Path::new()
        .set("fill", "white")
        .set("stroke", "none")
        .set("stroke-width", 1)
        .set("d", data);

    path
}
