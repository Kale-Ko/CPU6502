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
fn test_registers() {
    let mut registers = Registers::new();

    assert_eq!(registers.pc(), 0);
    assert_eq!(registers.sp(), 0);
    assert_eq!(registers.x(), 0);
    assert_eq!(registers.y(), 0);
    assert_eq!(registers.a(), 0);
    assert_eq!(registers.p(), 0);

    registers.set_pc(0x00FA);
    assert_eq!(registers.pc(), 0x00FA);

    registers.set_sp(0xFF);
    assert_eq!(registers.sp(), 0xFF);

    registers.set_a(0xF1);
    assert_eq!(registers.a(), 0xF1);

    registers.set_x(0x12);
    assert_eq!(registers.x(), 0x12);

    registers.set_y(0x34);
    assert_eq!(registers.y(), 0x34);

    registers.set_p(0b10101010);
    assert_eq!(registers.p(), 0b10101010);
}

/**
* Test all the program counter specific methods on the Registers struct.
*/
#[test]
fn test_pc() {
    let mut registers = Registers::new();

    assert_eq!(registers.pc(), 0);

    registers.set_pc(0x1234);
    assert_eq!(registers.pc(), 0x1234);

    registers.incr_pr(0x0010);
    assert_eq!(registers.pc(), 0x1244);

    registers.decr_pr(0x0002);
    assert_eq!(registers.pc(), 0x1242);

    registers.set_pc(0xFFFF);
    assert_eq!(registers.pc(), 0xFFFF);

    registers.incr_pr(0x0001); // TODO See #incr_pr
    assert_eq!(registers.pc(), 0x0000);

    registers.decr_pr(0x0005); // TODO See #decr_pr
    assert_eq!(registers.pc(), 0xFFFB);
}

/**
* Test all the stack pointer specific methods on the Registers struct.
*/
#[test]
fn test_sp() {
    let mut registers = Registers::new();

    assert_eq!(registers.sp(), 0);

    registers.set_sp(0x80);
    assert_eq!(registers.sp(), 0x80);

    registers.incr_sp(0x10);
    assert_eq!(registers.sp(), 0x90);

    registers.decr_sp(0x20);
    assert_eq!(registers.sp(), 0x70);

    registers.set_sp(0x00);
    assert_eq!(registers.sp(), 0x00);

    registers.decr_sp(0x02); // TODO See #decr_sp
    assert_eq!(registers.sp(), 0xFE);

    registers.incr_sp(0x05); // TODO See #incr_sp
    assert_eq!(registers.sp(), 0x03);
}

/**
* Test all the processor status register specific methods on the Registers struct.
*/
#[test]
fn test_p() {
    let mut registers = Registers::new();

    assert_eq!(registers.p(), 0);

    registers.set_p(0b00001111);
    assert_eq!(registers.p(), 0b00001111);

    registers.set_p_bit(ProcessorStatusBit::Negative, true);
    assert!(registers.p_bit(ProcessorStatusBit::Negative));
    assert_eq!(registers.p(), 0b10001111);

    registers.set_p_bit(ProcessorStatusBit::Carry, false);
    assert!(!registers.p_bit(ProcessorStatusBit::Carry));
    assert_eq!(registers.p(), 0b10001110);

    registers.toggle_p_bit(ProcessorStatusBit::InterruptDisable);
    assert!(!registers.p_bit(ProcessorStatusBit::InterruptDisable));
    assert_eq!(registers.p(), 0b10001010);

    registers.toggle_p_bit(ProcessorStatusBit::InterruptDisable);
    assert!(registers.p_bit(ProcessorStatusBit::InterruptDisable));
    assert_eq!(registers.p(), 0b10001110);
}
