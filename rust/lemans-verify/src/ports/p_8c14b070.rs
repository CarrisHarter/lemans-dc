//! 0x8c14b070 - teardown: stamp a vtable word, drain four sub-objects, free.
//!
//! Each sub-object at `+68`, `+40`, `+24`, `+4` is released by its own routine
//! when its address is non-zero (`tst` on `r13+off`, which is only zero for an
//! absurd `r13`). The first release sits behind a `bt.s`, so its delay-slot
//! vtable store `[r13] = 0x8c1ef28c` runs on both paths. The trailing
//! `mov r13,r0 ; add #off,r0 ; tst r0,r0` after each call only sets T, which the
//! next block overwrites. Finally `exts.w(r14) > 0` frees the object itself.
//! Returns r13.

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
        cpu.r[0] = mem.read_u32(0x8c14b0e8).unwrap_or(0x8c1ef28c); // mov.l 0x8c14b0e8,r0
        cpu.r[4] = cpu.r[4].wrapping_add(68); // add #68,r4
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        let _ = mem.write_u32(cpu.r[13], cpu.r[0]); // bt.s delay: mov.l r0,@r13
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c14b0ec).unwrap_or(0x8c14d1e4); // mov.l 0x8c14b0ec,r0
            cpu.pr = 0x8c14b08e;
            crate::registry::p_8c14d1e4::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[13].wrapping_add(68); // mov r13,r0 ; add #68,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0 (dead: overwritten below)

        cpu.r[4] = cpu.r[13].wrapping_add(40); // mov r13,r4 ; add #40,r4
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        if cpu.r[4] != 0 {
            cpu.r[0] = mem.read_u32(0x8c14b0f0).unwrap_or(0x8c14cdcc); // mov.l 0x8c14b0f0,r0
            cpu.pr = 0x8c14b0a2;
            crate::registry::p_8c14cdcc::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[13].wrapping_add(40); // mov r13,r0 ; add #40,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0 (dead)

        cpu.r[4] = cpu.r[13].wrapping_add(24); // mov r13,r4 ; add #24,r4
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        if cpu.r[4] != 0 {
            cpu.r[0] = mem.read_u32(0x8c14b0f4).unwrap_or(0x8c14b664); // mov.l 0x8c14b0f4,r0
            cpu.pr = 0x8c14b0b6;
            crate::registry::p_8c14b664::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[13].wrapping_add(24); // mov r13,r0 ; add #24,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0 (dead)

        cpu.r[4] = cpu.r[13].wrapping_add(4); // mov r13,r4 ; add #4,r4
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        if cpu.r[4] != 0 {
            cpu.r[0] = mem.read_u32(0x8c14b0f8).unwrap_or(0x8c14b2c0); // mov.l 0x8c14b0f8,r0
            cpu.pr = 0x8c14b0ca;
            crate::registry::p_8c14b2c0::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[13].wrapping_add(4); // mov r13,r0 ; add #4,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0 (dead)

        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c14b0fc).unwrap_or(0x8c01032c); // mov.l 0x8c14b0fc,r0
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c14b0dc;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13]; // mov r13,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
