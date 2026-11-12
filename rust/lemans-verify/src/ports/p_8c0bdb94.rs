//! 0x8c0bdb94 - stamp a vtable word, run the 0x8c14b070 teardown, free when positive.
//!
//! `[r13] = 0x8c1eb328` lands in the `jsr` delay slot and 0x8c14b070 runs with
//! `r5 = 0` (so it never frees the object itself). A positive `exts.w(r14)` then
//! frees r13. Returns r13.

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
        cpu.r[1] = mem.read_u32(0x8c0bdbc4).unwrap_or(0x8c1eb328); // mov.l 0x8c0bdbc4,r1
        cpu.r[5] = 0; // mov #0,r5
        cpu.r[0] = mem.read_u32(0x8c0bdbc8).unwrap_or(0x8c14b070); // mov.l 0x8c0bdbc8,r0
        let _ = mem.write_u32(cpu.r[13], cpu.r[1]); // jsr delay: mov.l r1,@r13
        cpu.pr = 0x8c0bdbac;
        crate::registry::p_8c14b070::run(cpu, mem);
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c0bdbcc).unwrap_or(0x8c01032c); // mov.l 0x8c0bdbcc,r0
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c0bdbb8;
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
