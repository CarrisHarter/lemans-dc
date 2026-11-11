//! 0x8c0100e4 — release a block and maybe its owner.
//!
//! If `r4` is non-null: frees `[r4]` when non-null and zeroes `[r4]`; then if
//! the sign-extended low half of `r5` is positive, frees `r4` itself.
//! Returns `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    let t = cpu.r[13] == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        let t2 = cpu.r[4] == 0;
        cpu.set_t(t2);
        if !t2 {
            cpu.r[0] = mem.read_u32(0x8c010118).unwrap_or(0);
            crate::registry::p_8c159fc0::run(cpu, mem);
            cpu.r[0] = 0;
            let _ = mem.write_u32(cpu.r[13], cpu.r[0]);
        }
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        let pl = (cpu.r[0] as i32) > 0;
        cpu.set_t(pl);
        if pl {
            cpu.r[0] = mem.read_u32(0x8c010118).unwrap_or(0);
            cpu.r[4] = cpu.r[13]; // delay slot of `jsr`
            crate::registry::p_8c159fc0::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
