use std::ptr::null;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
struct World {
    circles: Vec<Circle>,
    laser_beams: Vec<LaserBeam>,
}

#[derive(Clone)]
struct LaserBeam {
    x: f64,
    y: f64,
    angle: f64,
    length: f64,
}

#[derive(Clone)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

#[wasm_bindgen]
pub fn gen_number(circle_amount_x: i32, circle_amount_y: i32, spacing: i32, radius: i32, ini_laser_angle: f64, ini_laser_offset_y: f64) -> i64 {
    let mut world: World = World { circles: Vec::new(), laser_beams: Vec::new() };

    // generate circles in the world
    for x in 0..circle_amount_x {
        for y in 0..circle_amount_y{
            let mut x_pos = x;
            if x % 2 ==0 {
                x_pos = x_pos + spacing;
            }
            world.circles.push(Circle {
                // subtract half of the amount of circles to center the pattern
                x: (x_pos*spacing - (circle_amount_x/2)) as f64,
                y: (y*spacing - (circle_amount_y/2)) as f64,
                radius: radius as f64 });
        }
    }

    fn laser_trajectory(laser_pos_y: f64, laser_pos_x: f64, laser_angle: f64, world: World) {
        let reflecting_circle: Circle;

        let angle_from_deg = ((90f64 - laser_angle) * (std::f64::consts::PI/180f64)).tan();
        for circle in world.circles {
            let circle_x = circle.x - laser_pos_x;
            let circle_y = circle.y - laser_pos_y;

            let a = angle_from_deg.powf(2f64) + 1f64;
            let b = -2f64 * (circle_x * angle_from_deg + circle_y);
            let c = circle_x.powf(2f64) + circle_y.powf(2f64) - circle.radius.powf(2f64);

            let inside_sqrt = b.powf(2f64) - 4f64 * a * c;
            if inside_sqrt < 0f64 {
                continue;
            }
            let pos_quadratic = (-b + inside_sqrt.sqrt()) / (2f64 * a);
            let neg_quadratic = (-b - inside_sqrt.sqrt()) / (2f64 * a);

        }
    };
    laser_trajectory(ini_laser_offset_y, 0f64, ini_laser_angle, world.clone());



    return world.circles.len() as i64;
}