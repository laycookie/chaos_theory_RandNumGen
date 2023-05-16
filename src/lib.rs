use wasm_bindgen::prelude::*;
use serde::{Serialize};
use std::f64;
use web_sys::window;

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
    length: f64,
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
pub fn gen_number(circle_amount_x: i32, circle_amount_y: i32, spacing: i32, radius: i32, ini_laser_offset_y: f64, ini_laser_angle: f64) -> JsValue {
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

    fn laser_beam_calc(laser_pos_y: f64, laser_pos_x: f64, laser_angle: f64, world: &mut World)  {
        let mut reflecting_circle: &Circle;
        let mut dist_from_reflection: Option<f64> = None;
        let mut laser_end_x= -1f64;
        let mut laser_end_y= -1f64;


        let mut filtered_angle = laser_angle;

        filtered_angle = laser_angle % 360f64;

        if laser_angle < 0f64 {
            filtered_angle = 360f64 + laser_angle;
        }

        let angle_from_deg = ((90f64 - filtered_angle) * (std::f64::consts::PI/180f64)).tan();
        for circle in &world.circles {
            let circle_x = circle.x - laser_pos_x;
            let circle_y = circle.y - laser_pos_y;

            let a = angle_from_deg.powf(2f64) + 1f64;
            let b = -2f64 * (circle_y * angle_from_deg + circle_x);
            let c = circle_x.powf(2f64) + circle_y.powf(2f64) - circle.radius.powf(2f64);

            let inside_sqrt = b.powf(2f64) - 4f64 * a * c;
            if inside_sqrt < 0f64 {
                continue;
            }
            let pos_quadratic = (-b + inside_sqrt.sqrt()) / (2f64 * a);
            // this is are point x
            let neg_quadratic = (-b - inside_sqrt.sqrt()) / (2f64 * a);
            laser_end_x = neg_quadratic;

            // if pos is bigger than neg that means that the interception is happening when x is negative so we ignore it.

            if pos_quadratic.abs() < neg_quadratic.abs() {
                continue;
            }

            let y_point = neg_quadratic * angle_from_deg;
            laser_end_y = y_point;

            if dist_from_reflection == None {
                reflecting_circle = circle;
                dist_from_reflection = Some(f64::hypot(neg_quadratic, y_point));
            } else if dist_from_reflection > Some(f64::hypot(neg_quadratic, y_point)) {
                reflecting_circle = circle;
                dist_from_reflection = Some(f64::hypot(neg_quadratic, y_point));
            }
        }

        if dist_from_reflection != None {
            world.laser_beams.push(LaserBeam {
                x: laser_pos_x,
                y: laser_pos_y,
                angle: filtered_angle,
                length: dist_from_reflection.unwrap(),
                end_x: laser_end_x + laser_pos_x,
                end_y: laser_end_y + laser_pos_y,
            });
        } else {
            world.laser_beams.push(LaserBeam {
                x: laser_pos_x,
                y: laser_pos_y,
                angle: filtered_angle,
                length: -1f64,
                end_x: laser_end_x + laser_pos_x,
                end_y: laser_end_y + laser_pos_y,
            });
        }
    }
    laser_beam_calc(ini_laser_offset_y, 0f64, ini_laser_angle, &mut world);


    let json_string = serde_json::to_string(&world).unwrap();
    return JsValue::from_str(&json_string);
}