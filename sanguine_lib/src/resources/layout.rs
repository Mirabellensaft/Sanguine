use svg::{
    node::{
        self,
        element::{path::Data, Path},
    },
    Node,
};

use super::{shapes::point::Point, composition::voronoi};

/// This module contains types that describe the fundamental properties of the work.
///
/// Currently everything is grid based.

/// Format contains the works's properties and a container for the grid.
pub trait Layout {
    fn new(parameters: Parameters) -> Self where Self: Sized;
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;


    
}

pub struct Parameters {
    pub height: i32, 
    pub width: i32, 
    pub margin: i32, 
    pub rows: usize, 
    pub columns: usize,
    pub layout_type: LayoutType
}
pub struct Work(pub Box <dyn Layout>);

pub enum LayoutType {
    GridBased,
    VoronoiBased,

}

impl Work {
    pub fn new(parameters: Parameters) -> Self {
        match parameters.layout_type {
            LayoutType::GridBased => {
                Work(Box::new(Grid::new(parameters)))
            },
            LayoutType::VoronoiBased => {
                Work(Box::new(Voronoi::new(parameters)))
            },
        }
    }

        /// Adds a white background so png conversion is easier.
    pub fn background(&self) -> node::element::Group {
        let mut graph = node::element::Group::new();

        let data = node::element::path::Data::new()
            .move_to((0, 0))
            .line_to((0, self.0.get_height()))
            .line_to((self.0.get_width(), self.0.get_height()))
            .line_to((self.0.get_width(), 0))
            .close();

        let path = path(data);
        graph.append(path);
        graph
    }

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
    
#[derive(Debug, Clone)]
pub struct Grid {
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

impl Layout for Grid {
    /// Creates a new grid.
    fn new(parameters: Parameters) -> Self {
        let column_width = parameters.width / parameters.columns as i32;
        let row_height = parameters.height / parameters.rows as i32;

        let mut fields = Vec::new();

        for row in 0..parameters.rows {
            let mut inner = Vec::new();
            for col in 0..parameters.columns {
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

        let work = Grid {
            height: parameters.height,
            width: parameters.width,
            margin: parameters.margin,
            rows: parameters.rows,
            columns: parameters.columns,
            field_container: fields,
        };

        work
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }
}



#[derive(Debug, Clone)]
pub struct Voronoi {
    /// Work height in pixels
    pub height: i32,
    /// Work width in pixels
    pub width: i32,
    /// A margin, value is currently never applied anywhere.
    pub margin: i32,
    /// Vector that contains points for generating a voronoi diagram, can be empty.
    pub field_container: Vec<Point>,
}

impl Layout for Voronoi {
    /// generates a new voronoi layout, with an empty point vector.
    fn new(parameters: Parameters) -> Self {
        let work = Voronoi {
            height: parameters.height,
            width: parameters.width,
            margin: parameters.margin,
            field_container: Vec::new(),
        };
        work
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }
}
