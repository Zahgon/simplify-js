//! Benchmark for the polyline simplifier.
//!
//! The original suite was built on benchmark.js, which samples repeatedly and
//! reports throughput with a margin of error rather than a single timing. This
//! reproduces that: the same two cases over the same fixture, printed in the
//! same shape benchmark.js used —
//! `name x N ops/sec ±E% (S runs sampled)`.

use std::hint::black_box;
use std::time::{Duration, Instant};

use simplify::simplify;

#[path = "../tests/common/mod.rs"]
mod common;

/// How many throughput samples to collect per case.
const SAMPLES: u32 = 100;

/// A sample runs at least this long, so the clock's resolution does not
/// dominate the measurement.
const MIN_SAMPLE_TIME: Duration = Duration::from_millis(10);

/// Times `run` repeatedly and reports operations per second.
fn bench(name: &str, mut run: impl FnMut()) {
    // Find an iteration count that keeps each sample comfortably above the
    // clock's resolution.
    let mut iterations: u32 = 1;
    loop {
        let start = Instant::now();
        for _ in 0..iterations {
            run();
        }
        if start.elapsed() >= MIN_SAMPLE_TIME {
            break;
        }
        iterations *= 2;
    }

    // Warm up, so the first sample is not paying for cold caches.
    for _ in 0..iterations {
        run();
    }

    let mut rates = Vec::with_capacity(SAMPLES as usize);
    for _ in 0..SAMPLES {
        let start = Instant::now();
        for _ in 0..iterations {
            run();
        }
        rates.push(f64::from(iterations) / start.elapsed().as_secs_f64());
    }

    let n = f64::from(SAMPLES);
    let mean = rates.iter().sum::<f64>() / n;
    let variance = rates.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (n - 1.0);
    // Normal approximation to the 95% confidence interval on the mean, which
    // is what benchmark.js reports as its "±" figure.
    let margin = 1.96 * (variance.sqrt() / n.sqrt()) / mean * 100.0;

    println!("{name} x {mean:.0} ops/sec \u{b1}{margin:.2}% ({SAMPLES} runs sampled)");
}

fn main() {
    let points = common::load_1k();
    println!("Benchmarking simplify on {} points...", points.len());

    bench("simplify (HQ)", || {
        black_box(simplify(black_box(&points), Some(1.0), true));
    });
    bench("simplify", || {
        black_box(simplify(black_box(&points), Some(1.0), false));
    });
}
