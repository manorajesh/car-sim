mod engine;
mod clutch;

use macroquad::prelude::*;
use gilrs::{ Gilrs, Event, Axis, Button };
use engine::Engine;
use clutch::Clutch;

#[macroquad::main("Car Simulator")]
async fn main() {
    let mut engine = Engine::new(7000.0);
    let mut clutch = Clutch::new();
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("New event: {:?}", event);

            match event {
                gilrs::EventType::ButtonPressed(Button::South, _) => {
                    println!("Button South is pressed (XBox - A, PS - X)");
                }
                gilrs::EventType::ButtonChanged(Button::RightTrigger2, value, _) => {
                    engine.increase_rpm((value as f32) * 10.0);
                }
                gilrs::EventType::ButtonChanged(Button::LeftTrigger2, value, _) => {
                    clutch.set_engagement(1.0 - (value as f32));
                }
                gilrs::EventType::AxisChanged(axis, value, _) => {
                    if let Axis::RightStickY = axis {
                        engine.decrease_rpm((value as f32) * 20.0); // Brake
                    }
                }
                _ => {}
            }
        }

        // Update engine RPM based on clutch state
        let load = calculate_load(); // Implement this function to simulate load on the engine
        engine.update_rpm_with_clutch(&clutch, 0.0, 0.0, load); // Adjust throttle and brake as needed

        // Clear the screen
        clear_background(RED);

        // Render RPM
        draw_text(&format!("Engine RPM: {}", engine.rpm), 20.0, 20.0, 30.0, WHITE);

        // Render other simulation details...

        // Wait for the next frame
        next_frame().await;
    }
}

// Placeholder functions for load calculation
fn calculate_load() -> f32 {
    // Simplified load calculation
    // This should be replaced with a more realistic calculation based on your simulation needs
    0.5
}
