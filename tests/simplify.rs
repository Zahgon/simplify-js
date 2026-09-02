//! Behaviour tests for the polyline simplifier.
//!
//! The first three tests are carried over from the original `test/test.js`,
//! keeping their names, inputs and assertions. The rest pin down behaviour
//! the reference implementation got from JavaScript itself — default
//! arguments, point identity, float semantics — which is now this crate's own
//! responsibility.

mod common;

use simplify::{simplify, Point, Point2D};

#[rustfmt::skip]
const POINTS: [Point2D; 100] = [
    Point2D::new(224.55, 250.15), Point2D::new(226.91, 244.19),
    Point2D::new(233.31, 241.45), Point2D::new(234.98, 236.06),
    Point2D::new(244.21, 232.76), Point2D::new(262.59, 215.31),
    Point2D::new(267.76, 213.81), Point2D::new(273.57, 201.84),
    Point2D::new(273.12, 192.16), Point2D::new(277.62, 189.03),
    Point2D::new(280.36, 181.41), Point2D::new(286.51, 177.74),
    Point2D::new(292.41, 159.37), Point2D::new(296.91, 155.64),
    Point2D::new(314.95, 151.37), Point2D::new(319.75, 145.16),
    Point2D::new(330.33, 137.57), Point2D::new(341.48, 139.96),
    Point2D::new(369.98, 137.89), Point2D::new(387.39, 142.51),
    Point2D::new(391.28, 139.39), Point2D::new(409.52, 141.14),
    Point2D::new(414.82, 139.75), Point2D::new(427.72, 127.3),
    Point2D::new(439.6, 119.74), Point2D::new(474.93, 107.87),
    Point2D::new(486.51, 106.75), Point2D::new(489.2, 109.45),
    Point2D::new(493.79, 108.63), Point2D::new(504.74, 119.66),
    Point2D::new(512.96, 122.35), Point2D::new(518.63, 120.89),
    Point2D::new(524.09, 126.88), Point2D::new(529.57, 127.86),
    Point2D::new(534.21, 140.93), Point2D::new(539.27, 147.24),
    Point2D::new(567.69, 148.91), Point2D::new(575.25, 157.26),
    Point2D::new(580.62, 158.15), Point2D::new(601.53, 156.85),
    Point2D::new(617.74, 159.86), Point2D::new(622.0, 167.04),
    Point2D::new(629.55, 194.6), Point2D::new(638.9, 195.61),
    Point2D::new(641.26, 200.81), Point2D::new(651.77, 204.56),
    Point2D::new(671.55, 222.55), Point2D::new(683.68, 217.45),
    Point2D::new(695.25, 219.15), Point2D::new(700.64, 217.98),
    Point2D::new(703.12, 214.36), Point2D::new(712.26, 215.87),
    Point2D::new(721.49, 212.81), Point2D::new(727.81, 213.36),
    Point2D::new(729.98, 208.73), Point2D::new(735.32, 208.2),
    Point2D::new(739.94, 204.77), Point2D::new(769.98, 208.42),
    Point2D::new(779.6, 216.87), Point2D::new(784.2, 218.16),
    Point2D::new(800.24, 214.62), Point2D::new(810.53, 219.73),
    Point2D::new(817.19, 226.82), Point2D::new(820.77, 236.17),
    Point2D::new(827.23, 236.16), Point2D::new(829.89, 239.89),
    Point2D::new(851.0, 248.94), Point2D::new(859.88, 255.49),
    Point2D::new(865.21, 268.53), Point2D::new(857.95, 280.3),
    Point2D::new(865.48, 291.45), Point2D::new(866.81, 298.66),
    Point2D::new(864.68, 302.71), Point2D::new(867.79, 306.17),
    Point2D::new(859.87, 311.37), Point2D::new(860.08, 314.35),
    Point2D::new(858.29, 314.94), Point2D::new(858.1, 327.6),
    Point2D::new(854.54, 335.4), Point2D::new(860.92, 343.0),
    Point2D::new(856.43, 350.15), Point2D::new(851.42, 352.96),
    Point2D::new(849.84, 359.59), Point2D::new(854.56, 365.53),
    Point2D::new(849.74, 370.38), Point2D::new(844.09, 371.89),
    Point2D::new(844.75, 380.44), Point2D::new(841.52, 383.67),
    Point2D::new(839.57, 390.4), Point2D::new(845.59, 399.05),
    Point2D::new(848.4, 407.55), Point2D::new(843.71, 411.3),
    Point2D::new(844.09, 419.88), Point2D::new(839.51, 432.76),
    Point2D::new(841.33, 441.04), Point2D::new(847.62, 449.22),
    Point2D::new(847.16, 458.44), Point2D::new(851.38, 462.79),
    Point2D::new(853.97, 471.15), Point2D::new(866.36, 480.77),];

