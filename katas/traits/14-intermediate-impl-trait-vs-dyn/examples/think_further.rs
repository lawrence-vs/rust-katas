//! 14 intermediate: `impl Trait` vs `dyn Trait`
//!
//! Think further, from the kata README:
//! 1. `parse_shape("triangle 3")` and `parse_shape("circle abc")` both return a shape. What should a parser return for input it doesn't understand, and how would that change the test?
//! 2. Someone adds `fn scaled<T: Into<f64>>(&self, by: T) -> Self` to `Shape`. Does `Box<dyn Shape>` still compile? Read the reference's rules for dyn compatibility, then decide what the method should look like.
//! 3. The shapes move into a `tokio::spawn` task (kata 17). What does `Box<dyn Shape>` not promise that the task needs, and where would you write that promise?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

trait Shape {
    fn area(&self) -> f64;
}

struct Circle(f64);
struct Square(f64);

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.0 * self.0
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

// Think further 1: parsing can fail, so return a Result instead of a shape for bad input.
fn parse_shape(s: &str) -> Result<Box<dyn Shape>, String> {
    let mut parts = s.split_whitespace();
    let kind = parts.next().ok_or("empty")?;
    let size: f64 = parts.next().ok_or("no size")?.parse().map_err(|_| "bad size".to_string())?;
    match kind {
        "circle" => Ok(Box::new(Circle(size))),
        "square" => Ok(Box::new(Square(size))),
        _ => Err(format!("unknown shape {kind}")),
    }
}

// impl Trait when the function picks one concrete type: no heap allocation, no dynamic dispatch.
fn unit_circle() -> impl Shape {
    Circle(1.0)
}

fn main() {
    println!("unit circle area: {:.2}", unit_circle().area());

    for input in ["circle 2", "square 3", "triangle 3", "circle abc"] {
        match parse_shape(input) {
            Ok(shape) => println!("{input}: {:.2}", shape.area()),
            Err(e) => println!("{input}: error {e}"),
        }
    }

    // Think further 3: a thread that borrows the shapes needs them Send and Sync; dyn Shape alone promises neither.
    let shapes: Vec<Box<dyn Shape + Send + Sync>> = vec![Box::new(Square(2.0))];
    let total: f64 = std::thread::scope(|s| {
        s.spawn(|| shapes.iter().map(|sh| sh.area()).sum()).join().unwrap_or(0.0)
    });
    println!("total area in another thread: {total:.2}");
}
