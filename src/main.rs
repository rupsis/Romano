mod tuple;
use tuple::{point, vector, Point, Vector};

mod canvas;
use canvas::{canvas, Canvas};

mod color;
use color::color;

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

fn main() {
    let mut proj = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.0, 0.0).normalize(),
    };

    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    // loop {
    //     tick(&env, &mut proj);
    //
    //     dbg!(&proj);
    //
    //     if proj.position.y <= 0.0 {
    //         break;
    //     }
    // }

    let mut canvas = canvas(2, 2);
    canvas.write_pixel(0, 0, color(1.5, 0.0, 0.0));
    canvas.write_pixel(1, 0, color(0.0, 0.5, 0.0));
    canvas.write_pixel(0, 1, color(-0.5, 0.0, 1.0));
    canvas.write_pixel(1, 1, color(0.5, 5.0, 0.0));

    canvas.save_to_file("src/test.ppm");
}
