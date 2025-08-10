use super::{AddressableHardware, Hardware, HardwareError};

/**
 * Represents a piece of addressable hardware that discards all input.
 */
#[derive(Debug)]
pub struct DiscardHardware {}

impl DiscardHardware {
    /**
     * Create a new DiscardHardware.
     */
    pub fn new() -> DiscardHardware {
        DiscardHardware {}
    }
}

impl Hardware for DiscardHardware {
    fn name(&self) -> &str {
        "Discard Device"
    }

    fn as_addressable(&self) -> Option<&dyn AddressableHardware> {
        Some(self)
    }

    fn as_addressable_mut(&mut self) -> Option<&mut dyn AddressableHardware> {
        Some(self)
    }
}

impl AddressableHardware for DiscardHardware {
    fn read(&self, _address: u16) -> Result<u8, HardwareError> {
        Ok(0)
    }

    fn write(&mut self, _address: u16, _value: u8) -> Result<(), HardwareError> {
        Ok(())
    }
}

/**
 * Test the DiscardHardware implementation.
 */
#[test]
fn test() {
    let mut discard: DiscardHardware = DiscardHardware::new();

    assert!(
        discard.as_addressable().is_some(),
        "DiscardHardware should be addressable"
    );
    assert!(
        discard.as_addressable_mut().is_some(),
        "DiscardHardware should be addressable"
    );
    assert!(
        discard.as_sized_addressable().is_none(),
        "DiscardHardware should not be sized"
    );
    assert!(
        discard.as_sized_addressable_mut().is_none(),
        "DiscardHardware should not be sized"
    );

    assert_eq!(
        discard.read(0x0000).expect("DiscardHardware should always be readable"),
        0,
        "DiscardHardware should always read 0"
    );
    assert_eq!(
        discard.read(0xF035).expect("DiscardHardware should always be readable"),
        0,
        "DiscardHardware should always read 0"
    );

    discard.write(0x0F6A, 0x58).expect("DiscardHardware should always be writable");
    assert_eq!(
        discard.read(0x0F6A).expect("DiscardHardware should always be readable"),
        0,
        "DiscardHardware should always read 0 even after write"
    );
}
