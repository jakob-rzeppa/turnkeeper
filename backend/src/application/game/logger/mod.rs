use sqlx::types::chrono::Local;

pub struct GameLogger {}

impl GameLogger {
    pub fn info(&self, message: &str) {
        println!("[GameLogger] {}: {}", Local::now(), message);
    }

    pub fn warn(&self, message: &str) {
        println!("[GameLogger] {} WARNING: {}", Local::now(), message);
    }

    pub fn error(&self, message: &str) {
        println!("[GameLogger] {} ERROR: {}", Local::now(), message);
    }

    pub fn debug(&self, message: &str) {
        println!("[GameLogger] {} DEBUG: {}", Local::now(), message);
    }
}
