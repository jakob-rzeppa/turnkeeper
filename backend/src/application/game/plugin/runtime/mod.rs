use crate::application::game::plugin::runtime::{memory::MemoryManager};

mod memory;
mod execute;

struct RuntimeEnvironment {
    memory_manager: MemoryManager,
}

impl RuntimeEnvironment {
    pub fn new() -> Self {
        Self {
            memory_manager: MemoryManager::default(),
        }
    }
}