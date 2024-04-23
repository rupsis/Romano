use float_cmp::approx_eq;
use std::collections::HashMap;
use std::io;

use cucumber::writer::*;
use cucumber::{gherkin::Step, given, then, when, writer, World};

#[path = "../src/tuple.rs"]
mod tuple;

#[path = "../src/canvas.rs"]
mod canvas;

use canvas::*;

#[path = "../src/color.rs"]
mod color;

use color::*;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct CanvasWorld {
    canvas: Canvas,
    colors: HashMap<String, Color>,
    ppm: String,
}

fn strip_newlines(line: &str) -> String {
    line.strip_prefix("\n")
        .unwrap()
        .strip_suffix("\n")
        .unwrap()
        .to_string()
}

/* Givens */
#[given(expr = "c <- canvas[{int}, {int}]")]
fn create_canvas(world: &mut CanvasWorld, width: i32, height: i32) {
    world.canvas = canvas(width.try_into().unwrap(), height.try_into().unwrap());
}

#[given(expr = "{word} <- color[{float}, {float}, {float}]")]
fn create_color(world: &mut CanvasWorld, key: String, r: f64, g: f64, b: f64) {
    world.colors.insert(key, color(r, g, b));
}

/* Thens */

#[then(expr = "c.width = {int}")]
fn test_width(world: &mut CanvasWorld, width: i32) {
    assert!(world.canvas.width == width.try_into().unwrap());
}

#[then(expr = "c.height = {int}")]
fn test_height(world: &mut CanvasWorld, height: i32) {
    assert!(world.canvas.height == height.try_into().unwrap());
}

#[then(expr = "every pixel of c is color[{float}, {float}, {float}]")]
fn test_pixels(world: &mut CanvasWorld, r: f64, g: f64, b: f64) {
    for pixel in world.canvas.pixels.iter() {
        assert!(pixel.r == r);
        assert!(pixel.g == g);
        assert!(pixel.b == b);
    }
}

#[when(expr = "write_pixel[{int}, {int}, {word}]")]
fn test_write_pixel(world: &mut CanvasWorld, x: i32, y: i32, key: String) {
    world.canvas.write_pixel(
        x.try_into().unwrap(),
        y.try_into().unwrap(),
        *world.colors.get(&key).unwrap(),
    );
}

#[then(expr = "pixel_at[{int}, {int}] = {word}")]
fn test_pixel_at(world: &mut CanvasWorld, x: i32, y: i32, key: String) {
    assert!(
        world
            .canvas
            .pixel_at(x.try_into().unwrap(), y.try_into().unwrap())
            == *world.colors.get(&key).unwrap()
    );
}

#[when("ppm <- to_ppm")]
fn test_canvas_to_ppm(world: &mut CanvasWorld) {
    world.ppm = world.canvas.to_ppm()
}

#[then(expr = "lines 1-3 of ppm are")]
fn test_ppm_header(world: &mut CanvasWorld, step: &Step) {
    assert!(
        strip_newlines(step.docstring.as_ref().unwrap())
            == world.ppm.lines().take(3).collect::<Vec<&str>>().join("\n")
    );
}

#[then(expr = "lines 4-6 of ppm are")]
fn test_ppm_body(world: &mut CanvasWorld, step: &Step) {
    dbg!(strip_newlines(step.docstring.as_ref().unwrap()));
    dbg!(world.ppm.lines().take(3).collect::<Vec<&str>>().join("\n"));
    assert!(
        strip_newlines(step.docstring.as_ref().unwrap())
            == world.ppm.lines().take(3).collect::<Vec<&str>>().join("\n")
    );
}

// fn main() {
//     futures::executor::block_on(CanvasWorld::run("tests/features/canvas.feature"));
// }

#[tokio::main]
async fn main() {
    CanvasWorld::cucumber()
        .max_concurrent_scenarios(1)
        .with_writer(
            writer::Basic::raw(io::stdout(), writer::Coloring::Never, 0)
                .summarized()
                .assert_normalized(),
        )
        .run_and_exit("tests/features/canvas.feature")
        .await;
}
