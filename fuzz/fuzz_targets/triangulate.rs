#![no_main]
use libfuzzer_sys::fuzz_target;
use delaunator;

fuzz_target!(|input: Vec<(f64, f64)>| {
    if input.iter().all(|x| x.0.is_finite() && x.1.is_finite()) {
        let points: Vec<_> = input.iter().map(|x| delaunator::Point { x: x.0, y: x.1 }).collect();
        delaunator::triangulate(&points);
    }
});