#[rustfmt::skip]
const SIMPLIFIED: [Point2D; 33] = [
    Point2D::new(224.55, 250.15), Point2D::new(267.76, 213.81),
    Point2D::new(296.91, 155.64), Point2D::new(330.33, 137.57),
    Point2D::new(409.52, 141.14), Point2D::new(439.6, 119.74),
    Point2D::new(486.51, 106.75), Point2D::new(529.57, 127.86),
    Point2D::new(539.27, 147.24), Point2D::new(617.74, 159.86),
    Point2D::new(629.55, 194.6), Point2D::new(671.55, 222.55),
    Point2D::new(727.81, 213.36), Point2D::new(739.94, 204.77),
    Point2D::new(769.98, 208.42), Point2D::new(779.6, 216.87),
    Point2D::new(800.24, 214.62), Point2D::new(820.77, 236.17),
    Point2D::new(859.88, 255.49), Point2D::new(865.21, 268.53),
    Point2D::new(857.95, 280.3), Point2D::new(867.79, 306.17),
    Point2D::new(859.87, 311.37), Point2D::new(854.54, 335.4),
    Point2D::new(860.92, 343.0), Point2D::new(849.84, 359.59),
    Point2D::new(854.56, 365.53), Point2D::new(844.09, 371.89),
    Point2D::new(839.57, 390.4), Point2D::new(848.4, 407.55),
    Point2D::new(839.51, 432.76), Point2D::new(853.97, 471.15),
    Point2D::new(866.36, 480.77),];

// --- carried over from test/test.js ---

#[test]
fn simplifies_points_correctly_with_the_given_tolerance() {
    let result = simplify(&POINTS, Some(5.0), false);
    assert_eq!(result, SIMPLIFIED);
}

#[test]
fn just_return_the_points_if_it_has_only_one_point() {
    let result = simplify(&[Point2D::new(1.0, 2.0)], None, false);
    assert_eq!(result, [Point2D::new(1.0, 2.0)]);
}

#[test]
fn just_return_the_points_if_it_has_no_points() {
    let points: [Point2D; 0] = [];
    let result = simplify(&points, None, false);
    assert_eq!(result, points);
}

// --- behaviour the reference implementation inherited from JavaScript ---

#[test]
fn just_return_the_points_if_it_has_two_points() {
    let points = [Point2D::new(1.0, 2.0), Point2D::new(3.0, 4.0)];

    assert_eq!(simplify(&points, Some(5.0), false), points);
    assert_eq!(simplify(&points, Some(5.0), true), points);
}

#[test]
fn uses_a_tolerance_of_one_when_none_is_given() {
    // `None` stands in for an omitted argument, which the reference
    // implementation squares to a tolerance of 1.
    assert_eq!(
        simplify(&POINTS, None, false),
        simplify(&POINTS, Some(1.0), false)
    );
    assert_eq!(
        simplify(&POINTS, None, true),
        simplify(&POINTS, Some(1.0), true)
    );
}

#[test]
fn highest_quality_skips_the_radial_distance_prefilter() {
    // At this tolerance the pre-filter costs one point of detail, which is
    // exactly the difference the flag is there to buy back.
    assert_eq!(simplify(&POINTS, Some(6.0), false).len(), 26);
    assert_eq!(simplify(&POINTS, Some(6.0), true).len(), 27);
}

