use crate::{bus::Bus, nes_machine::cpu::Cpu};

impl Cpu {
    fn instr_lax(&mut self, value: u8) {
        self.set_negative(value);
        self.set_zero(value);
        self.a = value;
        self.x = value;
    }

    pub(super) fn instr_nop_imm(&mut self, bus: &mut Bus) -> usize {
        let _ = self.fetch_operand_imm(bus);
        2
    }

    pub(super) fn instr_nop_abs(&mut self, bus: &mut Bus) -> usize {
        let _ = self.fetch_operand_abs(bus);
        4
    }

    pub(super) fn instr_nop_absx(&mut self, bus: &mut Bus) -> usize {
        let mut cycles = 4;
        let addr = self.fetch_address_absx(bus);
        // Page boundary crossed
        if addr & 0xff < self.x as u16 {
            cycles += 1
        }
        cycles
    }

    pub(super) fn instr_nop_zpg(&mut self, bus: &mut Bus) -> usize {
        let _ = self.fetch_operand_zpg(bus);
        3
    }

    pub(super) fn instr_nop_zpgx(&mut self, bus: &mut Bus) -> usize {
        let _ = self.fetch_operand_zpgx(bus);
        4
    }

    pub(super) fn instr_lax_abs(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_abs(bus);
        self.instr_lax(value);
        4
    }
    pub(super) fn instr_lax_absy(&mut self, bus: &mut Bus) -> usize {
        let mut cycles = 4;
        let addr = self.fetch_address_absy(bus);
        // Page boundary crossed
        if addr & 0xff < self.y as u16 {
            cycles += 1
        }
        self.instr_lax(bus.read(addr));
        cycles
    }
    pub(super) fn instr_lax_xind(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_xind(bus);
        self.instr_lax(value);
        6
    }
    pub(super) fn instr_lax_indy(&mut self, bus: &mut Bus) -> usize {
        let mut cycles = 5;
        let addr = self.fetch_address_indy(bus);
        // Page boundary crossed
        if addr & 0xff < self.y as u16 {
            cycles += 1
        }
        self.instr_lax(bus.read(addr));
        cycles
    }
    pub(super) fn instr_lax_zpg(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_zpg(bus);
        self.instr_lax(value);
        3
    }
    pub(super) fn instr_lax_zpgy(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_zpgy(bus);
        self.instr_lax(value);
        4
    }

