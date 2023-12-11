use super::layout;
use rand::{thread_rng, Rng};

/// This module only prescribes compositional elements. How they are rendered depends highly on the
/// individual project. It's probably helpful to provide a code template for this.

/// The Density variants are used to describe centers of attention and allow to generate gradients.
/// Some variants can have a direction marker.
///
/// I omit an exact defintion of every single variant, as the interpretation will
/// vary from art work to art work.

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Density {
    Transition(Direction),
    ThreeWay(Direction),
    Corner(Direction),
    Edge(Direction),
    Empty,
    Low,
    Mid,
    High,
    Focus,
}

/// Direction variants describe from where to where something is going in a field.
/// There is currently no 'None' option, a workouround is to just not read out a
/// set direction when rendering the Density variant that has a direction.
///
/// I omit an exact defintion of every single variant, as the interpretation will
/// vary from art work to art work.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    LeftRight,
    UpDown,
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
    Up,
    Down,
    Left,
    Right,
}

/// A possibility to set a center in the entire composition.
/// This is not properly implemented yet.
pub enum CompositionCenter {
    TopLeft,
    TopMid,
    TopRight,
    MidLeft,
    MidMid,
    MidRight,
    BottomLeft,
    BottomMid,
    BottomRight,
    Bottom,
    Top,
    Left,
    Right,
    VerticalCenter,
    HorizontalCenter,
}

/// This type contains the density variants for the entire grid.
pub struct CompositionOverlay(pub Vec<Vec<Density>>);

impl CompositionOverlay {
    /// Creates a new grid where all fields have the Density::Empty variant
    /// This is to start with a homogenous field of "nothing", depending
    /// on how rendering of the variant is set.
    pub fn new_empty(format: &layout::Grid) -> Self {
        let mut vec = Vec::new();

        for _row in 0..format.rows {
            let mut inner = Vec::new();
            for _col in 0..format.columns {
                inner.push(Density::Empty);
            }
            vec.push(inner);
        }
        CompositionOverlay(vec)
    }

    /// Creates a new grid where all fields have the Density::Mid variant.
    /// This is to start with a homogenous field of "something", depending
    /// on how rendering of the variant is set.
    pub fn new_flat(format: &layout::Grid) -> Self {
        let mut vec = Vec::new();

        for _row in 0..format.rows {
            let mut inner = Vec::new();
            for _col in 0..format.columns {
                inner.push(Density::Mid);
            }
            vec.push(inner);
        }
        CompositionOverlay(vec)
    }

