//! 0x8c221392 — register a slot descriptor.
//!
//! Rejects with -1 when `0x8c221da8` reports the slot occupied. Otherwise
//! stores key 7 in the slot (`0x8c221db8`), fills `+20..+40` from `r4`/`r5` and
//! zero stack args (`0x8c221dc8`), then installs the 12-byte names from `r5`
//! into both name fields (`0x8c221e18`). Returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[3] = mem.read_u32(0x8c2215e8).unwrap_or(0); // 0x8c221da8
    cpu.r[12] = cpu.r[5]; // delay slot of `jsr`
    crate::registry::p_8c221da8::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c2215ec).unwrap_or(0); // 0x8c221db8
        cpu.r[5] = 7;
        cpu.r[4] = cpu.r[14]; // delay slot of `jsr`
        crate::registry::p_8c221db8::run(cpu, mem);
        cpu.r[13] = 0;
        cpu.r[2] = mem.read_u32(0x8c2215f0).unwrap_or(0); // 0x8c221dc8
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
        cpu.r[5] = cpu.r[14];
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
        cpu.r[6] = cpu.r[12];
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
        cpu.r[7] = cpu.r[13];
        cpu.r[4] = cpu.r[14]; // delay slot of `jsr`
        crate::registry::p_8c221dc8::run(cpu, mem);
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        cpu.r[3] = mem.read_u32(0x8c2215f4).unwrap_or(0); // 0x8c221e18
        cpu.r[5] = cpu.r[12];
        cpu.r[6] = cpu.r[12];
        cpu.r[4] = cpu.r[14]; // delay slot of `jsr`
        crate::registry::p_8c221e18::run(cpu, mem);
        cpu.r[0] = cpu.r[13];
    } else {
        cpu.r[0] = 0xffff_ffff;
    }
    // Epilogue: lds.l @r15+,pr; mov.l @r15+,r12; mov.l @r15+,r13;
    // rts; mov.l @r15+,r14.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
