pub enum DebugCommand {
    StepInto,
    StepOver,
    Continue,
    SetBreakpoint(usize),   // Line number
    ClearBreakpoint(usize), // Line number
}
