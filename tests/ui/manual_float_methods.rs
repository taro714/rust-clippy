//@aux-build:proc_macros.rs:proc-macro
#![allow(clippy::needless_if, unused)]
#![warn(clippy::manual_is_infinite, clippy::manual_is_finite)]

#[macro_use]
extern crate proc_macros;

const INFINITE: f32 = f32::INFINITY;
const NEG_INFINITE: f32 = f32::NEG_INFINITY;

fn main() {
    let x = 1.0f32;
    if x == f32::INFINITY || x == f32::NEG_INFINITY {}
    if x != f32::INFINITY && x != f32::NEG_INFINITY {}
    if x == INFINITE || x == NEG_INFINITE {}
    if x != INFINITE && x != NEG_INFINITE {}
    let x = 1.0f64;
    if x == f64::INFINITY || x == f64::NEG_INFINITY {}
    if x != f64::INFINITY && x != f64::NEG_INFINITY {}
    // Don't lint
    if x.is_infinite() {}
    if x.is_finite() {}
    // If they're doing it this way, they probably know what they're doing
    if x.abs() < f64::INFINITY {}
    external! {
        let x = 1.0;
        if x == f32::INFINITY || x == f32::NEG_INFINITY {}
        if x != f32::INFINITY && x != f32::NEG_INFINITY {}
    }
    with_span! {
        span
        let x = 1.0;
        if x == f32::INFINITY || x == f32::NEG_INFINITY {}
        if x != f32::INFINITY && x != f32::NEG_INFINITY {}
    }
}
