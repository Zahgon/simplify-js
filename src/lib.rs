/*
 (c) 2017, Vladimir Agafonkin
 Simplify.rs, a high-performance polyline simplification library
 mourner.github.io/simplify-js
*/

//! A high-performance 2D polyline simplification library.
//!
//! Simplification is done with a combination of two algorithms: a fast radial
//! distance pre-filter that drops points too close to their predecessor,
//! followed by the Ramer-Douglas-Peucker algorithm.
//!
//! ```
//! use simplify::{simplify, Point2D};
//!
//! let points = [
//!     Point2D::new(0.0, 0.0),
//!     Point2D::new(1.0, 0.1),
//!     Point2D::new(2.0, 0.0),
//! ];
//!
//! let result = simplify(&points, Some(1.0), false);
//! assert_eq!(result, [Point2D::new(0.0, 0.0), Point2D::new(2.0, 0.0)]);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]
// The geometry helpers below could be written with `f64::mul_add`, and Clippy
// suggests exactly that. Fused multiply-add rounds once where a separate
// multiply and add round twice, so it produces results that differ by an ulp
// for roughly a sixth of all inputs. Squared distances are compared against
// the tolerance with a strict `>`, so an ulp is enough to change which points
// survive. The plain arithmetic is deliberate.
#![allow(clippy::suboptimal_flops)]

/// Anything that can be treated as a point on the plane.
///
/// Implement this for your own point type to simplify polylines without
/// converting them first; [`simplify`] hands back your own values, so any
/// extra data they carry is preserved.
///
/// ```
/// use simplify::{simplify, Point};
///
/// #[derive(Clone, Debug, PartialEq)]
/// struct Waypoint { lon: f64, lat: f64, name: &'static str }
///
/// impl Point for Waypoint {
///     fn x(&self) -> f64 { self.lon }
///     fn y(&self) -> f64 { self.lat }
/// }
///
/// let route = [
///     Waypoint { lon: 0.0, lat: 0.0, name: "start" },
///     Waypoint { lon: 1.0, lat: 0.1, name: "middle" },
///     Waypoint { lon: 2.0, lat: 0.0, name: "end" },
/// ];
///
/// let result = simplify(&route, Some(1.0), false);
/// assert_eq!(result.len(), 2);
/// assert_eq!(result[1].name, "end");
/// ```
pub trait Point {
    /// The point's horizontal coordinate.
    fn x(&self) -> f64;
    /// The point's vertical coordinate.
    fn y(&self) -> f64;
}

impl<T: Point + ?Sized> Point for &T {
    fn x(&self) -> f64 {
        (**self).x()
    }
    fn y(&self) -> f64 {
        (**self).y()
    }
}

impl Point for (f64, f64) {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
}

impl Point for [f64; 2] {
    fn x(&self) -> f64 {
        self[0]
    }
    fn y(&self) -> f64 {
        self[1]
    }
}

/// A point on the plane.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point2D {
    /// The point's horizontal coordinate.
    pub x: f64,
    /// The point's vertical coordinate.
    pub y: f64,
}

impl Point2D {
    /// Creates a point from its coordinates.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Point for Point2D {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
}

impl From<(f64, f64)> for Point2D {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}

impl From<[f64; 2]> for Point2D {
    fn from([x, y]: [f64; 2]) -> Self {
        Self::new(x, y)
    }
}

// square distance between 2 points
fn get_sq_dist<P: Point>(p1: &P, p2: &P) -> f64 {
    let dx = p1.x() - p2.x();
    let dy = p1.y() - p2.y();

    dx * dx + dy * dy
}

// square distance from a point to a segment
fn get_sq_seg_dist<P: Point>(p: &P, p1: &P, p2: &P) -> f64 {
    let mut x = p1.x();
    let mut y = p1.y();
    let mut dx = p2.x() - x;
    let mut dy = p2.y() - y;

    // A zero-length segment degenerates to its start point. The exact
    // comparison against zero is deliberate: it guards the division below,
    // which is the only value that would misbehave.
    if dx != 0.0 || dy != 0.0 {
        let t = ((p.x() - x) * dx + (p.y() - y) * dy) / (dx * dx + dy * dy);

        if t > 1.0 {
            x = p2.x();
            y = p2.y();
        } else if t > 0.0 {
            x += dx * t;
            y += dy * t;
        }
    }

    dx = p.x() - x;
    dy = p.y() - y;

    dx * dx + dy * dy
}

// basic distance-based simplification
fn simplify_radial_dist<P: Point + Clone>(points: &[P], sq_tolerance: f64) -> Vec<P> {
    // The reference implementation tracks the previous kept point by identity
    // and, at the end, appends the final point unless it was the one just
    // kept. Tracking its index reproduces that exactly: two points that are
    // numerically equal but at different positions still count as distinct.
    let mut prev_index = 0;
    let mut new_points = vec![points[0].clone()];

    for i in 1..points.len() {
        if get_sq_dist(&points[i], &points[prev_index]) > sq_tolerance {
            new_points.push(points[i].clone());
            prev_index = i;
        }
    }

    let last = points.len() - 1;
    if prev_index != last {
        new_points.push(points[last].clone());
    }

    new_points
}

fn simplify_dp_step<P: Point + Clone>(
    points: &[P],
    first: usize,
    last: usize,
    sq_tolerance: f64,
    simplified: &mut Vec<P>,
) {
    let mut max_sq_dist = sq_tolerance;
    let mut index = None;

    for i in (first + 1)..last {
        let sq_dist = get_sq_seg_dist(&points[i], &points[first], &points[last]);

        if sq_dist > max_sq_dist {
            index = Some(i);
            max_sq_dist = sq_dist;
        }
    }

    // `max_sq_dist` only ever moves above `sq_tolerance` together with
    // `index`, so having a farthest point is the same condition the reference
    // implementation spells as `maxSqDist > sqTolerance`.
    if let Some(index) = index {
        if index - first > 1 {
            simplify_dp_step(points, first, index, sq_tolerance, simplified);
        }
        simplified.push(points[index].clone());
        if last - index > 1 {
            simplify_dp_step(points, index, last, sq_tolerance, simplified);
        }
    }
}

// simplification using Ramer-Douglas-Peucker algorithm
fn simplify_douglas_peucker<P: Point + Clone>(points: &[P], sq_tolerance: f64) -> Vec<P> {
    let last = points.len() - 1;

    let mut simplified = vec![points[0].clone()];
    simplify_dp_step(points, 0, last, sq_tolerance, &mut simplified);
    simplified.push(points[last].clone());

    simplified
}

/// Simplifies a polyline, both algorithms combined for awesome performance.
///
/// `tolerance` is the maximum distance a point may sit from the simplified
/// line and still be discarded; [`None`] means a tolerance of `1`. Passing
/// `highest_quality` skips the radial-distance pre-filter, which is slower but
/// keeps a little more detail.
///
/// Polylines of two points or fewer are returned unchanged.
#[must_use]
pub fn simplify<P: Point + Clone>(
    points: &[P],
    tolerance: Option<f64>,
    highest_quality: bool,
) -> Vec<P> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let sq_tolerance = tolerance.map_or(1.0, |tolerance| tolerance * tolerance);

    if highest_quality {
        simplify_douglas_peucker(points, sq_tolerance)
    } else {
        simplify_douglas_peucker(&simplify_radial_dist(points, sq_tolerance), sq_tolerance)
    }
}
