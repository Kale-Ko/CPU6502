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
    message: String,
    // TODO signal other than segfault?
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
    pub fn message(&self) -> &String {
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
    fn get_name(&self) -> &str;
}

/**
* Represents a piece of hardware that can be addressed on the bus.
*/
pub trait AddressableHardware: Hardware {
    /**
     * Gets the local address range of this piece of hardware.
     *
     * A local address is not the actual address a program would use to access this hardware, it is local to the hardware itself (should generally start at 0).
     */
    fn get_range(&self) -> (u16, u16);

    /**
     * Reads a byte from the specified local address.
     *
     * See `get_range()` for a note on local addresses.
     */
    fn read(&self, address: u16) -> Result<u8, HardwareError>;

    /**
     * Writes a byte to the specified local address.
     *
     * See `get_range()` for a note on local addresses.
     */
    fn write(&mut self, address: u16, value: u8) -> Result<(), HardwareError>;
}

/**
* Represents a piece of hardware that does absolutely nothing.
*/
#[derive(Debug)]
pub struct EmptyHardware {}

impl Hardware for EmptyHardware {
    fn get_name(&self) -> &str {
        "<Empty>"
    }
}
