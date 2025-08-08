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

/**
* Test the EmptyHardware implementation.
*/
#[test]
fn test() {
    let mut empty: EmptyHardware = EmptyHardware::new();

    assert!(
        empty.as_addressable().is_none(),
        "EmptyHardware should not be addressable"
    );
    assert!(
        empty.as_addressable_mut().is_none(),
        "EmptyHardware should not be addressable"
    );
    assert!(
        empty.as_sized_addressable().is_none(),
        "EmptyHardware should not be sized"
    );
    assert!(
        empty.as_sized_addressable_mut().is_none(),
        "EmptyHardware should not be sized"
    );
}
