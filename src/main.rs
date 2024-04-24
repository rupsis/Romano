mod tuple;
use tuple::{point, vector, Point, Vector};

mod canvas;
use canvas::canvas;

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

fn to_canvas(n: u32, max: u32) -> u32 {
    n.min(max - 1).max(0)
}

fn main() {
    let mut canvas = canvas(900, 500);

    let mut proj = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    let orange = color(0.98, 0.54, 0.0);

    loop {
        tick(&env, &mut proj);

        if proj.position.y <= 0.0 {
            break;
        }

        canvas.write_pixel(
            to_canvas(proj.position.x.round() as u32, canvas.width),
            to_canvas((f64::from(canvas.height) - proj.position.y).round() as u32, canvas.height),
            orange,
        );
    }

    canvas.save_to_file("src/test.ppm");
}
