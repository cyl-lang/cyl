//! CLI command definitions and dispatch

pub enum Command {
    Run,
    Build,
    Check,
    Plugins,
    Ast,
    Config,
}

// TODO: Implement command parsing and dispatch logic
