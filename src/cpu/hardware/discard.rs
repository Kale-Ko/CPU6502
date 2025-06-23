use super::{AddressableHardware, Hardware, HardwareError};

/**
* Represents a piece of addressable hardware that discards all input.
*/
#[derive(Debug)]
pub struct DiscardHardware {}

impl Hardware for DiscardHardware {
    fn get_name(&self) -> &str {
        "Discard Device"
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