#[test]
fn keeps_a_trailing_point_that_repeats_an_earlier_position() {
    // The last point is dropped only when it is the point that was just
    // kept -- not merely when it shares coordinates with it. Here the middle
    // point falls inside the tolerance, so the run ends on the first point
    // and the coincident final point is still appended.
    let points = [
        Point2D::new(0.0, 0.0),
        Point2D::new(0.5, 0.0),
        Point2D::new(0.0, 0.0),
    ];

    let expected = [Point2D::new(0.0, 0.0), Point2D::new(0.0, 0.0)];
    assert_eq!(simplify(&points, Some(1.0), false), expected);
    assert_eq!(simplify(&points, Some(1.0), true), expected);
}

#[test]
fn keeps_a_returning_point_that_repeats_the_start() {
    let points = [
        Point2D::new(0.0, 0.0),
        Point2D::new(10.0, 0.0),
        Point2D::new(0.2, 0.0),
        Point2D::new(0.0, 0.0),
    ];

    assert_eq!(
        simplify(&points, Some(1.0), false),
        [
            Point2D::new(0.0, 0.0),
            Point2D::new(10.0, 0.0),
            Point2D::new(0.0, 0.0),
        ]
    );
}

#[test]
fn drops_every_intermediate_point_within_tolerance() {
    let points = [
        Point2D::new(0.0, 0.0),
        Point2D::new(0.1, 0.0),
        Point2D::new(0.2, 0.0),
        Point2D::new(0.3, 0.0),
    ];

    assert_eq!(
        simplify(&points, Some(5.0), false),
        [Point2D::new(0.0, 0.0), Point2D::new(0.3, 0.0)]
    );
}

#[test]
fn handles_repeated_identical_points() {
    let points = [
        Point2D::new(5.0, 5.0),
        Point2D::new(5.0, 5.0),
        Point2D::new(5.0, 5.0),
    ];

    assert_eq!(
        simplify(&points, Some(1.0), false),
        [Point2D::new(5.0, 5.0), Point2D::new(5.0, 5.0)]
    );
}

#[test]
fn collapses_collinear_points() {
    let points = [
        Point2D::new(0.0, 0.0),
        Point2D::new(1.0, 1.0),
        Point2D::new(2.0, 2.0),
    ];

    assert_eq!(
        simplify(&points, Some(0.5), false),
        [Point2D::new(0.0, 0.0), Point2D::new(2.0, 2.0)]
    );
}

#[test]
fn collapses_axis_aligned_runs() {
    let vertical = [
        Point2D::new(1.0, 0.0),
        Point2D::new(1.0, 1.0),
        Point2D::new(1.0, 2.0),
        Point2D::new(1.0, 3.0),
        Point2D::new(1.0, 4.0),
    ];
    assert_eq!(
        simplify(&vertical, Some(0.1), true),
        [Point2D::new(1.0, 0.0), Point2D::new(1.0, 4.0)]
    );

    let horizontal = [
        Point2D::new(0.0, 7.0),
        Point2D::new(1.0, 7.0),
        Point2D::new(2.0, 7.0),
        Point2D::new(3.0, 7.0),
    ];
    assert_eq!(
        simplify(&horizontal, Some(0.1), true),
        [Point2D::new(0.0, 7.0), Point2D::new(3.0, 7.0)]
    );
}

#[test]
fn handles_zero_length_segments() {
    // The outer segment has zero length, so distances are measured to its
    // start point rather than through a division by zero.
    let points = [
        Point2D::new(0.0, 0.0),
        Point2D::new(0.0, 0.0),
        Point2D::new(5.0, 5.0),
        Point2D::new(0.0, 0.0),
        Point2D::new(0.0, 0.0),
    ];

    assert_eq!(
        simplify(&points, Some(0.1), true),
        [
            Point2D::new(0.0, 0.0),
            Point2D::new(5.0, 5.0),
            Point2D::new(0.0, 0.0),
        ]
    );
}

