use std::collections::HashMap;
use std::io;

use cucumber::writer::*;
use cucumber::{given, then, writer, World};

// Need to explicitly add path to mod.
// Not idiomatic Rust, but work around for TDD Cucumber.
#[path = "../src/tuple.rs"]
mod tuple;

use tuple::*;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TupleWorld {
    tuples: HashMap<String, Tuple>,
}

/* Givens */
// TODO make these more robust
#[given(expr = "{word} <- tuple[{float}, {float}, {float}, {float}]")]
fn create_tuple(world: &mut TupleWorld, key: String, x: f64, y: f64, z: f64, w: f64) {
    world.tuples.insert(key, Tuple { x, y, z, w });
    ()
}

#[given(expr = "{word} <- point[{float}, {float}, {float}]")]
fn create_point(world: &mut TupleWorld, key: String, x: f64, y: f64, z: f64) {
    world.tuples.insert(key, point(x, y, z));
    ()
}

#[given(expr = "{word} <- vector[{float}, {float}, {float}]")]
fn create_vector(world: &mut TupleWorld, key: String, x: f64, y: f64, z: f64) {
    world.tuples.insert(key, vector(x, y, z));
    ()
}

/* Thens */

#[then(expr = "{word}.x = {float}")]
fn test_x(world: &mut TupleWorld, key: String, x: f64) {
    assert!(world.tuples.get(&key).unwrap().x == x);
}

#[then(expr = "{word}.y = {float}")]
fn test_y(world: &mut TupleWorld, key: String, y: f64) {
    assert!(world.tuples.get(&key).unwrap().y == y);
}

#[then(expr = "{word}.z = {float}")]
fn test_z(world: &mut TupleWorld, key: String, z: f64) {
    assert!(world.tuples.get(&key).unwrap().z == z);
}

#[then(expr = "{word}.w = {float}")]
fn test_w(world: &mut TupleWorld, key: String, w: f64) {
    assert!(world.tuples.get(&key).unwrap().w == w);
}

#[then(expr = "{word} is a {word}")]
fn is_a_point(world: &mut TupleWorld, key: String, tuple: String) {
    match tuple.as_str() {
        "point" => assert!(world.tuples.get(&key).unwrap().w == 1.0),
        "vector" => assert!(world.tuples.get(&key).unwrap().w == 0.0),
        _ => unreachable!(),
    }
}

/*
* () in cucumber expression is an optional expression
* using []'s for method construction
*/

#[then(expr = "{word} is not a {word}")]
fn is_not_a_vector(world: &mut TupleWorld, key: String, tuple: String) {
    match tuple.as_str() {
        "point" => assert!(world.tuples.get(&key).unwrap().w != 1.0),
        "vector" => assert!(world.tuples.get(&key).unwrap().w != 0.0),
        _ => unreachable!(),
    }
}

#[then(expr = "{word} = tuple[{float}, {float}, {float}, {float}]")]
fn p_equals(world: &mut TupleWorld, key: String, x: f64, y: f64, z: f64, w: f64) {
    let n: Tuple = *world.tuples.get(&key).unwrap();
    assert!(n.x == x);
    assert!(n.y == y);
    assert!(n.z == z);
    assert!(n.w == w);
}
#[then(expr = "- {word} = tuple[{float}, {float}, {float}, {float}]")]
fn neg_equals(world: &mut TupleWorld, key: String, x: f64, y: f64, z: f64, w: f64) {
    let n: Tuple = -*world.tuples.get(&key).unwrap();
    assert!(n.x == x);
    assert!(n.y == y);
    assert!(n.z == z);
    assert!(n.w == w);
}

#[then(expr = "{word} + {word} = tuple[{float}, {float}, {float}, {float}]")]
fn tuple_add_equals(
    world: &mut TupleWorld,
    key1: String,
    key2: String,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let n: Tuple = *world.tuples.get(&key1).unwrap() + *world.tuples.get_mut(&key2).unwrap();
    assert!(n.x == x);
    assert!(n.y == y);
    assert!(n.z == z);
    assert!(n.w == w);
}

#[then(expr = "{word} - {word} = {word}[{float}, {float}, {float}]")]
fn point_minus_equals(
    world: &mut TupleWorld,
    key1: String,
    key2: String,
    f_or_v: String,
    x: f64,
    y: f64,
    z: f64,
) {
    let n: Tuple = *world.tuples.get(&key1).unwrap() - *world.tuples.get_mut(&key2).unwrap();
    assert!(n.x == x);
    assert!(n.y == y);
    assert!(n.z == z);
    let w = if f_or_v == "vector" { 0.0 } else { 1.0 };
    assert!(n.w == w);
}

#[then(expr = "{word} * {float} = tuple[{float}, {float}, {float}, {float}]")]
fn tuple_scalar_equals(
    world: &mut TupleWorld,
    key: String,
    scalar: f64,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let n: Tuple = *world.tuples.get(&key).unwrap() * scalar;
    assert!(n.x == x);
    assert!(n.y == y);
    assert!(n.z == z);
    assert!(n.w == w);
}

#[then(expr = "{word} \\/ {float} = tuple[{float}, {float}, {float}, {float}]")]
fn tuple_div_equals(
    world: &mut TupleWorld,
    key: String,
    scalar: f64,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let n: Tuple = *world.tuples.get(&key).unwrap() / scalar;
    assert!(n.x == x);
    assert!(n.y == y);
    assert!(n.z == z);
    assert!(n.w == w);
}

#[then(expr = "magnitude[{word}] = {float}")]
fn tuple_magnitude_equals(world: &mut TupleWorld, key: String, answer: f64) {
    let n: f64 = world.tuples.get(&key).unwrap().magnitude();
    assert!(n == answer);
}

// This runs before everything else, so you can setup things here.
#[tokio::main]
async fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TupleWorld::run("tests/features/tuples.feature"));
    // TupleWorld::cucumber()
    // .max_concurrent_scenarios(1)
    // .with_writer(
    //     writer::Basic::raw(io::stdout(), writer::Coloring::Never, 0)
    //         .summarized()
    //         .assert_normalized(),
    // )
    // .run_and_exit("tests/features/tuples.feature")
    // .await;
}
