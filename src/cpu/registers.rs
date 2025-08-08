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
#[derive(Clone, Debug)]
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

/**
* Test all the simple methods on the Registers struct.
*/
#[test]
fn test() {
    let mut registers: Registers = Registers::new();

    assert_eq!(
        registers.pc(),
        0,
        "Program counter should be initialized to 0"
    );
    assert_eq!(
        registers.sp(),
        0,
        "Stack pointer should be initialized to 0"
    );
    assert_eq!(registers.x(), 0, "X register should be initialized to 0");
    assert_eq!(registers.y(), 0, "Y register should be initialized to 0");
    assert_eq!(registers.a(), 0, "A register should be initialized to 0");
    assert_eq!(
        registers.p(),
        0,
        "Processor status register should be initialized to 0"
    );

    registers.set_pc(0x00FA);
    assert_eq!(registers.pc(), 0x00FA, "Program counter should be set");

    registers.set_sp(0xFF);
    assert_eq!(registers.sp(), 0xFF, "Stack pointer should be set");

    registers.set_a(0xF1);
    assert_eq!(registers.a(), 0xF1, "A register should be set");

    registers.set_x(0x12);
    assert_eq!(registers.x(), 0x12, "X register should be set");

    registers.set_y(0x34);
    assert_eq!(registers.y(), 0x34, "Y register should be set");

    registers.set_p(0b10101010);
    assert_eq!(
        registers.p(),
        0b10101010,
        "Processor status register should be set"
    );
}

/**
* Test all the program counter specific methods on the Registers struct.
*/
#[test]
fn test_pc() {
    let mut registers: Registers = Registers::new();

    assert_eq!(
        registers.pc(),
        0,
        "Program counter should be initialized to 0"
    );

    registers.set_pc(0x1234);
    assert_eq!(registers.pc(), 0x1234, "Program counter should be set");

    registers.incr_pr(0x0010);
    assert_eq!(
        registers.pc(),
        0x1244,
        "Program counter should be incremented"
    );

    registers.decr_pr(0x0002);
    assert_eq!(
        registers.pc(),
        0x1242,
        "Program counter should be decremented"
    );

    registers.set_pc(0xFFFF);
    assert_eq!(registers.pc(), 0xFFFF, "Program counter should be set");

    registers.incr_pr(0x0001); // TODO See #incr_pr
    assert_eq!(
        registers.pc(),
        0x0000,
        "Program counter should wrap around after incrementing"
    );

    registers.decr_pr(0x0005); // TODO See #decr_pr
    assert_eq!(
        registers.pc(),
        0xFFFB,
        "Program counter should wrap around after decrementing"
    );
}

/**
* Test all the stack pointer specific methods on the Registers struct.
*/
#[test]
fn test_sp() {
    let mut registers: Registers = Registers::new();

    assert_eq!(
        registers.sp(),
        0,
        "Stack pointer should be initialized to 0"
    );

    registers.set_sp(0x80);
    assert_eq!(registers.sp(), 0x80, "Stack pointer should be set");

    registers.incr_sp(0x10);
    assert_eq!(registers.sp(), 0x90, "Stack pointer should be incremented");

    registers.decr_sp(0x20);
    assert_eq!(registers.sp(), 0x70, "Stack pointer should be decremented");

    registers.set_sp(0x00);
    assert_eq!(registers.sp(), 0x00, "Stack pointer should be set");

    registers.decr_sp(0x02); // TODO See #decr_sp
    assert_eq!(
        registers.sp(),
        0xFE,
        "Stack pointer should wrap around after decrementing"
    );

    registers.incr_sp(0x05); // TODO See #incr_sp
    assert_eq!(
        registers.sp(),
        0x03,
        "Stack pointer should wrap around after incrementing"
    );
}

/**
* Test all the processor status register specific methods on the Registers struct.
*/
#[test]
fn test_p() {
    let mut registers: Registers = Registers::new();

    assert_eq!(
        registers.p(),
        0,
        "Processor status register should be initialized to 0"
    );

    registers.set_p(0b00001111);
    assert_eq!(
        registers.p(),
        0b00001111,
        "Processor status register should be set"
    );

    registers.set_p_bit(ProcessorStatusBit::Negative, true);
    assert!(registers.p_bit(ProcessorStatusBit::Negative));
    assert_eq!(registers.p(), 0b10001111, "Negative bit should be set");

    registers.set_p_bit(ProcessorStatusBit::Carry, false);
    assert!(!registers.p_bit(ProcessorStatusBit::Carry));
    assert_eq!(registers.p(), 0b10001110, "Carry bit should be cleared");

    registers.toggle_p_bit(ProcessorStatusBit::InterruptDisable);
    assert!(!registers.p_bit(ProcessorStatusBit::InterruptDisable));
    assert_eq!(
        registers.p(),
        0b10001010,
        "Interrupt disable bit should be toggled off"
    );

    registers.toggle_p_bit(ProcessorStatusBit::InterruptDisable);
    assert!(registers.p_bit(ProcessorStatusBit::InterruptDisable));
    assert_eq!(
        registers.p(),
        0b10001110,
        "Interrupt disable bit should be toggled on"
    );
}
