pub struct DebugEnvironment {
    /// Line numbers where breakpoints are set
    breakpoints: Vec<usize>,
}

impl DebugEnvironment {
    pub fn new(breakpoints: Vec<usize>) -> Self {
        Self { breakpoints }
    }
}
