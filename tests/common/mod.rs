//! Fixture loading shared by the test suite and the benchmark.

use simplify::Point2D;

/// Reads the 1000-point sample polyline.
///
/// Coordinates are converted with [`str::parse`], which is correctly rounded,
/// so every one lands on exactly the `f64` the fixture spells out. That
/// matters more than it looks: squared distances are compared against the
/// tolerance with a strict `>`, so a coordinate that is off by a single ulp
/// can move a point across the threshold and change the output. Some JSON
/// parsers round these literals to the neighbouring float, which is why this
/// reads the numbers itself rather than pulling in a dependency.
pub fn load_1k() -> Vec<Point2D> {
    let mut raw = include_str!("../fixtures/1k.json");
    let mut points = Vec::new();

    while let Some((x, rest)) = take_number(raw, "\"x\":") {
        let (y, rest) = take_number(rest, "\"y\":").expect("every point should have a y");
        points.push(Point2D::new(x, y));
        raw = rest;
    }

    points
}

/// Reads the number following the next occurrence of `key`, with the input
/// left after it.
fn take_number<'a>(raw: &'a str, key: &str) -> Option<(f64, &'a str)> {
    let rest = &raw[raw.find(key)? + key.len()..];
    let end = rest
        .find(|c: char| !matches!(c, '0'..='9' | '-' | '+' | '.' | 'e' | 'E'))
        .unwrap_or(rest.len());
    let (number, tail) = rest.split_at(end);

    Some((number.parse().ok()?, tail))
}
