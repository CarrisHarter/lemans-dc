//! 0x8c0113b4 — reverse byte search.
//!
//! Calls 0x8c0112f4(r4) to get a length, then scans backwards from `r4+len`
//! for the sign-extended byte `r5`, returning its address or 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[3] = mem.read_u32(0x8c0113f0).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]);
    cpu.r[14] = cpu.r[4]; // delay slot of `jsr`
    crate::registry::p_8c0112f4::run(cpu, mem);
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = cpu.r[0];
    let mut pz = (cpu.r[4] as i32) >= 0;
    cpu.set_t(pz);
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[4]);
    cpu.r[5] = cpu.r[5] as i8 as i32 as u32; // delay slot of `bf.s`
    if pz {
        loop {
            cpu.r[2] = mem.read_u8(cpu.r[14]).unwrap_or(0) as u32;
            let eq = cpu.r[5] == cpu.r[2];
            cpu.set_t(eq);
            if eq {
                cpu.r[0] = cpu.r[14];
                cpu.r[15] = cpu.r[15].wrapping_add(4);
                cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
                cpu.r[15] = cpu.r[15].wrapping_add(4);
                cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                cpu.r[15] = cpu.r[15].wrapping_add(4);
                return;
            }
            cpu.r[4] = cpu.r[4].wrapping_sub(1);
            pz = (cpu.r[4] as i32) >= 0;
            cpu.set_t(pz);
            cpu.r[14] = cpu.r[14].wrapping_sub(1); // delay slot of `bt.s`
            if !pz {
                break;
            }
        }
    }
    cpu.r[0] = 0;
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
