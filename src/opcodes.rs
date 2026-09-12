// GameBun Opcodes
use crate::processor::CPU;

pub mod opcodes;

fn reverse_bytes(an16: u16) -> u16 {
    (an16 & 0xFF00) >> 8 
    | (an16 & 0x00FF) << 8
}

// All opcode method names are in the format
// CMD_ARG1_ARG2__CODE_IN_HEX()
// I.E. LD_BC_n16__01xxxx() is equivalent to the assembly `LD BC n16` and the hexadecimal `01xxxx` 
// where xxxx is the two-byte argument (n16)
// at_r16/n16 is equivalent to `[r16]/[n16]`, aka memory address

// 00-0F
impl CPU {
    // No operation
    fn nop__00(&mut self) {
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load register BC with immediate n16
    fn ld_bc_n16__01xxxx(&mut self) {
        let n16: u16 = ((self.ir >> 8) & 0xFFFF) as u16;
        self.registers.set_bc(reverse_bytes(n16));
        self.pc += 3;
        self.M_cycle(3);
    }

    // Load byte at memory address stored in BC with register A
    fn ld_at_bc_a__02(&mut self) {
        self.ram[self.registers.get_bc()] = self.registers.a;
        self.pc += 1;
        self.M_cycle(2);
    }

    // Increment the value in register BC by 1
    fn inc_bc__03(&mut self) {
        self.registers.set_bc(self.registers.get_bc() + 1);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Increment the value in register B by 1
    // set Z/zero if the result is 0
    // reset N/subtract to 0
    // set H/half-carry if overflow from bit 3
    fn inc_b__04(&mut self) {
        self.registers.b += 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.b == 0);
        flag.subtract = 0;
        flag.half_carry = ((self.registers.b & 0x0F) == 0);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Decrement the value in register B by 1
    // set Z/zero if the result is 0
    // set N/subtract to 1
    // set H/half-carry if overflow from bit 3
    fn dec_b__05(&mut self) {
        self.registers.b -= 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.b == 0);
        flag.subtract = true;
        flag.half_carry = ((self.registers.b & 0x0F) == 0xF);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load register B with immediate n8
    fn ld_b_n8__06xx(&mut self) {
        let n8: u8 = ((self.ir >> 16) & 0xFF) as u8;
        self.registers.b = n8;
        self.pc += 2;
        self.M_cycle(2);
    }

    // Rotate left register A
    // set C to b7
    // reset all other registers
    fn rlca__07(&mut self) {
        let mut flag = FlagsRegister::from(0x0000);
        flag.carry = (self.registers.a >> 7) & 0b1;
        self.registers.a <<= 1;
        self.registser.A |= flag.carry;
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load memory address a16 with register SP
    fn ld_at_a16_sp__08(&mut self) {
        let a16: u16 = reverse_bytes(((self.ir & 0x00FFFF00) >> 8) as u16);
        self.ram[a16] = self.sp;
        self.pc += 3;
        self.M_cycle(5);
    }

    // Add value in register BC to register HL
    // reset N/subtract to 0
    // set H/half-carry if overflow from bit 11
    // set C if overflow from bit 15
    fn add_hl_bc__09(&mut self) {
        let hl: u16 = self.registers.get_hl();
        self.registers.set_hl(hl + self.registers.get_bc());
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.subtract = 0;
        flag.half_carry = (self.registers.get_hl() & 0x0FFF) < (hl & 0x0FFF);
        flag.carry = self.registers.get_hl() < hl;
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Load register A with the byte in memory address stored in register BC
    fn ld_a_at_bc__0a(&mut self) {
        self.registers.a = self.ram[self.registers.get_bc()];
        self.pc += 1;
        self.M_cycle(2);
    }

    // Decrement value in register BC by 1
    fn dec_bc__0b(&mut self) {
        self.registers.set_bc(self.registers.get_bc() - 1);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Incrememnt value in register C by 1
    // set Z/zero flag if resultant value is 0
    // reset N/subtract flag
    // set H/half-carry flag if overflow from bit 3
    fn inc_c__0c(&mut self) {
        self.registers.c += 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.c == 0);
        flag.subtract = 0;
        flag.half_carry = ((self.registers.c & 0x0F) == 0);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Decrement the value in register C by 1
    // set Z/zero if the result is 0
    // set N/subtract to 1
    // set H/half-carry if overflow from bit 3
    fn dec_c__0d(&mut self) {
        self.registers.c -= 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.c == 0);
        flag.subtract = true;
        flag.half_carry = ((self.registers.c & 0x0F) == 0xF);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load register C with immediate value n8
    fn ld_c_n8__0exx(&mut self) {
        let n8: u8 = ((self.ir >> 16) & 0xFF) as u8;
        self.registers.c = n8;
        self.pc += 2;
        self.M_cycle(2);
    }

    // Rotate register A right
    // set C to b0
    // reset all other flags
    fn rrca__0f(&mut self) {
        let mut flag = FlagsRegister::from(0x0000);
        flag.carry = self.registers.a & 0b1;
        self.registers.a >>= 1;
        self.registser.A |= ((flag.carry << 7) as u8);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }
}




// 10-1F
impl CPU {
    // STOP -- go into low power mode
    // This instruction is complicated and isn't actually used, so we're just gonna treat it as NOP
    fn stop__10(&mut self) {
        self.pc += 2;
        self.M_cycle(2);
    }

    // Load register DE with immediate n16
    fn ld_de_n16__11xxxx(&mut self) {
        let n16: u16 = ((self.ir >> 8) & 0xFFFF) as u16;
        self.registers.set_de(reverse_bytes(n16));
        self.pc += 3;
        self.M_cycle(3);
    }

    // Load byte at memory address stored in DE with register A
    fn ld_at_de_a__12(&mut self) {
        self.ram[self.registers.get_de()] = self.registers.a;
        self.pc += 1;
        self.M_cycle(2);
    }

    // Increment the value in register DE by 1
    fn inc_de__13(&mut self) {
        self.registers.set_de(self.registers.get_de() + 1);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Increment the value in register D by 1
    // set Z/zero if the result is 0
    // reset N/subtract to 0
    // set H/half-carry if overflow from bit 3
    fn inc_d__14(&mut self) {
        self.registers.d += 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.d == 0);
        flag.subtract = 0;
        flag.half_carry = ((self.registers.d & 0x0F) == 0);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Decrement the value in register D by 1
    // set Z/zero if the result is 0
    // set N/subtract to 1
    // set H/half-carry if overflow from bit 3
    fn dec_d__15(&mut self) {
        self.registers.d -= 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.d == 0);
        flag.subtract = true;
        flag.half_carry = ((self.registers.d & 0x0F) == 0xF);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load register D with immediate n8
    fn ld_d_n8__16xx(&mut self) {
        let n8: u8 = ((self.ir >> 16) & 0xFF) as u8;
        self.registers.d = n8;
        self.pc += 2;
        self.M_cycle(2);
    }

    // Rotate left register A through flag C
    // reset all other flags
    fn rla__17(&mut self) {
        let mut flag = FlagsRegister::from(self.registers.f);
        let c = flag.carry;
        flag.carry = (self.registers.a >> 7) & 0b1;
        self.registers.a <<= 1;
        self.registser.A |= c;
        flag.zero = false;
        flag.subtract = false;
        flag.half_carry = false;
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Jump by signed 8-bit offset e8
    fn jr_e8__18xx(&mut self) {
        let e8: u8 = ((self.ir >> 16) & 0xFF) as i8;
        self.pc += e8;
        self.M_cycle(3);
    }

    // Add value in register de to register HL
    // reset N/subtract to 0
    // set H/half-carry if overflow from bit 11
    // set C if overflow from bit 15
    fn add_hl_de__19(&mut self) {
        let hl: u16 = self.registers.get_hl();
        self.registers.set_hl(hl + self.registers.get_de());
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.subtract = 0;
        flag.half_carry = (self.registers.get_hl() & 0x0FFF) < (hl & 0x0FFF);
        flag.carry = self.registers.get_hl() < hl;
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Load register A with the byte in memory address stored in register DE
    fn ld_a_at_de__1a(&mut self) {
        self.registers.a = self.ram[self.registers.get_de()];
        self.pc += 1;
        self.M_cycle(2);
    }

    // Decrement value in register DE by 1
    fn dec_de__1b(&mut self) {
        self.registers.set_de(self.registers.get_de() - 1);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Incrememnt value in register E by 1
    // set Z/zero flag if resultant value is 0
    // reset N/subtract flag
    // set H/half-carry flag if overflow from bit 3
    fn inc_e__1c(&mut self) {
        self.registers.e += 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.e == 0);
        flag.subtract = 0;
        flag.half_carry = ((self.registers.e & 0x0F) == 0);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Decrement the value in register E by 1
    // set Z/zero if the result is 0
    // set N/subtract to 1
    // set H/half-carry if overflow from bit 3
    fn dec_e__1d(&mut self) {
        self.registers.e -= 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.e == 0);
        flag.subtract = true;
        flag.half_carry = ((self.registers.e & 0x0F) == 0xF);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load register E with immediate value n8
    fn ld_e_n8__1exx(&mut self) {
        let n8: u8 = ((self.ir >> 16) & 0xFF) as u8;
        self.registers.e = n8;
        self.pc += 2;
        self.M_cycle(2);
    }

    // Rotate right register A through flag C
    // reset all other flags
    fn rra__1f(&mut self) {
        let mut flag = FlagsRegister::from(self.registers.f);
        let c = flag.carry;
        flag.carry = (self.registers.a) & 0b1;
        self.registers.a >>= 1;
        self.registser.A |= (c << 7) as u8;
        flag.zero = false;
        flag.subtract = false;
        flag.half_carry = false;
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }
}




// 20-2F
impl CPU {
    // Jump if not zero to relative address e8
    fn jr_nz_e8__20(&mut self) {
        let flags = FlagsRegister::from(self.registers.f);
        if !(flags.zero) {
            let e8: u8 = ((self.ir >> 16) & 0xFF) as i8;
            self.pc += e8;
            self.M_cycle(3);
        } else {
            self.pc += 2;
            self.M_cycle(2);
        }
    }

    // Load register HL with immediate n16
    fn ld_hl_n16__21xxxx(&mut self) {
        let n16: u16 = ((self.ir >> 8) & 0xFFFF) as u16;
        self.registers.set_hl(reverse_bytes(n16));
        self.pc += 3;
        self.M_cycle(3);
    }

    // Load byte at memory address stored in HL with register A
    // then increment HL
    fn ld_at_hl_inc_a__22(&mut self) {
        self.ram[self.registers.get_hl()] = self.registers.a;
        self.registers.set_hl(self.registers.get_hl() + 1);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Increment the value in register HL by 1
    fn inc_hl__23(&mut self) {
        self.registers.set_hl(self.registers.get_hl() + 1);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Increment the value in register H by 1
    // set Z/zero if the result is 0
    // reset N/subtract
    // set H/half-carry if overflow from bit 3
    fn inc_h__24(&mut self) {
        self.registers.h += 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.h == 0);
        flag.subtract = 0;
        flag.half_carry = ((self.registers.h & 0x0F) == 0);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Decrement the value in register H by 1
    // set Z/zero if the result is 0
    // set N/subtract to 1
    // set H/half-carry if overflow from bit 3
    fn dec_h__25(&mut self) {
        self.registers.h -= 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.h == 0);
        flag.subtract = true;
        flag.half_carry = ((self.registers.h & 0x0F) == 0xF);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load register H with immediate n8
    fn ld_h_n8__26xx(&mut self) {
        let n8: u8 = ((self.ir >> 16) & 0xFF) as u8;
        self.registers.h = n8;
        self.pc += 2;
        self.M_cycle(2);
    }

    // Decimal Adjust Accumulator
    // Designed to be used after performing an arithmetic instruction (ADD, ADC, SUB, SBC) whose 
    // inputs were in Binary-Coded Decimal (BCD), adjusting the result to likewise be in BCD.
    fn daa__27(&mut self) {
        let mut flags = FlagsRegister::from(self.registers.f);
        if flags.subtract {
            let adjustment: u8 = 0;
            let adjustment = if flags.half_carry { adjustment + 0x06 } else { adjustment };
            let adjustment = if flags.carry { adjustment + 0x60 } else { adjustment };
            self.registers.a -= adjustment;
        } else {
            let adjustment: u8 = 0;
            let adjustment = if (flags.half_carry || (self.registers.a & 0x0F) > 0x09) { adjustment + 0x06 } else { adjustment };
            let adjustment = if (flags.carry || self.registers.a > 0x99 ) { adjustment + 0x60 } else { adjustment };
            self.registers.a += adjustment;
        }
        flags.zero = if self.registers.a == 0 { true } else { flags.zero };
        flags.half_carry = false;
        self.registers.f = u8::from(flags);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Jump if Z/zero by signed 8-bit offset e8 if zero
    fn jr_z_e8__28xx(&mut self) {
        let flags = FlagsRegister::from(self.registers.f);
        if flags.zero {
            let e8: u8 = ((self.ir >> 16) & 0xFF) as i8;
            self.pc += e8;
            self.M_cycle(3);
        } else {
            self.pc += 2;
            self.M_cycle(2);
        }
    }

    // Add value in register HL to register HL
    // reset N/subtract to 0
    // set H/half-carry if overflow from bit 11
    // set C if overflow from bit 15
    fn add_hl_hl__29(&mut self) {
        let hl: u16 = self.registers.get_hl();
        self.registers.set_hl(hl + self.registers.get_hl());
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.subtract = 0;
        flag.half_carry = (self.registers.get_hl() & 0x0FFF) < (hl & 0x0FFF);
        flag.carry = self.registers.get_hl() < hl;
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Load register A with the byte in memory address stored in register HL
    // Then increment value in HL
    fn ld_a_at_hl__2a(&mut self) {
        self.registers.a = self.ram[self.registers.get_hl()];
        self.registers.set_hl(self.registers.get_hl() + 1);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Decrement value in register HL by 1
    fn dec_hl__2b(&mut self) {
        self.registers.set_hl(self.registers.get_hl() - 1);
        self.pc += 1;
        self.M_cycle(2);
    }

    // Incrememnt value in register L by 1
    // set Z/zero flag if resultant value is 0
    // reset N/subtract flag
    // set H/half-carry flag if overflow from bit 3
    fn inc_l__2c(&mut self) {
        self.registers.l += 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.l == 0);
        flag.subtract = 0;
        flag.half_carry = ((self.registers.l & 0x0F) == 0);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Decrement the value in register L by 1
    // set Z/zero if the result is 0
    // set N/subtract to 1
    // set H/half-carry if overflow from bit 3
    fn dec_l__2d(&mut self) {
        self.registers.l -= 1;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.zero = (self.registers.l == 0);
        flag.subtract = true;
        flag.half_carry = ((self.registers.l & 0x0F) == 0xF);
        self.registers.f = u8::from(flag);
        self.pc += 1;
        self.M_cycle(1);
    }

    // Load register L with immediate value n8
    fn ld_l_n8__2exx(&mut self) {
        let n8: u8 = ((self.ir >> 16) & 0xFF) as u8;
        self.registers.l = n8;
        self.pc += 2;
        self.M_cycle(2);
    }

    // ComPLement Accumulator (A = ~A) -- bitwise NOT on A
    // set N/subtract
    // set H/half-carry
    fn cpl__2f(&mut self) {
        self.registers.a = !self.registers.a;
        let mut flag = FlagsRegister::from(self.registers.f);
        flag.subtract = true;
        flag.half_carry = true;
        self.pc += 1;
        self.M_cycle(1);
    }
}




// Fetch-Decode-Execute
impl CPU {
    fn fetch_decode_execute(&mut self) {
        let opcode: u8 = (self.ir & 0xFF000000) >> 24 as u8;
        match opcode {
            // 00-0F
            0x00 => cpu.nop__00();
            0x01 => cpu.ld_bc_n16__01xxxx();
            0x02 => cpu.ld_at_bc_a__02();
            0x03 => cpu.inc_bc__03();
            0x04 => cpu.inc_b__04();
            0x05 => cpu.dec_b__05();
            0x06 => cpu.ld_b_n8__06xx();
            0x07 => cpu.rlca__07();
            0x08 => cpu.ld_at_a16_sp__08();
            0x09 => cpu.add_hl_bc__09();
            0x0A => cpu.ld_a_at_bc__0a();
            0x0B => cpu.dec_bc__0b();
            0x0C => cpu.inc_c__0c();
            0x0D => cpu.dec_c__0d();
            0x0E => cpu.ld_c_n8__0exx();
            0x0F => cpu.rrca__0f();
            // 10-1F
            0x10 => cpu.stop__10();
            0x11 => cpu.ld_de_n16__11xxxx();
            0x12 => cpu.ld_at_de_a__12();
            0x13 => cpu.inc_de__13();
            0x14 => cpu.inc_d__14();
            0x15 => cpu.dec_d__15()
            0x16 => cpu.ld_d_n8__16xx();
            0x17 => cpu.rla__17();
            0x18 => cpu.jr_e8__18xx();
            0x19 => cpu.add_hl_de__19();
            0x1A => cpu.ld_a_at_de__1a();
            0x1B => cpu.dec_de__1b();
            0x1C => cpu.inc_e__1c();
            0x1D => cpu.dec_e__1d();
            0x1E => cpu.ld_e_n8__1exx();
            0x1F => cpu.rra__1f();
            // 20-2F
            0x20 => cpu.jr_nz_e8__20();
            0x21 => cpu.ld_hl_n16__21xxxx();
            0x22 => cpu.ld_at_hl_inc_a__22();
            0x23 => cpu.inc_hl__23();
            0x24 => cpu.inc_h__24();
            0x25 => cpu.dec_h__25();
            0x26 => cpu.ld_h_n8__26xx();
            0x27 => cpu.daa__27();
            0x28 => cpu.jr_z_e8__28xx();
            0x29 => cpu.add_hl_hl__29();
            0x2A => cpu.ld_a_at_hl__2a();
            0x2B => cpu.dec_hl__2b();
            0x2C => cpu.inc_l__2c();
            0x2D => cpu.dec_l__2d();
            0x2E => cpu.ld_l_n8__2exx();
            0x2F => cpu.cpl__2f();
        }
    }
}