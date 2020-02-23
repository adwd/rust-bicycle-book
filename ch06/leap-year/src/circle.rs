#[derive(Debug)]
pub struct Circle {
    pub radius: u32,
}

impl Circle {
    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    pub fn small_circle() -> Self {
        Self { radius: 1 }
    }
}
