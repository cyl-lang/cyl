//! Cyl Plugin system
//! Defines plugin API and event system

pub trait CylPlugin {
    fn name(&self) -> &str;
    // TODO: Plugin hooks (syntax, events, etc.)
}

// TODO: Plugin loader, event system
