// GameBun opcode methods

fn reverse_bytes(an16: u16) -> u16 {
    (an16 & 0xFF00) >> 8 
    | (an16 & 0x00FF) << 8
}

// All opcode method names are in the format
// CMD_ARG1_ARG2__CODE_IN_HEX()
// I.E. LD_BC_n16__01xxxx() is equivalent to the assembly `LD BC n16` and the binary `01xxxx` where
// xxxx is the two-byte argument (n16)
// at_r16/n16 is equivalent to `[r16]/[n16]`, aka memory address
impl CPU {
    // No operation
    fn NOP__00(&mut self) {
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load register BC with immediate n16
    fn LD_BC_n16__01xxxx(&mut self) {
        let n16: u16 = (self.ir >> 8) & 0xFFFF;
        self.registers.set_BC(reverse_bytes(n16));
        self.pc += 3;
        self.M_cycle(3);
    }

    // Load register A with byte at address in register BC
    fn LD_A_at_BC__02(&mut self) {
        self.registers.A = self.wram[self.registers.get_BC()];
        self.pc += 1;
        self.M_cycle(2);
    }

    // Increment the value in register BC by 1
    fn INC_BC__03(&mut self) {
        self.registers.set_BC(self.registers.get_BC() + 1);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Increment the value in register B by 1
    // set Z if the result is 0
    // reset N to 0
    // set H if overflow from bit 3
    fn INC_B__04(&mut self) {
        self.registers.B += 1;
        let mut flag = FlagsRegister::from(self.registers.F);
        flag.zero = (self.registers.B == 0);
        flag.subtract = 0;
        flag.half_carry = ((self.registers.B & 0x0F) == 0);
        self.registers.F = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Decrement the value in register B by 1
    // set Z if the result is 0
    // set N to 1
    // set H if overflow from bit 3
    fn DEC_B__05(&mut self) {
        self.registers.B -= 1;
        let mut flag = FlagsRegister::from(self.registers.F);
        flag.zero = (self.registers.B == 0);
        flag.subtract = true;
        flag.half_carry = ((self.registers.B & 0x0F) == 0xF);
        self.registers.F = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load register B with immediate n8
    fn LD_B_n8__06xx(&mut self) {
        let n8: u8 = ((self.ir >> 16) & 0xFF) as u8;
        self.registers.B = n8;
        self.pc += 2;
        self.M_cycle(2);
    }

    // Rotate left register A
    // set C to b7
    // reset all other registers
    fn RLCA__07(&mut self) {
        let mut flag = FlagsRegister::from(0x0000);
        flag.carry = (self.registers.A >> 7) & 0b1;
        self.registers.A <<= 1;
        self.registser.A |= flag.carry;
        self.registers.F = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load memory address a16 with register SP
    fn LD_at_a16_SP__08(&mut self) {
        let a16: u16 = reverse_bytes((self.ir & 0x00FFFF00) >> 8) as u16;
        self.wram[a16] = self.sp;
        self.pc += 3;
        self.M_cycle(5);
    }

    // Add value in register BC to register HL
    // reset N to 0
    // set H if overflow from bit 11
    // set C if overflow from bit 15
    fn ADD_HL_BC__09(&mut self) {
        let hl: u16 = self.registers.get_HL();
        self.registers.set_HL(hl + self.registers.get_BC());
        let mut flag = FlagsRegister::from(self.registers.F);
        flag.subtract = 0;
        flag.half_carry = (self.registers.get_HL() & 0x0FFF) < (hl & 0x0FFF);
        flag.carry = self.registesr.get_HL() < hl;
        self.registers.F = u8::from(flag);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Load register A with the byte in memory address stored in register BC
    fn LD_A_at_BC__0A(&mut self) {
        self.registers.A = self.wram[self.registers.get_BC()];
        self.pc += 1;
        self.M_cycle(2);
    }

    // Decrement value in register BC by 1
    fn DEC_BC__0B(&mut self) {
        self.registers.B -= 1;
        self.pc += 1;
        self.M_cycle(2);
    }

    // Incrememnt value in register C by 1
    // set Z flag if resultant value is 0
    // reset N flag
    // set H flag if overflow from bit 3
    fn INC_C__0C(&mut self) {
        self.registers.C += 1;
        let mut flag = FlagsRegister::from(self.registers.F);
        flag.zero = (self.registers.C == 0);
        flag.subtract = 0;
        flag.half_carry = ((self.registers.C & 0x0F) == 0);
        self.registers.F = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Decrement the value in register C by 1
    // set Z if the result is 0
    // set N to 1
    // set H if overflow from bit 3
    fn DEC_C__0D(&mut self) {
        self.registers.C -= 1;
        let mut flag = FlagsRegister::from(self.registers.F);
        flag.zero = (self.registers.C == 0);
        flag.subtract = true;
        flag.half_carry = ((self.registers.C & 0x0F) == 0xF);
        self.registers.F = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load register C with immediate value n8
    fn LD_C_n8__0Exx(&mut self) {
        let n8: u8 = ((self.ir >> 16) & 0xFF) as u8;
        self.registers.C = n8;
        self.pc += 2;
        self.M_cycle(2);
    }

    // Rotate register A right
    // set C to b0
    // reset all other flags
    fn RRCA__0F(&mut self) {
        let mut flag = FlagsRegister::from(0x0000);
        flag.carry = self.registers.A & 0b1;
        self.registers.A >>= 1;
        self.registser.A |= ((flag.carry << 7) as u8);
        self.registers.F = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }
}