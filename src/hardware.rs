pub mod empty;
pub mod discard;

/**
 * A hardware error that may or may not be fatal.
 */
#[derive(Debug)]
pub struct HardwareError {
    /**
     * If this error should cause the system to segfault.
     */
    fatal: bool,

    /**
     * A message to describe the error to a user debugging.
     */
    message: String // TODO signal other than segfault?
}

impl HardwareError {
    /**
     * Get if this error is fatal.
     */
    pub fn fatal(&self) -> bool {
        self.fatal
    }

    /**
     * Get the message describing this error.
     */
    pub fn message(&self) -> &str {
        &self.message
    }

    /**
     * Create a new hardware error with a given fatality and message.
     */
    pub fn new(fatal: bool, message: String) -> HardwareError {
        HardwareError { fatal, message }
    }
}

/**
 * Represents a piece of hardware in the system.
 */
pub trait Hardware {
    /**
     * Get the name of this piece of hardware.
     */
    fn name(&self) -> &str;

    fn as_addressable(&self) -> Option<&dyn AddressableHardware> {
        None
    }

    fn as_addressable_mut(&mut self) -> Option<&mut dyn AddressableHardware> {
        None
    }

    fn as_sized_addressable(&self) -> Option<&dyn SizedAddressableHardware> {
        None
    }

    fn as_sized_addressable_mut(&mut self) -> Option<&mut dyn SizedAddressableHardware> {
        None
    }
}

/**
 * Represents a piece of hardware that can be addressed on the bus.
 */
pub trait AddressableHardware: Hardware {
    /**
     * Reads a byte from the specified local address.
     *
     * Local addresses start at 0 rather than the hardware offset.
     */
    fn read(&self, address: u16) -> Result<u8, HardwareError>;

    /**
     * Writes a byte to the specified local address.
     *
     * Local addresses start at 0 rather than the hardware offset.
     */
    fn write(&mut self, address: u16, value: u8) -> Result<(), HardwareError>;
}

/**
 * Represents a piece of hardware that can be addressed on the bus and is a fixed size.
 */
pub trait SizedAddressableHardware: AddressableHardware {
    /**
     * Get the size of this piece of hardware in pages.
     */
    fn size(&self) -> u8;
}
