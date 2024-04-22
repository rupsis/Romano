use float_cmp::approx_eq;
use std::collections::HashMap;
use std::io;

use cucumber::writer::*;
use cucumber::{given, then, when, writer, World};

#[path = "../src/tuple.rs"]
mod tuple;

// Need to explicitly add path to mod.
// Not idiomatic Rust, but work around for TDD Cucumber.
#[path = "../src/color.rs"]
mod color;

use color::*;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct ColorWorld {
    tuples: HashMap<String, Color>,
}

/* Givens */
#[given(expr = "{word} <- color[{float}, {float}, {float}]")]
fn create_color(world: &mut ColorWorld, key: String, r: f64, g: f64, b: f64) {
    world.tuples.insert(key, Color { r, g, b });
}

/* Thens */

#[then(expr = "{word}.r = {float}")]
fn test_r(world: &mut ColorWorld, key: String, r: f64) {
    assert!(world.tuples.get(&key).unwrap().r == r);
}

#[then(expr = "{word}.g = {float}")]
fn test_g(world: &mut ColorWorld, key: String, g: f64) {
    assert!(world.tuples.get(&key).unwrap().g == g);
}

#[then(expr = "{word}.b = {float}")]
fn test_b(world: &mut ColorWorld, key: String, b: f64) {
    assert!(world.tuples.get(&key).unwrap().b == b);
}

#[then(expr = "{word} + {word} = color[{float}, {float}, {float}]")]
fn tuple_add_equals(world: &mut ColorWorld, key1: String, key2: String, r: f64, g: f64, b: f64) {
    let n: Color = *world.tuples.get(&key1).unwrap() + *world.tuples.get_mut(&key2).unwrap();
    assert!(n.r == r);
    assert!(n.g == g);
    assert!(n.b == b);
}

#[then(expr = "{word} - {word} = color[{float}, {float}, {float}]")]
fn color_minus_equals(world: &mut ColorWorld, key1: String, key2: String, r: f64, g: f64, b: f64) {
    let n: Color = *world.tuples.get(&key1).unwrap() - *world.tuples.get(&key2).unwrap();
    dbg!("{:?}", n);
    assert!(approx_eq!(f64, n.r, r, epsilon = tuple::EPSILON));
    assert!(approx_eq!(f64, n.g, g, epsilon = tuple::EPSILON));
    assert!(approx_eq!(f64, n.b, b, epsilon = tuple::EPSILON));
}

#[then(expr = "{word} * {word} = color[{float}, {float}, {float}]")]
fn color_multiply_equals(
    world: &mut ColorWorld,
    key1: String,
    key2: String,
    r: f64,
    g: f64,
    b: f64,
) {
    let n: Color = *world.tuples.get(&key1).unwrap() * *world.tuples.get(&key2).unwrap();
    assert!(approx_eq!(f64, n.r, r, epsilon = tuple::EPSILON));
    assert!(approx_eq!(f64, n.g, g, epsilon = tuple::EPSILON));
    assert!(approx_eq!(f64, n.b, b, epsilon = tuple::EPSILON));
}

#[then(expr = "{word} * {float}  = color[{float}, {float}, {float}]")]
fn color_scalar_equals(world: &mut ColorWorld, key: String, scalar: f64, r: f64, g: f64, b: f64) {
    let n: Color = *world.tuples.get(&key).unwrap() * scalar;
    assert!(approx_eq!(f64, n.r, r, epsilon = tuple::EPSILON));
    assert!(approx_eq!(f64, n.g, g, epsilon = tuple::EPSILON));
    assert!(approx_eq!(f64, n.b, b, epsilon = tuple::EPSILON));
}

fn main() {
    futures::executor::block_on(ColorWorld::run("tests/features/colors.feature"));
}
