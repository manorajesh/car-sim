use crate::clutch::Clutch;

pub struct Engine {
    pub rpm: f32,
    pub throttle: f32,
    max_rpm: f32,
}

impl Engine {
    pub fn new(max_rpm: f32) -> Self {
        Engine {
            rpm: 0.0,
            throttle: 0.0,
            max_rpm,
        }
    }

    pub fn increase_rpm(&mut self, amount: f32) {
        self.rpm = (self.rpm + amount).min(self.max_rpm);
    }

    pub fn decrease_rpm(&mut self, amount: f32) {
        self.rpm = (self.rpm - amount).max(0.0);
    }

    pub fn update_rpm_with_clutch(
        &mut self,
        clutch: &Clutch,
        throttle: f32,
        brake: f32,
        load: f32
    ) {
        if clutch.engagement > 0.0 {
            let clutch_factor = clutch.engagement;
            let throttle_effect = throttle * clutch_factor * 10.0;
            let brake_effect = brake * clutch_factor * 20.0;
            self.rpm = (self.rpm + throttle_effect - brake_effect - load * clutch_factor).clamp(
                0.0,
                self.max_rpm
            );
        }
    }
}
