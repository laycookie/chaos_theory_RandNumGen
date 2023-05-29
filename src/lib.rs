use wasm_bindgen::prelude::*;
use serde::{Serialize};
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
    reflecting_angle: f64,
    bounces: bool,
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

    fn simulate_laser(laser_x_offset: f64, laser_y_offset: f64, in_angle: f64, world_ref: &mut World) {
        let mut end_x: f64 = -1f64;
        let mut end_y: f64 = -1f64;
        let mut reflecting_angle: f64 = -1f64;
        let mut bounces: bool = false;

        let mut angle = in_angle;
        // make sure the angle is always positive and within 360 degrees
        fn filter_angle(angle:&mut f64) {
            while *angle < 0f64 {
                *angle += 360f64;
            }
            while *angle >= 360f64 {
                *angle -= 360f64;
            }
        }
        filter_angle(&mut angle);
        for circle in &mut world_ref.circles {
            // centering the laser by changing circles location
            let cir_x = circle.x - laser_x_offset;
            let cir_y = circle.y - laser_y_offset;


            // checking for angles at which quadrants intersect
            if angle == 0f64 {
                if !(cir_x > 0f64) {
                    continue;
                }
            } else if angle == 90f64 {
                if cir_y < 0f64 {
                    continue;
                }
            } else if angle == 180f64 {
                if cir_x > 0f64 {
                    continue;
                }
            } else if angle == 270f64 {
                if cir_y > 0f64 {
                    continue;
                }
            }
            // validate that the circle is in the correct quadrant based on the angle
            else if angle > 0f64 && angle < 90f64 {
                if cir_x < 0f64 && cir_y < 0f64 {
                    continue;
                }
            } else if angle > 90f64 && angle < 180f64 {
                if cir_x > 0f64 && cir_y < 0f64 {
                    continue;
                }
            } else if angle > 180f64 && angle < 270f64 {
                if cir_x > 0f64 && cir_y > 0f64 {
                    continue;
                }
            } else if angle > 270f64 && angle < 360f64 {
                if cir_x < 0f64 && cir_y > 0f64 {
                    continue;
                }
            }

            // calculate the intersection of the laser and the circle
            let pi = std::f64::consts::PI;
            let a;
            let b;
            let c;

            // TODO: Fix for negative tan
            if !(angle == 90f64 || angle == 270f64) {
                a = 1.0 + ((angle*pi)/180f64).tan().powi(2);
                b = (-2f64 * cir_y * ((pi*angle)/180f64).tan()) - (2f64 * cir_x);
                c = cir_y.powi(2) - circle.radius.powi(2) + cir_x.powi(2);
            } else {
                a = 1f64;
                b = -2f64 * cir_y;
                c = cir_y.powi(2) - circle.radius.powi(2) + cir_x.powi(2);
            }


            let r_a = round_to_2_decimals(a, 12);
            let r_b = round_to_2_decimals(b, 12);
            let r_c = round_to_2_decimals(c, 12);

            let raw_intersection = quadratic(r_a, r_b, r_c);

            // decompiles quadratic and skips if answer if imagery
            let intersection: (f64, f64);
            if raw_intersection.is_none() {
                continue;
            } else {
                intersection = raw_intersection.unwrap();
            }

            // validate that the closest intersection is used
            let temp_x: f64;
            let temp_y: f64;
            if intersection.1 < 0f64 {
                if angle == 90f64 || angle == 270f64 {
                    temp_y = intersection.0;
                    temp_x = 0f64;
                } else {
                    temp_x = intersection.0;
                    temp_y = (angle * (pi/180f64)).tan() * temp_x;
                }
            } else {
                if angle == 90f64 || angle == 270f64 {
                    temp_y = intersection.1;
                    temp_x = 0f64;
                } else {
                    temp_x = intersection.1;
                    temp_y = (angle * (pi/180f64)).tan() * temp_x;
                }
            }

            // check if the intersection is closer than the previous one
            if distance(temp_x, temp_y) < distance(end_x, end_y) || reflecting_angle == -1f64 {
                end_x = temp_x + laser_x_offset;
                end_y = temp_y + laser_y_offset;
            } else { continue; }

            // checks if intersects with the circle at the same point as the laser
            if round_to_2_decimals(end_x, 4) == round_to_2_decimals(laser_x_offset, 4) &&
                round_to_2_decimals(end_y, 4) == round_to_2_decimals(laser_y_offset,4) {
                continue;
            }

            // calculate the angle of the laser beam
            let mut tan_line_on_circle = ((end_y - circle.y) / (end_x - circle.x)).atan() * (180f64 / pi);

            // if negative rotate 90 degrees and than calculate reflecting angle
            if tan_line_on_circle < 0f64 {
                tan_line_on_circle = tan_line_on_circle + 90f64;
                reflecting_angle = -(angle - tan_line_on_circle + 180f64) + tan_line_on_circle;
                reflecting_angle = 180f64 - reflecting_angle;
            } else {
                reflecting_angle = -(angle - tan_line_on_circle + 180f64) + tan_line_on_circle;
            }

            // due to the dum reflection formula at 0 & 90 degrees the angle is not correct
            // so here we are manually setting it to 90 reflection if it intersects at 0 & 90 degrees
            if angle == 0f64 || angle == 180f64 {
                if cir_y > 0f64 {
                    reflecting_angle = 270f64;
                } else {
                    reflecting_angle = 90f64;
                }
            } else if angle == 90f64 || angle == 270f64 {
                if cir_x > 0f64 {
                    reflecting_angle = 180f64;
                } else {
                    reflecting_angle = 0f64;
                }
            }


            let window = window().unwrap();
            let message = format!("end_x: {}, x: {}", round_to_2_decimals(end_x, 4), round_to_2_decimals(laser_x_offset, 4));
            let alert = window.alert_with_message(message.as_str());


            filter_angle(&mut reflecting_angle);

            bounces = true;
        }

        let window = window().unwrap();
        let message = format!("2end_x: {}, 2x: {}", round_to_2_decimals(end_x, 4), round_to_2_decimals(laser_x_offset, 4));
        let alert = window.alert_with_message(message.as_str());
        
        // add the laser beam to the world
        world_ref.laser_beams.push(LaserBeam {
            x: laser_x_offset,
            y: laser_y_offset,
            angle,
            reflecting_angle,
            bounces,
            end_x,
            end_y,
        });
    }
    simulate_laser(ini_laser_offset_x, ini_laser_offset_y, ini_laser_angle, &mut world);

    
    for _ in 0..20 {
        // select last laser beam(LLB) in world
        let llb = world.laser_beams.last().unwrap();

        if llb.reflecting_angle == -1f64 {
            break;
        }
        
        simulate_laser(llb.end_x, llb.end_y, llb.reflecting_angle, &mut world);
    }

    let json_string = serde_json::to_string(&world).unwrap();
    return JsValue::from_str(&json_string);
}

/// first is the solution after adding and second is the solution after subtracting
fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let delta = b.powi(2) - 4.0*a*c;
    if delta < 0.0 {
        return None;
    }
    let x1 = (-b + delta.sqrt()) / (2.0*a);
    let x2 = (-b - delta.sqrt()) / (2.0*a);
    return Some((x1, x2));
}

/// please just don't pass 0 as rounding_to
fn round_to_2_decimals(num: f64, rounding_to: u32) -> f64 {
    return (num * (10_i32.pow(rounding_to)/10_i32) as f64).round() / (10_i32.pow(rounding_to)/10_i32) as f64;
}

fn distance(x: f64, y: f64) -> f64 {
    return (x.powi(2) + y.powi(2)).sqrt();
}