#![no_main]
use libfuzzer_sys::fuzz_target;
use delaunator;

fuzz_target!(|input: (f64, f64)| {
    delaunator::Point { x: input.0, y: input.1 };
});