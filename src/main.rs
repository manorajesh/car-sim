use macroquad::prelude::*;
use gilrs::{ Axis, Button, Event, EventType, Gilrs };

#[derive(Debug)]
struct Engine {
    current_rpm: f64,
    target_rpm: f64,
    minimum_rpm_threshold: f64,
    throttle_position: f64,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            current_rpm: 800.0,
            target_rpm: 800.0,
            minimum_rpm_threshold: 300.0,
            throttle_position: 0.0,
        }
    }

    fn apply_throttle(&mut self, position: f64) {
        self.throttle_position = position.clamp(0.0, 1.0);
        // self.torque = self.base_torque + self.base_torque * self.throttle_position * 2.0;
    }

    fn calculate_rpm(&mut self) {
        let target_rpm = 800.0 + self.throttle_position * 8000.0;
        let rpm_difference = target_rpm - self.current_rpm;

        if rpm_difference > 0.0 {
            self.current_rpm += self.throttle_position * 10.0;
        } else if rpm_difference < 0.0 {
            self.current_rpm -= self.throttle_position * 10.0;
        }
    }
}

#[macroquad::main("Engine Simulator")]
async fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut engine = Engine::new();

    loop {
        // Examine new events
        if let Some(Event { id, event, time }) = gilrs.next_event() {
            match event {
                EventType::ButtonChanged(Button::RightTrigger2, value, _) => {
                    engine.apply_throttle(value as f64);
                }
                EventType::ButtonChanged(Button::LeftTrigger2, value, _) => {
                    // engine.clutch_engagement = 1.0 - (value as f64);
                }
                EventType::AxisChanged(Axis::RightStickY, value, _) => {}
                _ => {}
            }
        }

        engine.calculate_rpm();

        clear_background(BLACK);
        draw_text("Engine", 20.0, 20.0, 20.0, WHITE);
        // draw_text(&format!("Torque: {:.2}", engine.torque), 20.0, 40.0, 20.0, WHITE);
        draw_text(&format!("RPM: {:.2}", engine.current_rpm), 20.0, 60.0, 20.0, WHITE);
        // draw_text(&format!("Clutch: {:.2}", engine.clutch_engagement), 20.0, 80.0, 20.0, WHITE);
        // draw_text(&format!("External Load: {:.2}", engine.external_load), 20.0, 100.0, 20.0, WHITE);
        // draw_text(
        //     &format!("Engine Inertia: {:.2}", engine.engine_inertia),
        //     20.0,
        //     120.0,
        //     20.0,
        //     WHITE
        // );
        draw_text(&format!("Throttle: {:.2}", engine.throttle_position), 20.0, 140.0, 20.0, WHITE);
        if engine.current_rpm == 0.0 {
            draw_text("Engine has stalled!", 20.0, 50.0, 30.0, RED);
        }

        next_frame().await;
    }
}
