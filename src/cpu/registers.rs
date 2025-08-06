/**
* The different processor status bits that can be set in the processor status register.
*/
#[derive(Copy, Clone, Debug)]
pub enum ProcessorStatusBit {
    /**
     * Carry flag, set if the last operation resulted in a carry.
     */
    Carry = 0,

    /**
     * Zero flag, set if the last operation resulted in a zero.
     */
    Zero = 1,

    /**
     * Interrupt disable flag, set if interrupts are disabled.
     */
    InterruptDisable = 2,

    /**
     * Decimal mode flag, set if the CPU is in decimal mode.
     */
    DecimalMode = 3,

    /**
     * Break command flag, set if the last operation was a break command. // TODO Check what this means.
     */
    BreakCommand = 4,

    Unused = 5,

    /**
     * Overflow flag, set if the last operation resulted in an overflow.
     */
    Overflow = 6,

    /**
     * Negative flag, set if the last operation resulted in a negative value.
     */
    Negative = 7,
}

/**
 * Representation of a CPU's registers with methods for common manipulations.
 */
#[derive(Debug)]
pub struct Registers {
    /**
     * The 16-bit program counter.
     */
    pc: u16,

    /**
     * The 8-bit stack pointer.
     */
    sp: u8,

    /**
     * The 8-bit X index register.
     */
    x: u8,

    /**
     * The 8-bit Y index register.
     */
    y: u8,

    /**
     * The 8-bit accumulator.
     */
    a: u8,

    /**
     * The 8-bit processor status register.
     */
    p: u8,
}

impl Registers {
    /**
     * Get the current value of the program counter.
     */
    pub fn pc(&self) -> u16 {
        self.pc
    }

    /**
     * Increment the program counter by `count`.
     */
    pub fn incr_pr(&mut self, count: u16) -> () {
        self.pc = self.pc.wrapping_add(count) // TODO This may be meant to not wrap around.
    }

    /**
     * Decrement the program counter by `count`.
     */
    pub fn decr_pr(&mut self, count: u16) -> () {
        self.pc = self.pc.wrapping_sub(count) // TODO This may be meant to not wrap around.
    }

    /**
     * Set the current value of the program counter.
     */
    pub fn set_pc(&mut self, value: u16) -> () {
        self.pc = value
    }

    /**
     * Get the current value of the stack pointer.
     */
    pub fn sp(&self) -> u8 {
        self.sp
    }

    /**
     * Increment the stack pointer by `count`.
     */
    pub fn incr_sp(&mut self, count: u8) -> () {
        self.sp = self.sp.wrapping_add(count) // TODO This may be meant to not wrap around.
    }

    /**
     * Decrement the stack pointer by `count`.
     */
    pub fn decr_sp(&mut self, count: u8) -> () {
        self.sp = self.sp.wrapping_sub(count) // TODO This may be meant to not wrap around.
    }

    /**
     * Set the current value of the stack pointer.
     */
    pub fn set_sp(&mut self, value: u8) -> () {
        self.sp = value
    }

    /**
     * Get the current value of the X index register.
     */
    pub fn x(&self) -> u8 {
        self.x
    }

    /**
     * Set the current value of the X index register.
     */
    pub fn set_x(&mut self, value: u8) -> () {
        self.x = value
    }

    /**
     * Get the current value of the Y index register.
     */
    pub fn y(&self) -> u8 {
        self.y
    }

    /**
     * Set the current value of the Y index register.
     */
    pub fn set_y(&mut self, value: u8) -> () {
        self.y = value
    }

    /**
     * Get the current value of the accumulator.
     */
    pub fn a(&self) -> u8 {
        self.a
    }

    /**
     * Set the current value of the accumulator.
     */
    pub fn set_a(&mut self, value: u8) -> () {
        self.a = value
    }

    /**
     * Get the current value of the processor status register.
     */
    pub fn p(&self) -> u8 {
        self.p
    }

    /**
     * Set the current value of the processor status register.
     */
    pub fn set_p(&mut self, value: u8) -> () {
        self.p = value
    }

    /**
     * Get the nth bit of the processor status register.
     */
    pub fn p_bit(&self, ps_bit: ProcessorStatusBit) -> bool {
        let bit: u8 = ps_bit as u8;
        if bit > 7 {
            panic!("Index out of bounds for processor status register")
        }
        (self.p & (1 << bit)) != 0
    }

    /**
     * Set the nth bit of the processor status register.
     */
    pub fn set_p_bit(&mut self, ps_bit: ProcessorStatusBit, value: bool) -> () {
        let bit: u8 = ps_bit as u8;
        if bit > 7 {
            panic!("Index out of bounds for processor status register")
        }
        if value {
            self.p |= 1 << bit
        } else {
            self.p &= !(1 << bit)
        }
    }

    /**
     * Toggle the nth bit of the processor status register.
     */
    pub fn toggle_p_bit(&mut self, ps_bit: ProcessorStatusBit) -> () {
        let bit: u8 = ps_bit as u8;
        if bit > 7 {
            panic!("Index out of bounds for processor status register")
        }
        self.p ^= 1 << bit
    }

    /**
     * Create a new instance of Registers with all registers set to 0.
     */
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            sp: 0,
            x: 0,
            y: 0,
            a: 0,
            p: 0,
        }
    }
}
