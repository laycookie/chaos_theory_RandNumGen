use wasm_bindgen::prelude::*;
use serde::{Serialize};
use web_sys::{window};

#[derive(Serialize)]
struct World {
    circles: Vec<Circle>,
    laser_beams: Vec<LaserBeam>,
}

#[derive(Serialize)]
struct LaserBeam {
    x: f64,
    y: f64,
    angle: f64,
    ref_angle: f64,
    end_x: f64,
    end_y: f64,
}

#[derive(Serialize)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}



#[wasm_bindgen]
pub fn simulate(circle_amount_x: i32, circle_amount_y: i32, spacing: i32, radius: i32, ini_laser_offset_x: f64, ini_laser_offset_y: f64, ini_laser_angle: f64) -> JsValue {
    let mut world: World = World { circles: Vec::new(), laser_beams: Vec::new() };

    // generate circles in the world
    for x in 0..circle_amount_x {
        for y in 0..circle_amount_y{
            world.circles.push(Circle {
                x: (x*spacing - (circle_amount_x/2)) as f64,
                y: (y*spacing - (circle_amount_y/2)) as f64,
                radius: radius as f64 });
        }
    }

    fn simulate_laser(laser_x_offset: f64, laser_y_offset: f64, angle: f64, world_ref: &mut World) {
        let mut end_x: f64 = -1f64;
        let mut end_y: f64 = -1f64;
        let mut ref_angle: f64 = -1f64;

        for circle in &mut world_ref.circles {
            // centering the laser by changing circles location
            let cir_x = circle.x - laser_x_offset;
            let cir_y = circle.y - laser_y_offset;

            // make sure the angle is always positive and within 360 degrees
            let mut angle = angle;
            while angle < 0f64 {
                angle += 360f64;
            }
            while angle > 360f64 {
                angle -= 360f64;
            }


            // calculate the intersection of the laser and the circle
            let pi = std::f64::consts::PI;
            let a = 1.0 + ((angle*pi)/180f64).tan().powi(2);
            let b = (-2f64 * cir_y * ((pi*angle)/180f64).tan()) - (2f64 * cir_x);
            let c = cir_y.powi(2) - circle.radius.powi(2) + cir_x.powi(2);

            let raw_intersection = quadratic(a, b, c);
            let intersection: (f64, f64);
            if raw_intersection.is_none() {
                continue;
            } else {
                intersection = raw_intersection.unwrap();
            }

            let neg = intersection.1;
            let pos = intersection.0;
            // alert
            let window = window().unwrap();
            window.alert_with_message(&format!("c_x: {cir_x} c_y: {cir_y}, pos: {pos} neg: {neg}")).unwrap();

            // validate that only one side of the circle is hit
            if intersection.0.abs() < 0f64 && intersection.1.abs() < 0f64 {
                continue;
            }

            // calculate the angle of the laser beam
            ref_angle = (cir_y / cir_x).atan();
            let laser_angle = 2.0 * ref_angle - angle;

            // calculate the end of the laser beam
            // TODO: fix the end of the laser beam
            if intersection.1 < 0f64 {
                end_x = intersection.0;
                end_y = (angle * (pi/180f64)).tan() * intersection.0;
            } else {
                end_x = intersection.1;
                end_y = (angle * (pi/180f64)).tan() * intersection.1;
            }
        }
        // add the laser beam to the world
        world_ref.laser_beams.push(LaserBeam {
            x: laser_x_offset,
            y: laser_y_offset,
            angle,
            ref_angle,
            end_x,
            end_y,
        });
    }
    simulate_laser(ini_laser_offset_x, ini_laser_offset_y, ini_laser_angle, &mut world);


    let json_string = serde_json::to_string(&world).unwrap();
    return JsValue::from_str(&json_string);
}

fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let delta = b.powi(2) - 4.0*a*c;
    if delta < 0.0 {
        return None;
    }
    let x1 = (-b + delta.sqrt()) / (2.0*a);
    let x2 = (-b - delta.sqrt()) / (2.0*a);
    return Some((x1, x2));
}