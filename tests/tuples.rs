use cucumber::{given, then, World};

// Need to explicitly add path to mod.
// Not idiomatic Rust, but work around for TDD Cucumber.
#[path = "../src/tuple.rs"]
mod tuple;

use tuple::*;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TupleWorld {
    tuple: Tuple,
}

/* Givens */

#[given(expr = "a <- tuple[{float}, {float}, {float}, {float}]")]
fn create_tuple(world: &mut TupleWorld, x: f64, y: f64, z: f64, w: f64) {
    world.tuple = Tuple {
        x: x,
        y: y,
        z: z,
        w: w,
    };
}

#[given(expr = "p <- point[{float}, {float}, {float}]")]
fn create_point(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    world.tuple = point(x, y, z);
}

#[given(expr = "p <- vector[{float}, {float}, {float}]")]
fn create_vector(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    world.tuple = vector(x, y, z);
}

/* Thens */

#[then(expr = "a.x = {float}")]
fn test_x(world: &mut TupleWorld, x: f64) {
    assert!(world.tuple.x == 4.3);
}

#[then(expr = "a.y = {float}")]
fn test_y(world: &mut TupleWorld, y: f64) {
    assert!(world.tuple.y == y);
}

#[then(expr = "a.z = {float}")]
fn test_z(world: &mut TupleWorld, z: f64) {
    assert!(world.tuple.z == z);
}

#[then(expr = "a.w = {float}")]
fn test_w(world: &mut TupleWorld, w: f64) {
    assert!(world.tuple.w == w);
}

#[then(regex = "^a is a (vector|point)$")]
fn is_a_point(world: &mut TupleWorld, tuple: String) {
    match tuple.as_str() {
        "point" => assert!(world.tuple.w == 1.0),
        "vector" => assert!(world.tuple.w == 0.0),
        _ => unreachable!(),
    }
}

/*
* () in cucumber expression is an optional expression
* using []'s for method construction
*/

#[then(regex = "^a is not a (vector|point)$")]
fn is_not_a_vector(world: &mut TupleWorld, tuple: String) {
    match tuple.as_str() {
        "point" => assert!(world.tuple.w != 1.0),
        "vector" => assert!(world.tuple.w != 0.0),
        _ => unreachable!(),
    }
}

#[then(expr = "p = tuple[{float}, {float}, {float}, {float}]")]
fn p_equals(world: &mut TupleWorld, x: f64, y: f64, z: f64, w: f64) {
    assert!(world.tuple.x == x);
    assert!(world.tuple.y == y);
    assert!(world.tuple.z == z);
    assert!(world.tuple.w == w);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TupleWorld::run("tests/features/tuples.feature"));
}
