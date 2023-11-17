#[derive(Copy, Clone)]

pub struct Point{
    pub x: f32,
    pub y: f32,
}

impl Point {

    pub fn new(x: f32, y: f32) -> Self {
        let point = Point {
            x: x,
            y: y,
        };

        point
    }
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        let line = Line {
            start: start,
            end: end,
        };

        line
    }

    pub fn slope(&self) -> f32 {


        let d_x = self.start.x - self.end.x; 
        let d_y = self.start.y - self.end.y;

               
        let m = d_x / d_y;
        m
    }

    pub fn y_intercept(&self) -> f32 {

        let b = self.start.y as f32 - self.slope() * self.start.x as f32;
        b

    }

    pub fn return_point_on_line(&self, x: f32) -> Point {
        
        let y = self.slope() * self.start.x  as f32 + self.y_intercept();
        
        let point = Point {
            x: x,
            y: y,
        };
        point

    }
}
