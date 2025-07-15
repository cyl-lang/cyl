use crate::stdlib;

pub struct StdLibWrapper {
    #[allow(dead_code)]
    pub modules: stdlib::StdLib,
}

impl StdLibWrapper {
    pub fn new() -> Self {
        StdLibWrapper {
            modules: stdlib::StdLib::new(),
        }
    }
}
