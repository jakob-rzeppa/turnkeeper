pub struct GameLogger {}

impl GameLogger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn info(&self, message: &str) {
        println!("[GameLogger] {}", message);
    }

    pub fn warn(&self, message: &str) {
        println!("[GameLogger] WARNING: {}", message);
    }

    pub fn error(&self, message: &str) {
        println!("[GameLogger] ERROR: {}", message);
    }

    pub fn debug(&self, message: &str) {
        println!("[GameLogger] DEBUG: {}", message);
    }
}