#[test]
fn keeps_the_corners_of_a_closed_loop() {
    let points = [
        Point2D::new(0.0, 0.0),
        Point2D::new(10.0, 0.0),
        Point2D::new(10.0, 10.0),
        Point2D::new(0.0, 10.0),
        Point2D::new(0.0, 0.0),
    ];

    assert_eq!(simplify(&points, Some(1.0), true), points);
}

#[test]
fn simplifies_negative_coordinates() {
    let points = [
        Point2D::new(-5.0, -5.0),
        Point2D::new(-2.5, -4.9),
        Point2D::new(0.0, 0.0),
        Point2D::new(3.0, -1.0),
        Point2D::new(7.0, 8.0),
    ];

    assert_eq!(simplify(&points, Some(1.0), false), points);
    assert_eq!(simplify(&points, Some(1.0), true), points);
}

#[test]
fn preserves_extreme_coordinate_magnitudes() {
    let huge = [
        Point2D::new(1e12, 1e12),
        Point2D::new(1e12 + 1.0, 1e12),
        Point2D::new(1e12 + 2.0, 1e12 + 3.0),
        Point2D::new(2e12, 1e12),
    ];
    assert_eq!(
        simplify(&huge, Some(1.5), true),
        [
            Point2D::new(1e12, 1e12),
            Point2D::new(1e12 + 2.0, 1e12 + 3.0),
            Point2D::new(2e12, 1e12),
        ]
    );

    let tiny = [
        Point2D::new(1e-9, 1e-9),
        Point2D::new(2e-9, 1e-9),
        Point2D::new(3e-9, 5e-9),
        Point2D::new(9e-9, 1e-9),
    ];
    assert_eq!(
        simplify(&tiny, Some(1e-9), true),
        [
            Point2D::new(1e-9, 1e-9),
            Point2D::new(3e-9, 5e-9),
            Point2D::new(9e-9, 1e-9),
        ]
    );
}

#[test]
fn always_keeps_the_first_and_last_point() {
    for &tolerance in &[0.0, 0.5, 1.0, 5.0, 100.0, 1e6] {
        for &highest_quality in &[false, true] {
            let result = simplify(&POINTS, Some(tolerance), highest_quality);

            assert!(result.len() >= 2);
            assert_eq!(result[0], POINTS[0]);
            assert_eq!(result[result.len() - 1], POINTS[POINTS.len() - 1]);
        }
    }
}

#[test]
fn matches_the_reference_tolerance_sweep() {
    // Output sizes recorded from the original JavaScript implementation.
    let expected = [
        (1.0, false, 100),
        (1.0, true, 100),
        (2.0, false, 70),
        (2.0, true, 71),
        (3.0, false, 46),
        (3.0, true, 46),
        (4.0, false, 39),
        (4.0, true, 39),
        (5.0, false, 33),
        (5.0, true, 33),
        (6.0, false, 26),
        (6.0, true, 27),
        (10.0, false, 15),
        (10.0, true, 15),
        (20.0, false, 9),
        (20.0, true, 9),
    ];

    for (tolerance, highest_quality, len) in expected {
        assert_eq!(
            simplify(&POINTS, Some(tolerance), highest_quality).len(),
            len,
            "tolerance {tolerance}, highest_quality {highest_quality}"
        );
    }
}

#[test]
fn simplifies_the_1k_fixture_across_tolerances() {
    // Output sizes recorded from the original JavaScript implementation.
    let expected = [
        (0.1, false, 1118),
        (0.1, true, 1118),
        (0.5, false, 258),
        (0.5, true, 257),
        (1.0, false, 141),
        (1.0, true, 144),
        (2.0, false, 71),
        (2.0, true, 71),
        (3.0, false, 42),
        (3.0, true, 46),
        (5.0, false, 28),
        (5.0, true, 33),
        (10.0, false, 15),
        (10.0, true, 15),
        (25.0, false, 7),
        (25.0, true, 8),
    ];

    let points = common::load_1k();
    assert_eq!(points.len(), 1118);

    for (tolerance, highest_quality, len) in expected {
        let result = simplify(&points, Some(tolerance), highest_quality);

        assert_eq!(
            result.len(),
            len,
            "tolerance {tolerance}, highest_quality {highest_quality}"
        );
        assert_eq!(result[0], Point2D::new(224.55, 250.15));
        assert_eq!(result[result.len() - 1], Point2D::new(866.36, 480.77));
    }
}

