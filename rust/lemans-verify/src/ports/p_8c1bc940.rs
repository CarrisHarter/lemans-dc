//! 0x8c1bc940 - release three sub-objects, zero their slots, then free.
//!
//! `[r13+8]` goes to 0x8c06c8e0 with `r5 = 1` and is cleared; `[r13+4]` goes to
//! 0x8c06cbc0 with `r5 = 1` and is cleared; `[r13]` goes to 0x8c1514f8. The
//! `bf.s` delay slot clears `[r13]` on both paths, and a positive `exts.w(r14)`
//! then frees r13. Returns r13.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    if cpu.r[13] != 0 {
        cpu.r[0] = mem.read_u32(0x8c1bc988).unwrap_or(0x8c06c8e0); // mov.l 0x8c1bc988,r0
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0); // jsr delay: mov.l @(8,r13),r4
        cpu.pr = 0x8c1bc954;
        crate::registry::p_8c06c8e0::run(cpu, mem);
        cpu.r[0] = 0; // mov #0,r0
        let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[0]); // mov.l r0,@(8,r13)
        cpu.r[5] = 1; // mov #1,r5
        cpu.r[0] = mem.read_u32(0x8c1bc98c).unwrap_or(0x8c06cbc0); // mov.l 0x8c1bc98c,r0
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0); // jsr delay: mov.l @(4,r13),r4
        cpu.pr = 0x8c1bc960;
        crate::registry::p_8c06cbc0::run(cpu, mem);
        cpu.r[0] = 0; // mov #0,r0
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r13)
        cpu.r[5] = 1; // mov #1,r5
        cpu.r[0] = mem.read_u32(0x8c1bc990).unwrap_or(0x8c1514f8); // mov.l 0x8c1bc990,r0
        cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0); // jsr delay: mov.l @r13,r4
        cpu.pr = 0x8c1bc96c;
        crate::registry::p_8c1514f8::run(cpu, mem);
        cpu.r[1] = 0; // mov #0,r1
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        let _ = mem.write_u32(cpu.r[13], cpu.r[1]); // bf.s delay: mov.l r1,@r13
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c1bc994).unwrap_or(0x8c01032c); // mov.l 0x8c1bc994,r0
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c1bc97c;
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
