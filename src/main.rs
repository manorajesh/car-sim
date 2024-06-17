use macroquad::prelude::*;
use gilrs::{ Axis, Button, Event, EventType, Gilrs };

struct Transmission {
    gear_ratios: Vec<f64>,
    current_gear: usize,
    clutch_engagement: f64,
    brake_engagement: f64,
    speed: f64,
}

impl Transmission {
    fn new() -> Transmission {
        Transmission {
            gear_ratios: vec![0.0, 3.0, 2.0, 1.5, 1.0, 0.8, 0.6],
            current_gear: 0,
            clutch_engagement: 1.0,
            brake_engagement: 0.0,
            speed: 0.0,
        }
    }

    fn shift_up(&mut self) {
        if self.current_gear < self.gear_ratios.len() - 1 {
            self.current_gear += 1;
        }
    }

    fn shift_down(&mut self) {
        if self.current_gear > 0 {
            self.current_gear -= 1;
        }
    }

    fn apply_clutch(&mut self, engagement: f64) {
        self.clutch_engagement = engagement.clamp(0.0, 1.0);
    }

    fn apply_brake(&mut self, engagement: f64) {
        self.brake_engagement = engagement.clamp(0.0, 1.0);
    }

    fn calculate_torque(&self, engine_rpm: f64) -> f64 {
        let gear_ratio = self.gear_ratios[self.current_gear];
        let torque =
            engine_rpm * gear_ratio * self.clutch_engagement * (1.0 - self.brake_engagement);
        torque
    }

    fn update_speed(&mut self, engine_rpm: f64) {
        let torque = self.calculate_torque(engine_rpm);
        let target_speed = torque * 0.1;
        let speed_difference = target_speed - self.speed;

        if speed_difference > 0.0 {
            self.speed += 0.1;
        } else if speed_difference < 0.0 {
            self.speed -= 0.1 + self.brake_engagement;
        }
    }

    fn calculate_load(&mut self, engine_rpm: f64) -> f64 {
        self.update_speed(engine_rpm);
        let gear_ratio = self.gear_ratios[self.current_gear];
        let load =
            engine_rpm * gear_ratio * self.clutch_engagement * (1.0 - self.brake_engagement) -
            self.speed * 10.0;
        load.max(0.0)
    }

    fn get_current_gear(&self) -> String {
        match self.current_gear {
            0 => String::from("N"),
            _ => self.current_gear.to_string(),
        }
    }
}

#[derive(Debug)]
struct Engine {
    current_rpm: f64,
    target_rpm: f64,
    minimum_rpm_threshold: f64,
    throttle_position: f64,
    engine_inertia: f64,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            current_rpm: 800.0,
            target_rpm: 800.0,
            minimum_rpm_threshold: 300.0,
            throttle_position: 0.0,
            engine_inertia: 0.0,
        }
    }

    fn apply_throttle(&mut self, position: f64) {
        self.throttle_position = position.clamp(0.0, 1.0);
    }

    fn update_rpm(&mut self, external_load: f64) {
        self.target_rpm = 800.0 + self.throttle_position * 8000.0;
        let rpm_difference = self.target_rpm - self.current_rpm;
        self.engine_inertia = 0.05 * self.current_rpm;

        if rpm_difference > 0.0 {
            self.current_rpm += self.engine_inertia + self.throttle_position * 10.0 - external_load;
        } else if rpm_difference < 0.0 {
            self.current_rpm -= self.engine_inertia + external_load;
        }

        // stall
        if self.current_rpm < self.minimum_rpm_threshold {
            self.current_rpm = 0.0;
        }
    }
}

#[macroquad::main("Engine Simulator")]
async fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut engine = Engine::new();
    let mut transmission = Transmission::new();

    loop {
        // Examine new events
        if let Some(Event { id: _, event, time: _ }) = gilrs.next_event() {
            match event {
                EventType::ButtonChanged(Button::RightTrigger2, value, _) => {
                    engine.apply_throttle(value as f64);
                }
                EventType::ButtonChanged(Button::LeftTrigger2, value, _) => {
                    transmission.apply_clutch(1.0 - (value as f64));
                }
                EventType::ButtonPressed(Button::West, _) => {
                    // 800.1 because then the rpm will fluctuate (see if rpm_diff statement in update_rpm)
                    engine.current_rpm = 800.1;
                }
                EventType::ButtonPressed(Button::South, _) => {
                    transmission.shift_up();
                }
                EventType::ButtonPressed(Button::East, _) => {
                    transmission.shift_down();
                }
                EventType::AxisChanged(Axis::RightStickY, value, _) => {
                    println!("RightStickY: {}", value);
                    transmission.apply_brake(value as f64);
                }
                _ => {}
            }
        }

        let external_load = transmission.calculate_load(engine.current_rpm);
        engine.update_rpm(external_load);

        clear_background(BLACK);
        draw_text("Engine", 20.0, 20.0, 20.0, WHITE);
        // draw_text(&format!("Torque: {:.2}", engine.torque), 20.0, 40.0, 20.0, WHITE);
        draw_text(&format!("RPM: {:.2}", engine.current_rpm), 20.0, 60.0, 20.0, WHITE);
        draw_text(
            &format!("Clutch: {:.2}", transmission.clutch_engagement),
            20.0,
            80.0,
            20.0,
            WHITE
        );
        draw_text(&format!("External Load: {:.2}", external_load), 20.0, 100.0, 20.0, WHITE);
        draw_text(
            &format!("Engine Inertia: {:.2}", engine.engine_inertia),
            20.0,
            120.0,
            20.0,
            WHITE
        );
        draw_text(&format!("Throttle: {:.2}", engine.throttle_position), 20.0, 140.0, 20.0, WHITE);
        draw_text(&format!("Target RPM: {:.2}", engine.target_rpm), 20.0, 160.0, 20.0, WHITE);
        draw_text(&format!("Gear: {}", transmission.get_current_gear()), 20.0, 180.0, 20.0, WHITE);
        draw_text(&format!("Speed: {:.2}", transmission.speed), 20.0, 200.0, 20.0, WHITE);
        draw_text(
            &format!("Brake: {:.2}", transmission.brake_engagement),
            20.0,
            220.0,
            20.0,
            WHITE
        );
        if engine.current_rpm == 0.0 {
            draw_text("Engine has stalled!", 20.0, 260.0, 30.0, RED);
        }

        // tachometer with needle
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let radius = 100.0;
        let needle_length = 80.0;
        let needle_angle = (engine.current_rpm / 8000.0) * 120.0;
        // semi circle
        draw_circle(center.x, center.y, radius, WHITE);
        draw_circle(center.x, center.y, radius - 5.0, BLACK);

        draw_line(
            center.x,
            center.y,
            center.x - ((needle_angle.to_radians().cos() * needle_length) as f32),
            center.y - ((needle_angle.to_radians().sin() * needle_length) as f32),
            5.0,
            RED
        );

        next_frame().await;
    }
}
