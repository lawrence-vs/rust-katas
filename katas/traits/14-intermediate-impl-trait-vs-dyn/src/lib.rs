pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub r: f64,
}

pub struct Square {
    pub side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

/// Parses `"circle 1.5"` or `"square 2"`.
/// CLUE: `impl Shape` in return position means one concrete type, chosen by this function. How many types does it return?
/// CLUE: What does the caller in the test want to store them in?
pub fn parse_shape(s: &str) -> impl Shape {
    let (kind, size) = s.split_once(' ').unwrap_or((s, "0"));
    let size: f64 = size.parse().unwrap_or(0.0);
    if kind == "circle" {
        Circle { r: size }
    } else {
        Square { side: size }
    }
}