    /// This function makes sure, that different Focal points are connected
    /// through fields of Density::Mid.
    pub fn connect_centers(&mut self) {
        let mut last_center: (usize, usize) = (0, 0);

        // this identifies the first center
        'first: for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                match self.0[row][col] {
                    Density::Focus => {
                        last_center = (row, col);
                        break 'first;
                    }
                    _ => (),
                }
            }
        }

        // this finds the next center, connects it to the first, and repeats for the second center etc.
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                match self.0[row][col] {
                    Density::Focus => {
                        for i in last_center.0 + 1..=row {
                            for j in last_center.1 + 1..=col {
                                match self.0[i][j] {
                                    Density::Empty => {
                                        self.0[i][j] = Density::Mid;
                                        self.0[i][j + 1] = Density::Mid;
                                    }
                                    _ => (),
                                }
                            }
                        }
                        last_center = (row, col);
                    }
                    _ => (),
                }
            }
        }
    }

    /// Adds a specified number of random centers with Density::Focus and surrounds them Density::High.
    pub fn add_random_center(&mut self, amount: usize, format: &layout::Grid) {
        let mut rng = thread_rng();

        for _i in 0..amount {
            let horizontal = rng.gen_range(1..format.columns - 2);
            let vertical = rng.gen_range(1..format.rows - 2);

            for row in vertical - 1..=vertical + 1 {
                for col in horizontal - 1..=horizontal + 1 {
                    self.0[row][col] = Density::High;
                    println!("mid");
                }
            }

            self.0[vertical][horizontal] = Density::Focus;
        }
    }

    /// Adds a specified number of random locations with Density::Low
    pub fn add_random_low(&mut self, amount: usize, format: &layout::Grid) {
        let mut rng = thread_rng();

        for _i in 0..amount {
            let horizontal = rng.gen_range(0..format.columns);
            let vertical = rng.gen_range(0..format.rows);

            self.0[vertical][horizontal] = Density::Low;
        }
    }

    /// Implements a compositional center. Not properly implemented yet.
    pub fn add_center(&mut self, center: CompositionCenter, format: &layout::Grid) {
        let multiplicator_rows = format.rows / 3;
        let multiplicator_columns = format.columns / 3;
        println!("{}, {}", multiplicator_columns, multiplicator_rows);

        // let _center_row = format.rows / 2;
        // let _center_col = format.columns / 2;

        let mut rng = thread_rng();
        let mut truth = false;

        match center {
            CompositionCenter::TopLeft => {
                for i in 0..multiplicator_rows + 1 {
                    for j in 0..multiplicator_columns + 1 {
                        truth = rng.gen_bool(1.0 / 3.0);

                        if truth {
                            self.0[i][j] = Density::Mid;
                        }
                        self.0[i][j] = Density::Empty;
                    }
                }
            }
            CompositionCenter::TopMid => {
                for i in 0..multiplicator_rows + 1 {
                    for j in multiplicator_columns..multiplicator_columns * 2 {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::TopRight => {
                for i in 0..multiplicator_rows + 1 {
                    for j in multiplicator_columns * 2..format.columns {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::MidLeft => {
                for i in multiplicator_rows..multiplicator_rows * 2 {
                    for j in 0..multiplicator_columns + 1 {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::MidMid => {
                for i in multiplicator_rows..multiplicator_rows * 2 {
                    for j in multiplicator_columns..multiplicator_columns * 2 {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::MidRight => {
                for i in multiplicator_rows..multiplicator_rows * 2 {
                    for j in multiplicator_columns * 2..format.columns {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::BottomLeft => {
                for i in multiplicator_rows * 2..format.rows {
                    for j in 0..multiplicator_columns + 1 {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::BottomMid => {
                for i in multiplicator_rows * 2..format.rows {
                    for j in multiplicator_columns..multiplicator_columns * 2 {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::BottomRight => {
                for i in multiplicator_rows * 2..format.rows {
                    for j in multiplicator_columns * 2..format.columns {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::Bottom => {
                for i in multiplicator_rows * 2..format.rows {
                    for j in 0..format.columns {
                        truth = rng.gen_bool(1.0 / 3.0);
                        if truth {
                            self.0[i][j] = Density::Mid;
                        } else {
                            self.0[i][j] = Density::Empty;
                        }
                    }
                }
            }
            CompositionCenter::Top => {
                for i in 0..multiplicator_rows + 1 {
                    for j in 0..format.columns {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::Left => {
                for i in 0..format.rows {
                    for j in 0..multiplicator_columns {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::Right => {
                for i in 0..format.rows {
                    for j in multiplicator_columns * 2..format.columns {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::VerticalCenter => {
                for i in 0..format.rows {
                    for j in multiplicator_columns..multiplicator_columns * 2 {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
            CompositionCenter::HorizontalCenter => {
                for i in multiplicator_rows..multiplicator_rows * 2 {
                    for j in 0..format.columns {
                        self.0[i][j] = Density::Mid;
                    }
                }
            }
        }
    }

    /// Fills gaps in the composition with Density variants Edge, ThreeWay,
    /// Corner, Low and Transition in their respective direction.
    pub fn retro_composition(&mut self, layout: &layout::Grid) {
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                match self.0[row][col] {
                    Density::Empty => {
                        let contact = self.direction_of_contact(row, col, layout);
                        match contact {
                            [true, true, true, true] => self.0[row][col] = Density::Low,

                            [true, true, true, false] => {
                                self.0[row][col] = Density::ThreeWay(Direction::Right)
                            }
                            [true, false, true, true] => {
                                self.0[row][col] = Density::ThreeWay(Direction::Left)
                            }
                            [true, true, false, true] => {
                                self.0[row][col] = Density::ThreeWay(Direction::Down)
                            }
                            [false, true, true, true] => {
                                self.0[row][col] = Density::ThreeWay(Direction::Up)
                            }

                            [true, true, false, false] => {
                                self.0[row][col] = Density::Corner(Direction::LeftUp)
                            }
                            [false, true, true, false] => {
                                self.0[row][col] = Density::Corner(Direction::LeftDown)
                            }
                            [false, false, true, true] => {
                                self.0[row][col] = Density::Corner(Direction::RightDown)
                            }
                            [true, false, false, true] => {
                                self.0[row][col] = Density::Corner(Direction::RightUp)
                            }

                            [false, false, false, true] => {
                                self.0[row][col] = Density::Edge(Direction::Left)
                            }
                            [true, false, false, false] => {
                                self.0[row][col] = Density::Edge(Direction::Up)
                            }
                            [false, false, true, false] => {
                                self.0[row][col] = Density::Edge(Direction::Down)
                            }
                            [false, true, false, false] => {
                                self.0[row][col] = Density::Edge(Direction::Right)
                            }

                            [false, true, false, true] => {
                                self.0[row][col] = Density::Transition(Direction::LeftRight)
                            }
                            [true, false, true, false] => {
                                self.0[row][col] = Density::Transition(Direction::UpDown)
                            }

                            [false, false, false, false] => self.0[row][col] = Density::Empty,
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Helper function that determines the contact points of each empty field detected by fn retro_composition and
    /// returns an array of 4 bools that each represent a side of the field.
    ///
    fn direction_of_contact(&mut self, row: usize, col: usize, layout: &layout::Grid) -> [bool; 4] {
        // Direction of the Edge variant points towards the block
        // edge on the right or left side of a block
        let mut sides = [false; 4];

        if row != 0 {
            match self.0[row - 1][col] {
                Density::Transition(direction) => match direction {
                    Direction::UpDown => sides[0] = true,
                    _ => sides[0] = false,
                },
                Density::Corner(direction) => match direction {
                    Direction::LeftDown => sides[0] = true,
                    Direction::RightDown => sides[0] = true,
                    _ => sides[0] = false,
                },
                Density::Edge(_) => sides[0] = false,
                Density::ThreeWay(_) => sides[0] = false,
                Density::Empty => sides[0] = false,
                _ => sides[0] = true,
            }
        }

        if row < layout.rows - 1 {
            match self.0[row + 1][col] {
                Density::Transition(direction) => match direction {
                    Direction::UpDown => sides[2] = true,
                    _ => sides[2] = false,
                },
                Density::ThreeWay(_) => sides[2] = false,
                Density::Corner(direction) => match direction {
                    Direction::LeftUp => sides[2] = true,
                    Direction::RightUp => sides[2] = true,
                    _ => sides[2] = false,
                },
                Density::Edge(_) => sides[2] = false,
                Density::Empty => sides[2] = false,
                _ => sides[2] = true,
            }
        }

        if col != 0 {
            match self.0[row][col - 1] {
                Density::Transition(direction) => match direction {
                    Direction::LeftRight => sides[1] = true,
                    _ => sides[1] = false,
                },
                Density::ThreeWay(_) => sides[1] = false,
                Density::Corner(direction) => match direction {
                    Direction::RightDown => sides[1] = true,
                    Direction::RightUp => sides[1] = true,
                    _ => sides[1] = false,
                },
                Density::Edge(_) => sides[1] = false,
                Density::Empty => sides[1] = false,
                _ => sides[1] = true,
            }
        }

        if col < layout.columns - 1 {
            match self.0[row][col + 1] {
                Density::Transition(direction) => match direction {
                    Direction::LeftRight => sides[3] = true,
                    _ => sides[3] = false,
                },
                Density::ThreeWay(_) => sides[3] = false,
                Density::Corner(direction) => match direction {
                    Direction::LeftDown => sides[3] = true,
                    Direction::LeftUp => sides[3] = true,
                    _ => sides[3] = false,
                },
                Density::Edge(_) => sides[3] = false,
                Density::Empty => sides[3] = false,
                _ => sides[3] = true,
            }
        }
        sides
    }
}

// #[cfg(test)]
// #[test]

// fn corner_test() {
//     let work = layout::Grid::new(100, 100, 1, 5, 5);
//     let mut comp = CompositionOverlay::new_empty(&work);
//     comp.0[0][4] = Density::Mid;
//     comp.0[0][3] = Density::Mid;
//     comp.0[1][4] = Density::Mid;
//     comp.0[2][4] = Density::Mid;
//     comp.0[2][3] = Density::Mid;
//     println!("{:?}", comp.0);
//     comp.retro_composition(&work);
//     println!("{:?}", comp.0);
//     // assert_eq!(comp.0[1][1], Density::Corner(()));
// }
