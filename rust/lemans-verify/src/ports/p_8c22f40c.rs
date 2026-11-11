//! 0x8c22f40c — state_gate.
//!
//! Returns 0 when `[r4+4]` has bit 2 clear and the global at `0x8c2c6628` is
//! zero, or when `[r4+12] == 2` and bit 32 is clear; otherwise `0xeeee`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    let t = (cpu.r[0] & 2) == 0;
    cpu.set_t(t);
    if t {
        cpu.r[3] = mem.read_u32(0x8c22f448).unwrap_or(0);
        cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        let t2 = cpu.r[2] == 0;
        cpu.set_t(t2);
        if t2 {
            cpu.r[0] = 0; // delay slot of `rts`
            return;
        }
    }
    cpu.r[0] = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
    let eq = cpu.r[0] == 2;
    cpu.set_t(eq);
    if eq {
        cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
        let t3 = (cpu.r[0] & 32) == 0;
        cpu.set_t(t3);
        if t3 {
            cpu.r[0] = 0; // delay slot of `rts`
            return;
        }
    }
    cpu.r[0] = mem.read_u32(0x8c22f44c).unwrap_or(0);
}
