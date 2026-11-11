//! 0x8c227fe2 — scan slots for a free one.
//!
//! Walks 0x1c4-byte slots from 0 to 0xe20. Returns the base table plus the
//! slot offset for the first slot whose state word is 1 and whose gate word is
//! zero; otherwise 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = mem.read_u16(0x8c228020).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[4] = 0;
    cpu.r[6] = mem.read_u16(0x8c228022).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[5] = cpu.r[4];
    let mut result: u32 = 0;
    loop {
        cpu.r[0] = mem.read_u32(0x8c2c6fa0).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
        let eq = cpu.r[0] == 1;
        cpu.set_t(eq);
        if eq {
            cpu.r[0] = mem.read_u32(0x8c228028).unwrap_or(0);
            cpu.r[2] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
            let t = cpu.r[2] == 0;
            cpu.set_t(t);
            if t {
                cpu.r[0] = mem.read_u32(0x8c2c6fa0).unwrap_or(0);
                result = cpu.r[0].wrapping_add(cpu.r[5]); // delay slot of `rts`
                break;
            }
        }
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[6]);
        let hs = cpu.r[4] >= cpu.r[7];
        cpu.set_t(hs);
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[6]); // delay slot of `bf.s`
        if hs {
            result = 0;
            break;
        }
    }
    cpu.r[0] = result;
}
