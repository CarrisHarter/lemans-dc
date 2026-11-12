//! 0x8c1bc318 - clear a byte flag, run a delay helper, clear a global, then free.
//!
//! When byte `[r13+24]` is non-zero it is cleared and 0x8c0b5e94 runs with
//! `r4 = 0`, `r5 = 0x1388`. Then `[0x8c2b82b8] = 0` lands in the `bf.s` delay
//! slot on both paths, and a positive `exts.w(r14)` frees r13. Returns r13.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    if cpu.r[13] != 0 {
        cpu.r[0] = cpu.r[13].wrapping_add(24); // mov r13,r0 ; add #24,r0
        cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32; // mov.b @r0,r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if !cpu.t() {
            cpu.r[0] = 24; // mov #24,r0
            cpu.r[4] = 0; // mov #0,r4
            let _ = mem.write_u8(cpu.r[13].wrapping_add(cpu.r[0]), cpu.r[4] as u8); // mov.b r4,@(r0,r13)
            cpu.r[5] = mem.read_u32(0x8c1bc360).unwrap_or(0x1388); // mov.l 0x8c1bc360,r5
            cpu.r[0] = mem.read_u32(0x8c1bc364).unwrap_or(0x8c0b5e94); // mov.l 0x8c1bc364,r0
            cpu.pr = 0x8c1bc342; // jsr delay is nop
            crate::registry::p_8c0b5e94::run(cpu, mem);
        }
        cpu.r[3] = mem.read_u32(0x8c1bc368).unwrap_or(0x8c2b82b8); // mov.l 0x8c1bc368,r3
        cpu.r[1] = 0; // mov #0,r1
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // bf.s delay: mov.l r1,@r3
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c1bc36c).unwrap_or(0x8c01032c); // mov.l 0x8c1bc36c,r0
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c1bc354;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13]; // mov r13,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
