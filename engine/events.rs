//! Event system for engine and plugins

pub enum EngineEvent {
    FunctionCall,
    Error,
    // TODO: Add more events
}

pub fn register_event_handler(event: EngineEvent, handler: fn()) {
    // TODO: Register event handler
}
