use super::Hardware;

/**
* Represents a piece of hardware that does absolutely nothing and has no device implementations.
*/
#[derive(Debug)]
pub struct EmptyHardware {}

impl Hardware for EmptyHardware {
    fn get_name(&self) -> &str {
        "<Empty>"
    }
}
