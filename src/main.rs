mod tuple;
use tuple::{point, vector, Point, Vector};

mod canvas;
mod color;

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

    loop {
        tick(&env, &mut proj);

        dbg!(&proj);

        if proj.position.y <= 0.0 {
            break;
        }
    }
}
