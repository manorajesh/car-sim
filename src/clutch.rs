pub struct Clutch {
    pub engagement: f32, // 0.0 (disengaged) to 1.0 (fully engaged)
}

impl Clutch {
    pub fn new() -> Self {
        Clutch { engagement: 1.0 }
    }

    pub fn set_engagement(&mut self, amount: f32) {
        self.engagement = amount.clamp(0.0, 1.0);
    }
}