    pub(super) fn instr_sax_abs(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_abs(bus);
        bus.write(addr, self.a & self.x);
        4
    }
    pub(super) fn instr_sax_xind(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_xind(bus);
        bus.write(addr, self.a & self.x);
        6
    }
    pub(super) fn instr_sax_zpg(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpg(bus);
        bus.write(addr, self.a & self.x);
        3
    }
    pub(super) fn instr_sax_zpgy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpgy(bus);
        bus.write(addr, self.a & self.x);
        4
    }

    fn instr_dcp(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr).wrapping_sub(1);
        bus.write(addr, value);
        self.status.c = self.a >= value;
        self.status.z = self.a == value;
        self.set_negative(self.a.wrapping_sub(value));
    }
    pub(super) fn instr_dcp_abs(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_abs(bus);
        self.instr_dcp(bus, addr);
        6
    }
    pub(super) fn instr_dcp_absx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absx(bus);
        self.instr_dcp(bus, addr);
        7
    }
    pub(super) fn instr_dcp_absy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absy(bus);
        self.instr_dcp(bus, addr);
        7
    }
    pub(super) fn instr_dcp_xind(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_xind(bus);
        self.instr_dcp(bus, addr);
        8
    }
    pub(super) fn instr_dcp_indy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_indy(bus);
        self.instr_dcp(bus, addr);
        8
    }
    pub(super) fn instr_dcp_zpg(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpg(bus);
        self.instr_dcp(bus, addr);
        5
    }
    pub(super) fn instr_dcp_zpgx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_dcp(bus, addr);
        6
    }

    fn instr_isc(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr).wrapping_add(1);
        bus.write(addr, value);

        let carry = if self.status.c { 1 } else { 0 };
        let result = self.a.overflowing_add(!value).0;
        let result = result.overflowing_add(carry).0;
        self.status.c = result <= self.a;
        self.set_zero(result);
        self.set_negative(result);
        self.set_overflow(!value, result);
        self.a = result;
    }
    pub(super) fn instr_isc_abs(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_abs(bus);
        self.instr_isc(bus, addr);
        6
    }
    pub(super) fn instr_isc_absx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absx(bus);
        self.instr_isc(bus, addr);
        7
    }
    pub(super) fn instr_isc_absy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absy(bus);
        self.instr_isc(bus, addr);
        7
    }
    pub(super) fn instr_isc_xind(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_xind(bus);
        self.instr_isc(bus, addr);
        8
    }
    pub(super) fn instr_isc_indy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_indy(bus);
        self.instr_isc(bus, addr);
        8
    }
    pub(super) fn instr_isc_zpg(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpg(bus);
        self.instr_isc(bus, addr);
        5
    }
    pub(super) fn instr_isc_zpgx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_isc(bus, addr);
        6
    }

    fn instr_slo(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr);
        self.status.c = value & 0x80 != 0;
        let result = value << 1;
        bus.write(addr, result);
        self.a |= result;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }
    pub(super) fn instr_slo_abs(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_abs(bus);
        self.instr_slo(bus, addr);
        6
    }
    pub(super) fn instr_slo_absx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absx(bus);
        self.instr_slo(bus, addr);
        7
    }
    pub(super) fn instr_slo_absy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absy(bus);
        self.instr_slo(bus, addr);
        7
    }
    pub(super) fn instr_slo_xind(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_xind(bus);
        self.instr_slo(bus, addr);
        8
    }
    pub(super) fn instr_slo_indy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_indy(bus);
        self.instr_slo(bus, addr);
        8
    }
    pub(super) fn instr_slo_zpg(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpg(bus);
        self.instr_slo(bus, addr);
        5
    }
    pub(super) fn instr_slo_zpgx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_slo(bus, addr);
        6
    }

    fn instr_rla(&mut self, bus: &mut Bus, addr: u16) {
        let data = bus.read(addr);
        let carry = if self.status.c { 1 } else { 0 };
        let shifted = (data << 1) | carry;
        self.status.c = data & 0x80 != 0;
        bus.write(addr, shifted);
        let result = self.a & shifted;
        self.set_zero(result);
        self.set_negative(result);
        self.a = result;
    }
    pub(super) fn instr_rla_abs(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_abs(bus);
        self.instr_rla(bus, addr);
        6
    }
    pub(super) fn instr_rla_absx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absx(bus);
        self.instr_rla(bus, addr);
        7
    }
    pub(super) fn instr_rla_absy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absy(bus);
        self.instr_rla(bus, addr);
        7
    }
    pub(super) fn instr_rla_xind(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_xind(bus);
        self.instr_rla(bus, addr);
        8
    }
    pub(super) fn instr_rla_indy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_indy(bus);
        self.instr_rla(bus, addr);
        8
    }
    pub(super) fn instr_rla_zpg(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpg(bus);
        self.instr_rla(bus, addr);
        5
    }
    pub(super) fn instr_rla_zpgx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_rla(bus, addr);
        6
    }

    fn instr_rra(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr);
        let carry = if self.status.c { 0x80 } else { 0 };
        let shifted = (value >> 1) | carry;
        self.status.c = value & 0x01 != 0;
        bus.write(addr, shifted);

        let carry = if self.status.c { 1 } else { 0 };
        let (result, ov1) = self.a.overflowing_add(shifted); // add data
        let (result, ov2) = result.overflowing_add(carry); // add carry from previous operation
        self.status.c = ov1 || ov2;
        self.set_zero(result);
        self.set_negative(result);
        self.set_overflow(shifted, result);
        self.a = result;
    }
    pub(super) fn instr_rra_abs(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_abs(bus);
        self.instr_rra(bus, addr);
        6
    }
    pub(super) fn instr_rra_absx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absx(bus);
        self.instr_rra(bus, addr);
        7
    }
    pub(super) fn instr_rra_absy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absy(bus);
        self.instr_rra(bus, addr);
        7
    }
    pub(super) fn instr_rra_xind(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_xind(bus);
        self.instr_rra(bus, addr);
        8
    }
    pub(super) fn instr_rra_indy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_indy(bus);
        self.instr_rra(bus, addr);
        8
    }
    pub(super) fn instr_rra_zpg(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpg(bus);
        self.instr_rra(bus, addr);
        5
    }
    pub(super) fn instr_rra_zpgx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_rra(bus, addr);
        6
    }

    fn instr_sre(&mut self, bus: &mut Bus, addr: u16) {
        let data = bus.read(addr);
        let result = data >> 1;
        bus.write(addr, result);
        self.a ^= result;
        self.set_zero(self.a);
        self.set_negative(self.a);
        self.status.c = data & 0x01 != 0;
    }
    pub(super) fn instr_sre_abs(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_abs(bus);
        self.instr_sre(bus, addr);
        6
    }
    pub(super) fn instr_sre_absx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absx(bus);
        self.instr_sre(bus, addr);
        7
    }
    pub(super) fn instr_sre_absy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absy(bus);
        self.instr_sre(bus, addr);
        7
    }
    pub(super) fn instr_sre_xind(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_xind(bus);
        self.instr_sre(bus, addr);
        8
    }
    pub(super) fn instr_sre_indy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_indy(bus);
        self.instr_sre(bus, addr);
        8
    }
    pub(super) fn instr_sre_zpg(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpg(bus);
        self.instr_sre(bus, addr);
        5
    }
    pub(super) fn instr_sre_zpgx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_sre(bus, addr);
        6
    }
}
