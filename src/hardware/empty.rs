use super::Hardware;

/**
* Represents a piece of hardware that does absolutely nothing and has no device implementations.
*/
#[derive(Debug)]
pub struct EmptyHardware {}

impl EmptyHardware {
    /**
     * Create a new EmptyHardware.
     */
    pub fn new() -> EmptyHardware {
        EmptyHardware {}
    }
}

impl Hardware for EmptyHardware {
    fn name(&self) -> &str {
        "<Empty>"
    }
}
