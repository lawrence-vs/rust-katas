use kata_traits_14::{Shape, parse_shape};

#[test]
fn mixed_shapes_in_one_vec() {
    let shapes: Vec<Box<dyn Shape>> = ["circle 1", "square 2"]
        .iter()
        .map(|s| parse_shape(s))
        .collect();
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    assert!((total - (std::f64::consts::PI + 4.0)).abs() < 1e-9);
}
