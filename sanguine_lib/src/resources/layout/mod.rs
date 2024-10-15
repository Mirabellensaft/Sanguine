#[allow(implied_bounds_entailment)]
use svg::{
    node::{
        self,
        element::{path::Data, Path},
    },
    Node,
};

use super::{
    errors::Error,
    layout::{grid::Field, voronoi::Cell},
};

use super::shapes::point::Point;

pub mod grid;
pub mod voronoi;

/// This module contains types that describe the fundamental properties of the work.
///
/// Currently everything is grid based.

/// Format contains the works's properties and a container for the grid.

pub trait Layout {
    fn new(parameters: Parameters) -> Result<Self, Error>
    where
        Self: Sized;
    // would it be possible to have a default behavior that replaces the content of the new functions
    // of the single Types that implement the layout trait?
    fn background(&self) -> node::element::Group {
        let mut graph = node::element::Group::new();

        let data = node::element::path::Data::new()
            .move_to((0, 0))
            .line_to((0, self.get_height()))
            .line_to((self.get_width(), self.get_height()))
            .line_to((self.get_width(), 0))
            .close();

        let path = path(data);
        graph.append(path);
        graph
    }
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_margin(&self) -> i32;
    fn get_rows(&self) -> usize;
    fn get_columns(&self) -> usize;
    fn get_cells(&self) -> Vec<Cell>;
    fn get_fields(&self) -> Vec<Vec<Field>>;
}

pub struct Parameters {
    pub height: i32,
    pub width: i32,
    pub margin: i32,
    pub rows: usize,
    pub columns: usize,
    pub layout_type: LayoutType,
}

pub enum VoronoiType {
    Custom(Vec<Point>),
    Uniform(i32),
}

pub enum LayoutType {
    GridBased(usize, usize),
    VoronoiBased(VoronoiType),
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

/// Orientation type, not implemented for use.
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
}
