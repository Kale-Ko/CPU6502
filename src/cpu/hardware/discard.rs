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
