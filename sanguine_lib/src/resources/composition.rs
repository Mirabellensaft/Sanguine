use super::layout;
use rand::{thread_rng, Rng};

pub enum Density {
    Empty,
    Low,
    Mid,
    High,
}

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
}

pub struct CompositionOverlay (
    pub Vec<Vec<Density>>
);

impl CompositionOverlay {
    pub fn new_flat(format: &layout::Format) -> Self {
        let mut vec = Vec::new();

        for row in 0..format.rows {
            let mut inner = Vec::new();
            for col in 0..format.columns {
                inner.push(Density::Low);
            }
            vec.push(inner);
        }
        CompositionOverlay(vec)
    }
    pub fn add_random_center(&mut self, amount: usize, format: &layout::Format) {
        let mut rng = thread_rng();

        for i in 0..amount {
            let horizontal = rng.gen_range(1..format.columns - 2);
            let vertical = rng.gen_range(1..format.rows - 2);

            for row in vertical - 1..=vertical + 1 {
                for col in horizontal - 1..=horizontal + 1  {
                    self.0[row][col] = Density::Mid;
                    println!("mid");

            }
        }

        self.0[vertical][horizontal] = Density::High;
        }
    }

    pub fn add_random_emty(&mut self, amount: usize, format: &layout::Format) {
        let mut rng = thread_rng();

        for i in 0..amount {
            let horizontal = rng.gen_range(0..format.columns);
            let vertical = rng.gen_range(0..format.rows);

        self.0[vertical][horizontal] = Density::Empty;

        }
    }


    pub fn add_center(&mut self, center: CompositionCenter, format: &layout::Format) {

        let multiplicator_rows = format.rows / 5;
        let multiplicator_columns = format.columns / 5;
        println!("{}, {}", multiplicator_columns, multiplicator_rows);

        let center_row = format.rows / 2;
        let center_col = format.columns / 2; 
        let max_row = format.rows - 1;
        let max_colum = format.columns - 1;
        
        match center {
            CompositionCenter::TopLeft => {
                for i in 0..multiplicator_rows + 1 {
                    for j in 0..multiplicator_columns + 1 {
                        self.0[i][j] = Density::High;
                        self.0[i][j+1] = Density::Mid;
                        self.0[i+1][j] = Density::Mid;
                        self.0[i+1][j+1] = Density::Mid;
                    }
                }    
            },
            CompositionCenter::TopMid => {
                self.0[0][center_col] = Density::High;
            
             },
            CompositionCenter::TopRight => {
                self.0[0][format.columns-1] = Density::High; },
            CompositionCenter::MidLeft => {
                self.0[center_row][0] = Density::High;
            },
            CompositionCenter::MidMid => {
                self.0[center_row][center_col] = Density::High;  
            },
            CompositionCenter::MidRight => {
                self.0[center_row][format.columns-1] = Density::High;},
            CompositionCenter::BottomLeft => {
                self.0[format.rows-1][0] = Density::High;
            },
            CompositionCenter::BottomMid => {
                self.0[format.rows-1][center_col] = Density::High;
            },
            CompositionCenter::BottomRight => {
                self.0[format.rows-1][format.columns-1] = Density::High;
            },

        }
    }
}


