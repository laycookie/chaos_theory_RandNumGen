use wasm_bindgen::prelude::*;
use serde::{Serialize};
use web_sys::console;

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
    id: Option<i128>,
}

static mut WORLD: World = World {
    circles: Vec::new(),
    laser_beams: Vec::new(),
};

#[wasm_bindgen]
pub unsafe fn add_circle(x: f64, y: f64, radius: f64) {
    // find the biggest id in the WORLD
    let mut biggest_id: i128 = 0;
    for circle in WORLD.circles.iter() {
        if let Some(id) = circle.id {
            if id > biggest_id {
                biggest_id = id;
            }
        }
    }
    // add the circle to the WORLD
    WORLD.circles.push(Circle {
        x,
        y,
        radius,
        id: Some(biggest_id + 1),
    });
}

#[wasm_bindgen]
pub unsafe fn del_circle(id: i64) {
    // remove the circle from the WORLD
    WORLD.circles.retain(|circle| {
        if let Some(circle_id) = circle.id {
            circle_id != id.into()
        } else {
            true
        }
    });
}

#[wasm_bindgen]
pub unsafe fn manny_circle_set(circle_amount_x: i32, circle_amount_y: i32, spacing: f64, radius: f64, shift_x: f64) {
    // clear circles previously generated (circles without id)
    WORLD.circles.retain(|circle| {
        if let Some(_) = circle.id {
            true
        } else {
            false
        }
    });

    // generate circles in the WORLD
    for x in 0..circle_amount_x {
        for y in 0..circle_amount_y{
            if y % 2 == 0 {
                WORLD.circles.push(Circle {
                    x: ((x as f64) * spacing - (circle_amount_x - 1) as f64 * spacing / 2f64),
                    y: ((y as f64) * spacing - (circle_amount_y - 1) as f64 * spacing / 2f64),
                    radius: radius as f64,
                    id: None,});
            } else {
                WORLD.circles.push(Circle {
                    x: ((x as f64) * spacing - (circle_amount_x - 1) as f64 * spacing / 2f64+ shift_x),
                    y: ((y as f64) * spacing - (circle_amount_y - 1) as f64 * spacing / 2f64),
                    radius: radius as f64,
                    id: None,});
            }
        }
    }
}

#[wasm_bindgen]
pub unsafe fn simulate(ini_laser_offset_x: f64, ini_laser_offset_y: f64, ini_laser_angle: f64, reflection_amount: i64) -> JsValue {
    // clear laser beams previously generated
    WORLD.laser_beams.clear();

    fn simulate_laser(laser_x_offset: f64, laser_y_offset: f64, in_angle: f64, world_ref: &mut World) {
        let mut end_x: f64 = -1f64;
        let mut end_y: f64 = -1f64;
        let mut reflecting_angle: f64 = -1f64;
        let mut bounces: bool = false;

        let mut angle = in_angle;
        filter_angle(&mut angle);

        for circle in &mut world_ref.circles {
            // centering the laser by changing circles location
            let cir_x = circle.x - laser_x_offset;
            let cir_y = circle.y - laser_y_offset;

            // calculate the intersection of the laser and the circle
            let pi = std::f64::consts::PI;
            let a;
            let b;
            let c;

            if angle != 90f64 || angle != 270f64 {
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

            // find set y and x function
            fn set_cords(x: &mut f64, y: &mut f64, angle: f64, intersection: f64) {
                if angle == 90f64 || angle == 270f64 {
                    *y = intersection;
                    *x = 0f64;
                } else {
                    *x = intersection;
                    *y = (angle * (std::f64::consts::PI/180f64)).tan() * *x;
                }
            }

            // validate that the closest intersection is used
            let mut temp_x: f64 = 0f64;
            let mut temp_y: f64 = 0f64;
            // just trust me this if statement makes perfect sense
            if intersection.1 < 0f64 {
                set_cords(&mut temp_x, &mut temp_y, angle, intersection.0);

            } else {
                set_cords(&mut temp_x, &mut temp_y, angle, intersection.1);
            }

            // checks that the intersection is in the right direction
            if angle == 90f64 || angle == 270f64  {
                if angle == 90f64 {
                    if temp_y < 0f64 {
                        continue;
                    }
                } else {
                    if temp_y > 0f64 {
                        continue;
                    }
                }
            } else if angle > 90f64 && angle < 270f64 {
                if temp_x > 0f64 {
                    continue;
                }
            } else {
                if temp_x < 0f64 {
                    continue;
                }
            }
            temp_y += laser_y_offset;
            temp_x += laser_x_offset;


            // checks if intersects with the circle at the same point as the laser
            if round_to_2_decimals(temp_x, 4) == round_to_2_decimals(laser_x_offset, 4) &&
                round_to_2_decimals(temp_y, 4) == round_to_2_decimals(laser_y_offset,4) {
                continue;
            }

            // check if the intersection is closer than the previous one
            if distance(temp_x - laser_x_offset, temp_y - laser_y_offset)
                < distance(end_x - laser_x_offset, end_y - laser_y_offset)
                || reflecting_angle == -1f64 {
                end_x = temp_x;
                end_y = temp_y;
            } else { continue; }

            // calculate the angle of the laser beam
            let tan_line_on_circle = ((end_y - circle.y) / (end_x - circle.x)).atan() * (180f64 / pi);
            reflecting_angle = -(angle - tan_line_on_circle + 180f64) + tan_line_on_circle;

            filter_angle(&mut reflecting_angle);

            // due to the dum reflection formula if intersection at tangent line angle is same as laser
            // angle reflection will be calculated incorrectly so we set it manually
            if angle == reflecting_angle {
                if cir_y > 0f64 {
                    if angle < 180f64 {
                        reflecting_angle = angle - 90f64;
                    } else {
                        reflecting_angle = angle + 90f64;
                    }
                } else if cir_y < 0f64 {
                    if angle < 180f64 {
                        reflecting_angle = angle + 90f64;
                    } else {
                        reflecting_angle = angle - 90f64;
                    }
                }
                if cir_x > 0f64 {
                    if angle > 180f64 {
                        reflecting_angle = angle - 90f64;
                    } else {
                        reflecting_angle = angle + 90f64;
                    }
                } else if cir_x < 0f64 {
                    if angle > 180f64 {
                        reflecting_angle = angle + 90f64;
                    } else {
                        reflecting_angle = angle - 90f64;
                    }
                }
            }

            filter_angle(&mut reflecting_angle);

            bounces = true;


        }
        
        // add the laser beam to the WORLD
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
    simulate_laser(ini_laser_offset_x, ini_laser_offset_y, ini_laser_angle, &mut WORLD);

    
    for _ in 0..reflection_amount {
        // select last laser beam(LLB) in WORLD
        let llb = WORLD.laser_beams.last().unwrap();

        if llb.reflecting_angle == -1f64 {
            break;
        }
        
        simulate_laser(llb.end_x, llb.end_y, llb.reflecting_angle, &mut WORLD);
    }

    let json_string = serde_json::to_string(&WORLD).unwrap();
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

/// make sure the angle is always positive and within 360 degrees
fn filter_angle(angle:&mut f64) {
    while *angle < 0f64 {
        *angle += 360f64;
    }
    while *angle >= 360f64 {
        *angle -= 360f64;
    }
}

fn distance(x: f64, y: f64) -> f64 {
    return (x.powi(2) + y.powi(2)).sqrt();
}