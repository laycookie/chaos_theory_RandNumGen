use wasm_bindgen::prelude::*;

struct World {
    circles: Vec<Circle>,
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

}

#[wasm_bindgen]
pub fn greet(circle_amount: i32) -> String {
    let mut world: World = World { circles: Vec::new() };
    for i in 0..circle_amount {
        world.circles.push(Circle::new((i*3) as f64, (i*3) as f64, 2 as f64));
    }

    return format!("{}", world.circles.len());
}