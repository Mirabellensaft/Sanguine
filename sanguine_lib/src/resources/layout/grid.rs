#[allow(implied_bounds_entailment)]
use crate::resources::composition::Density;

use super::voronoi::Cell;
use super::{Parameters, Layout, LayoutType};

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
    pub container: Vec<Vec<Field>>,
}

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
    /// Density
    pub density: Density,
}

impl Layout for Grid {
    /// Creates a new grid, with empty grid fields.
    fn new(parameters: Parameters) -> Self {

        let mut rows = 0;
        let mut columns = 0;

        match parameters.layout_type {
            LayoutType::GridBased(r, c) => {
                rows = r;
                columns = c;
            },
            _ => (),
        }
        let column_width = parameters.width / columns as i32;
        let row_height = parameters.height / rows as i32;

        let mut fields = Vec::new();

        for row in 0..parameters.rows {
            let mut inner = Vec::new();
            for col in 0..parameters.columns {
                let field = Field {
                    x: column_width * col as i32,
                    y: row_height * row as i32,
                    column_width: column_width,
                    row_height: row_height,
                    density: Density::Empty,
                };
                inner.push(field)
            }
            fields.push(inner)
        }

        let work = Grid {
            height: parameters.height,
            width: parameters.width,
            margin: parameters.margin,
            rows: rows,
            columns: columns,
            container: fields,
        };

        work
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }

    fn get_rows(&self) -> usize {
        self.rows
    }

    fn get_columns(&self) -> usize {
        self.columns
    }

    fn get_points(&self) -> Vec<Cell> {
        unimplemented!()
    }

    fn get_fields(&self) -> Vec<Vec<Field>> {
        self.container.clone()
    }

    fn get_grid(&self) -> Grid {
        Self { 
            height: self.height, 
            width: self.width, 
            margin: self.margin,
            rows: self.rows, 
            columns: self.columns,
            container: self.container.clone(),
        }
    }

    
}