// --- the generic point surface ---

#[test]
fn preserves_custom_point_data() {
    #[derive(Clone, Debug, PartialEq)]
    struct Waypoint {
        lon: f64,
        lat: f64,
        name: &'static str,
    }

    impl Point for Waypoint {
        fn x(&self) -> f64 {
            self.lon
        }
        fn y(&self) -> f64 {
            self.lat
        }
    }

    let route = [
        Waypoint {
            lon: 0.0,
            lat: 0.0,
            name: "start",
        },
        Waypoint {
            lon: 0.1,
            lat: 0.0,
            name: "noise",
        },
        Waypoint {
            lon: 5.0,
            lat: 0.0,
            name: "end",
        },
    ];

    let result = simplify(&route, Some(1.0), false);

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].name, "start");
    assert_eq!(result[1].name, "end");
}

#[test]
fn accepts_tuples_and_arrays() {
    let tuples = [(0.0, 0.0), (1.0, 0.1), (2.0, 0.0)];
    assert_eq!(
        simplify(&tuples, Some(1.0), false),
        [(0.0, 0.0), (2.0, 0.0)]
    );

    let arrays = [[0.0, 0.0], [1.0, 0.1], [2.0, 0.0]];
    assert_eq!(
        simplify(&arrays, Some(1.0), false),
        [[0.0, 0.0], [2.0, 0.0]]
    );
}

#[test]
fn measures_distance_to_the_nearer_end_of_a_segment() {
    // The middle point projects past the far end of the segment joining its
    // neighbours, so its distance is measured to that end rather than to the
    // infinite line through them.
    let beyond_end = [
        Point2D::new(0.0, 0.0),
        Point2D::new(5.0, 1.0),
        Point2D::new(1.0, 0.0),
    ];
    assert_eq!(simplify(&beyond_end, Some(1.0), true), beyond_end);
    assert_eq!(simplify(&beyond_end, Some(1.0), false), beyond_end);
    assert_eq!(
        simplify(&beyond_end, Some(10.0), true),
        [Point2D::new(0.0, 0.0), Point2D::new(1.0, 0.0)]
    );

    // ...and the mirror image, projecting back past the near end.
    let before_start = [
        Point2D::new(0.0, 0.0),
        Point2D::new(-5.0, 1.0),
        Point2D::new(1.0, 0.0),
    ];
    assert_eq!(simplify(&before_start, Some(1.0), true), before_start);
    assert_eq!(
        simplify(&before_start, Some(10.0), true),
        [Point2D::new(0.0, 0.0), Point2D::new(1.0, 0.0)]
    );

    let both = [
        Point2D::new(0.0, 0.0),
        Point2D::new(-3.0, 2.0),
        Point2D::new(8.0, 2.0),
        Point2D::new(1.0, 0.0),
    ];
    assert_eq!(simplify(&both, Some(1.0), true), both);
    assert_eq!(simplify(&both, Some(1.0), false), both);
}

#[test]
fn simplifies_borrowed_points() {
    let owned = [
        Point2D::new(0.0, 0.0),
        Point2D::new(1.0, 0.1),
        Point2D::new(2.0, 0.0),
    ];
    let borrowed: Vec<&Point2D> = owned.iter().collect();

    let result = simplify(&borrowed, Some(1.0), false);

    assert_eq!(result, [&owned[0], &owned[2]]);
}

#[test]
fn converts_tuples_and_arrays_into_points() {
    assert_eq!(Point2D::from((1.0, 2.0)), Point2D::new(1.0, 2.0));
    assert_eq!(Point2D::from([3.0, 4.0]), Point2D::new(3.0, 4.0));
    assert_eq!(Point2D::default(), Point2D::new(0.0, 0.0));
}
